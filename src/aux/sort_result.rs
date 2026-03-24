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

    pub fn print_full(&self) {
        println!("=== Full Sort Report ===");
        println!("Array: {:?}", self.array);
        self.print_stats();
    }

    pub fn print_stats(&self) {
        println!("--- Statistics ---");
        println!("Algorithm:   {}", self.algorithm);
        println!("Comparisons: {}", self.comparisons.to_formatted_string(&Locale::en));
        if self.swaps > 0 {println!("Swaps:       {}", self.swaps.to_formatted_string(&Locale::en))};
        if self.shifts > 0 {println!("Shifts:      {}", self.shifts.to_formatted_string(&Locale::en))};
        if self.insertions > 0 {println!("Insertions:  {}", self.insertions.to_formatted_string(&Locale::en))};
        println!("Duration:    {}", self.format_duration());
        
        let sys = get_system_info();
        let gpu = get_gpu_name(); 
        println!("\n--- System Info ---");
        println!("OS:  {}", sys.os);
        println!("CPU: {}", sys.cpu);
        println!("GPU: {}", gpu);
        println!("RAM: {} GiB", sys.ram_gb);
        println!("------------------");
    }
}