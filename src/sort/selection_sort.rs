use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};

use crate::aux::sort_result::SortResult;

pub fn sort<T: PartialOrd + Copy>(mut array: Vec<T>) -> SortResult<T> {

    let pb = ProgressBar::new(array.len() as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let mut comparisons: u64 = 0;
    let mut swaps: u64 = 0;

    let start = Instant::now();

    for i in 0..array.len(){
        let mut min_index = i;

        for j in (i+1)..array.len(){
            comparisons += 1;

            if array[j] < array[min_index]{
                min_index = j;
            }
        }

        if min_index != i {
            let aux = array[min_index];
            array[min_index] = array[i];
            array[i] = aux;
            
            swaps += 1;
        }
        
        pb.set_position(i as u64);
    }

    let duration = start.elapsed().as_nanos() as u128;
    pb.finish_with_message("Sorting completed!");

    SortResult {
        array,
        algorithm: String::from("Selection Sort"),
        comparisons: comparisons,
        swaps,
        duration,
    }
}