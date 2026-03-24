use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};

use crate::aux::sort_result::SortResult;

pub fn sort<T: PartialOrd + Copy>(mut array: Vec<T>) -> SortResult<T> {

    let pb = ProgressBar::new(array.len() as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let mut comparisons: u64 = 0;
    let swaps: u64 = 0;
    let mut shifts: u64 = 0;
    let mut insertions: u64 = 0;

    let start = Instant::now();
    
    for i in 0..array.len(){

        let key = array[i];
        let mut key_index = i;
        let mut shifted = false;

        for j in (0..i).rev(){ 
            comparisons += 1;

            if array[j] <= key{
                break;
            }

            array[j+1] = array[j];
            key_index = j;

            shifted = true;
            shifts += 1;
        }

        if shifted{
            array[key_index] = key;
            insertions += 1;
        }

        pb.set_position(i as u64);
    }

    let duration = start.elapsed().as_nanos();
    pb.finish_with_message("Sorting completed!");

    SortResult {
        array,
        algorithm: String::from("Insertion Sort"),
        comparisons,
        swaps,
        shifts,
        insertions,
        duration,
    }
}