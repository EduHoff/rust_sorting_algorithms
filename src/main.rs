use std::{io::Write, mem, path::Path};

use clearscreen::clear;
use rust_sorting_algorithms::{
    aux::{
        generate_data, read_array::read_array, read_input, sort_result::SortResult, write_array,
    },
    sort::{bubble_sort, insertion_sort, merge_sort, quick_sort, selection_sort, shell_sort},
};

const PATH_IN: &str = "in.csv";
const PATH_OUT: &str = "out.csv";

fn main() {
    let mut min: usize;
    let mut max: usize;

    clear().expect("Error: clear failed");
    if !Path::new(PATH_IN).exists() {
        eprintln!("File in.csv do not exists!\n");
        loop {
            println!("Enter the minimum value:");
            print!("||");
            std::io::stdout().flush().unwrap();

            match read_input::read_input() {
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

        clear().expect("Error: clear failed");
        loop {
            println!("Enter the maximum value:");
            print!("||");
            std::io::stdout().flush().unwrap();

            match read_input::read_input() {
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

        clear().expect("Error: clear failed");
        generate_data::generate_data(min, max, PATH_IN);
    }

    let array: Vec<u32> = read_array(PATH_IN);
    let result: SortResult<u32>;

    clear().expect("Error: clear failed");
    loop {
        print!(
            "
============================================
            Sorting Algorithms
============================================
 1 - ???
 2 - Bubble Sort
 3 - Selection Sort
 4 - Insertion Sort
 5 - Shell Sort
 6 - ???
 7 - Merge Sort
 8 - Quick Sort
 9 - ???
10 - ???
 0 - Exit
===========================================
"
        );
        print!("||");
        std::io::stdout().flush().unwrap();

        match read_input::read_input() {
            Ok(choose) => {
                result = match choose {
                    //Bogo Sort
                    2 => {
                        clear().expect("Error: clear failed");
                        bubble_sort::sort(array)
                    }
                    3 => {
                        clear().expect("Error: clear failed");
                        selection_sort::sort(array)
                    }
                    4 => {
                        clear().expect("Error: clear failed");
                        insertion_sort::sort(array)
                    }
                    5 => {
                        clear().expect("Error: clear failed");
                        shell_sort::sort(array)
                    }
                    //heap Sort
                    7 => {
                        clear().expect("Error: clear failed");
                        merge_sort::sort(array)
                    }
                    8 => {
                        clear().expect("Error: clear failed");
                        quick_sort::sort(array)
                    }
                    //Time Sort
                    //Radix Sort
                    0 => return,
                    _ => {
                        clear().expect("Error: clear failed");
                        println!("Invalid option!");
                        continue;
                    }
                };

                break;
            }
            Err(e) => {
                clear().expect("Error: clear failed");
                eprintln!("{}", e);
            }
        };
    }

    clear().expect("Error: clear failed");
    result.print_stats();
    write_array::write_array(&result.array, PATH_OUT);

    if let Err(e) = result.write_result() {
        eprintln!("Error writing log: {}", e);
    }

    if let Err(e) = result.write_history() {
        eprintln!("Error writing log: {}", e);
    }

    #[cfg(windows)]
    let _ = std::process::Command::new("cmd")
        .arg("/c")
        .arg("pause")
        .status();
    //cargo cross build --release --target x86_64-pc-windows-gnu
}
