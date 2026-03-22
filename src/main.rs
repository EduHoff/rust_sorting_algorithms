use std::{io::Write, mem, path::Path};

use clearscreen::clear;
use rust_sorting_algorithms::{aux::{generate_data, read_input}, sort::bubble_sort};


fn main() {
    clear().expect("Error: clear failed");

    let mut min: u32 = 0;
    let mut max: u32 = 0;
    if !Path::new("in.csv").exists() {
        eprintln!("File in.csv do not exists!");
        loop {
            println!("Enter the minimum value:");
            print!("||");
            std::io::stdout().flush().unwrap();

            match read_input::read_input(){
                Ok(num) => {
                    min = num;
                    break;
                }
                Err(e) => {
                    clear().expect("Error: clear failed");
                    eprintln!("{}", e);
                    0
                }                
            };
        }

        loop {
            println!("Enter the maximum value:");
            print!("||");
            std::io::stdout().flush().unwrap();

            match read_input::read_input(){
                Ok(num) => {
                    max = num;
                    break;
                }
                Err(e) => {
                    clear().expect("Error: clear failed");
                    eprintln!("{}", e);
                    0
                }                
            };
        }

        if min > max {
            mem::swap(&mut min, &mut max);
        }
        
        generate_data::generate_data(min, max);
    }
    


    clear().expect("Error: clear failed");
    let array_teste: Vec<i32> = (1..=10).rev().collect();

    println!("Original: {:?}\n", array_teste);

    println!("min: {}, max: {};", min, max);

    let result = bubble_sort::sort(array_teste);

    result.print_full();

     
}