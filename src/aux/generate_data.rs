use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn generate_data(min: u32, max: u32, path: &str){

    let mut nums: Vec<u32> = (min..=max).collect();
    nums.shuffle(&mut thread_rng());

    let file = match File::create(path){
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create file: {}", e);
            return;
        }
    };
  
    let mut writer = BufWriter::new(file);

    for (i, num) in nums.iter().enumerate(){
        if i > 0 {
            match write!(writer, ","){
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Failed to write comma: {}", e);
                    return;
                }    
            }
        }

        match write!(writer, "{}", num){
            Ok(_) => {},
            Err(e) => {
                eprintln!("Failed to write number: {}", e);
                return;
            }    
        }
    }

    match writer.flush(){
        Ok(_) => {},
        Err(e) => {
            eprintln!("Failed to flush buffer: {}", e);
        }
    }
}