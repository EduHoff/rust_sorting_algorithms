use std::fs::File;
use std::io::{BufWriter, Write};

pub fn write_array<T: std::fmt::Display> (array: &Vec<T>, path: &str){

    let file = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create file: {}", e);
            return;
        }
    };

    let mut writer = BufWriter::new(file);

    for (i, num) in array.iter().enumerate() {
        if i > 0 {
            if let Err(e) = write!(writer, ",") {
                eprintln!("Failed to write comma: {}", e);
                return;
            }
        }

        if let Err(e) = write!(writer, "{}", num) {
            eprintln!("Failed to write number: {}", e);
            return;
        }
    }

    if let Err(e) = writer.flush() {
        eprintln!("Failed to flush buffer: {}", e);
    }

}