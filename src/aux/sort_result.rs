use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{BufWriter, Write};
use std::path::Path;


use num_format::{Locale, ToFormattedString};
use sysinfo::System;
use wgpu::Instance;

pub struct SortResult<T> {
    pub array: Vec<T>,
    pub algorithm: String,
    pub comparisons: u64,
    pub swaps: u64,
    pub shifts: u64,
    pub insertions: u64,
    pub duration: u128,
}

struct SystemInfo {
    pub os: String,
    pub cpu: String,
    pub ram_gb: u64,
}

fn get_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let os = System::name().unwrap_or("Unknown OS".to_string());
    let os_version = System::os_version().unwrap_or("Unknown Version".to_string());
    let full_os = format!("{} {}", os, os_version);

    let cpu = sys
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or("Unknown CPU".to_string());

    let total_ram = sys.total_memory();
    let ram_gb = total_ram / 1024 / 1024 / 1024;

    SystemInfo {
        os: full_os,
        cpu,
        ram_gb,
    }
}


fn get_gpu_name() -> String {
    let instance = Instance::default();

    let adapter = pollster::block_on(instance.request_adapter(&Default::default()));

    match adapter {
        Some(adapter) => {
            let info = adapter.get_info();
            info.name
        }
        None => "Unknown GPU".to_string(),
    }
}

fn get_ram_type() -> String {
    
    // ================= WINDOWS =================

    

    #[cfg(windows)]
    {
        use std::process::Command;

        let map_smbios_type = |code: &str| -> String {
            match code.trim() {
                "20" => "DDR".to_string(),
                "21" => "DDR2".to_string(),
                "24" => "DDR3".to_string(),
                "26" => "DDR4".to_string(),
                "30" => "LPDDR4".to_string(),
                "34" => "DDR5".to_string(),
                "35" => "LPDDR5".to_string(),
                _ => format!("RAM ({})", code),
            }
        };

        let output = Command::new("powershell")
            .args(["-Command", "Get-CimInstance Win32_PhysicalMemory | Select-Object -ExpandProperty SMBIOSMemoryType"])
            .output();

        if let Ok(out) = output {
            let code = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !code.is_empty() {
                return map_smbios_type(&code);
            }
        }

        let wmic_output = Command::new("cmd")
            .args(["/C", "wmic memorychip get smbiosmemorytype"])
            .output();

        if let Ok(out) = wmic_output {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                let line = line.trim();

                if !line.is_empty() && line.chars().all(|c| c.is_numeric()) {
                    return map_smbios_type(line);
                }
            }
        }

        return "Unknown (Virtual/Generic)".to_string(); 
    }
    

    // ================= LINUX / BSD =================
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        use std::process::Command;

        let output = Command::new("dmidecode")
            .args(["-t", "memory"])
            .output();

        match output {
            Ok(out) => {
                if out.status.success() {
                    let text = String::from_utf8_lossy(&out.stdout);

                    for line in text.lines() {
                        let line = line.trim();

                        if line.starts_with("Type:") && !line.contains("Unknown") && let Some(t) = line.split(':').nth(1) {
                            return t.trim().to_string();
                        }
                    }
       
                    return "Unknown".to_string()
                } else {
                    return "Unknown (requires root)".to_string()
                }
            }
            Err(_) => {
                return "Unknown (dmidecode not found)".to_string()
            }
        }
    }
    
    // ================= MAC =================
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        
        let output = Command::new("system_profiler")
            .arg("SPMemoryDataType")
            .output();

        if let Ok(out) = output && out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);

            for line in text.lines() {
                let line = line.trim();

                if line.starts_with("Type:") && let Some(t) = line.split(':').nth(1) {
                    let ram_type = t.trim();
                    if !ram_type.is_empty() && ram_type != "Unknown" {
                        return ram_type.to_string();
                    }
                }
            }
            return "Unknown (Memory Type not found)".to_string();
        }

        return "Unknown (MacOS Profiler Error)".to_string();
    }

    #[allow(unreachable_code)]
    "Unsupported OS".to_string()
}

