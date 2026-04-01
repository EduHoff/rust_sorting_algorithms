use indicatif::{ProgressBar, ProgressStyle};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn generate_data(min: usize, max: usize, path: &str) {
    let pb = ProgressBar::new((max - min + 1) as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {human_pos}/{human_len} ({eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    let mut nums: Vec<usize> = (min..=max).collect();
    nums.shuffle(&mut thread_rng());

    let file = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create file: {}", e);
            return;
        }
    };

    let mut writer = BufWriter::new(file);

    for (i, num) in nums.iter().enumerate() {
        if i > 0 {
            match write!(writer, ",") {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to write comma: {}", e);
                    return;
                }
            }
        }

        match write!(writer, "{}", num) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to write number: {}", e);
                return;
            }
        }

        pb.set_position(i as u64);
    }

    pb.finish_with_message("data generation completed!");

    match writer.flush() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to flush buffer: {}", e);
        }
    }
}
