pub struct SortResult<T> {
    pub array: Vec<T>,
    pub algorithm: String,
    pub comparisons: u64,
    pub swaps: u64,
    pub duration: u128,
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
        println!("Comparisons: {}", self.comparisons);
        println!("Swaps:       {}", self.swaps);
        println!("Duration:    {}", self.format_duration());
        println!("------------------");
    }
}