impl<T: std::fmt::Debug> SortResult<T> {

    fn format_duration(&self) -> String {
        let total_nanos = self.duration;

        let nano_per_ms = 1_000_000;
        let nano_per_sec = 1_000_000_000;
        let nano_per_min = 60 * nano_per_sec;
        let nano_per_hour = 60 * nano_per_min;

        if total_nanos >= nano_per_hour {
            let hours = total_nanos / nano_per_hour;
            let mins = (total_nanos % nano_per_hour) / nano_per_min;
            let secs = (total_nanos % nano_per_min) / nano_per_sec;
            let ms = (total_nanos % nano_per_sec) / nano_per_ms;
            format!("{:02}h:{:02}min:{:02}s,{}ms", hours, mins, secs, ms)
        } else if total_nanos >= nano_per_min {
            let mins = total_nanos / nano_per_min;
            let secs = (total_nanos % nano_per_min) / nano_per_sec;
            let ms = (total_nanos % nano_per_sec) / nano_per_ms;
            format!("{:02}min:{:02}s,{}ms", mins, secs, ms)
        } else if total_nanos >= nano_per_sec {
            let secs = total_nanos / nano_per_sec;
            let ms = (total_nanos % nano_per_sec) / nano_per_ms;
            format!("{}s,{}ms", secs, ms)
        } else if total_nanos >= nano_per_ms {
            let ms = total_nanos / nano_per_ms;
            let us = (total_nanos % nano_per_ms) / 1000;
            format!("{}ms,{}µs", ms, us)
        } else {
            format!("{}ns", total_nanos)
        }
    }


    fn write_stats<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        let sys = get_system_info();
        let gpu = get_gpu_name();
        let ram_type = get_ram_type();

        writeln!(writer, "--- Statistics ---")?;
        writeln!(writer, "Algorithm:   {}", self.algorithm)?;
        writeln!(writer, "Comparisons: {}", self.comparisons.to_formatted_string(&Locale::en))?;
        if self.swaps > 0 {writeln!(writer, "Swaps:       {}", self.swaps.to_formatted_string(&Locale::en))?;}
        if self.shifts > 0 {writeln!(writer, "Shifts:      {}", self.shifts.to_formatted_string(&Locale::en))?;}
        if self.insertions > 0 {writeln!(writer, "Insertions:  {}", self.insertions.to_formatted_string(&Locale::en))?;}
        writeln!(writer, "Duration:    {}", self.format_duration())?;

        writeln!(writer, "\n--- System Info ---")?;
        writeln!(writer, "OS:  {}", sys.os)?;
        writeln!(writer, "CPU: {}", sys.cpu)?;
        writeln!(writer, "GPU: {}", gpu)?;
        writeln!(writer, "RAM: {} GiB {}", sys.ram_gb, ram_type)?;
        writeln!(writer, "------------------")?;

        Ok(())
    }

    pub fn print_full(&self) {
        println!("=== Full Sort Report ===");
        println!("Array: {:?}", self.array);
        self.print_stats();
    }

    pub fn print_stats(&self) {
        let mut stdout = std::io::stdout();

        if let Err(e) = self.write_stats(&mut stdout) {
            eprintln!("Error writing to stdout: {}", e);
        }
    }

    pub fn write_result(&self) -> Result<(), std::io::Error> {
        let dir = Path::new("sort_logs");
        create_dir_all(dir)?;

        let filename = format!("{}.log", self.algorithm.replace(" ", "_"));
        let path = dir.join(filename);

        let file = File::create(&path)?;
        let mut writer = BufWriter::new(file);

        self.write_stats(&mut writer)?;
        writer.flush()?;
        Ok(())
    }

    pub fn write_history(&self) -> Result<(), std::io::Error> {
        let dir = Path::new("sort_logs");
        create_dir_all(dir)?;

        let path = dir.join("sorting_history.log");

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;

        let mut writer = BufWriter::new(file);

        self.write_stats(&mut writer)?;
        writeln!(writer, "\n")?;

        writer.flush()?;
        Ok(())
    }
}