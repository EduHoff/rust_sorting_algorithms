use std::{io::Write, mem, path::Path};

use clearscreen::clear;
use rust_sorting_algorithms::{aux::{generate_data, read_array::read_array, read_input, write_array}, sort::{bubble_sort, selection_sort}};

const PATH_IN: &str = "in.csv";
const PATH_OUT: &str = "out.csv";


fn main() {
    clear().expect("Error: clear failed");

    let mut min;
    let mut max;
    if !Path::new(PATH_IN).exists() {
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
        
        generate_data::generate_data(min, max, PATH_IN);
    }
    


    clear().expect("Error: clear failed");
    let array: Vec<u32> = read_array(PATH_IN);

    //let result = bubble_sort::sort(array);
    let result = selection_sort::sort(array);

    clear().expect("Error: clear failed");
    result.print_stats();
    write_array::write_array(&result.array, PATH_OUT);
}