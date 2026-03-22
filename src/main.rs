use std::{io::Write, mem, path::Path};

use clearscreen::clear;
use rust_sorting_algorithms::{aux::{generate_array::generate_array, generate_data, read_input}, sort::bubble_sort};

const PATH: &str = "in.csv";


fn main() {
    clear().expect("Error: clear failed");

    let mut min: u32 = 0;
    let mut max: u32 = 0;
    if !Path::new(PATH).exists() {
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
        
        generate_data::generate_data(min, max, PATH);
    }
    


    clear().expect("Error: clear failed");
    let array: Vec<u32> = generate_array(PATH);

    println!("Original: {:?}\n", array);

    println!("min: {}, max: {};", min, max);

    let result = bubble_sort::sort(array);

    result.print_full();

     
}