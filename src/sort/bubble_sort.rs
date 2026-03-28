use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};

use crate::aux::sort_result::SortResult;

pub fn sort<T: PartialOrd + Copy>(mut array: Vec<T>) -> SortResult<T> {

    let pb = ProgressBar::new(array.len() as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let mut comparisons: usize = 0;
    let mut swaps: usize = 0;
    let shifts: usize = 0;
    let insertions: usize = 0;
    let moves: usize = 0;

    let start = Instant::now();
    
    for i in 0..array.len(){
        let mut swapped = false;

        for j in 0..(array.len()-(i+1)){
            comparisons += 1;

            if array[j] < array[j+1]{
                array.swap(j, j+1);

                swaps += 1;
                swapped=true;
            }
        }

        pb.set_position(i as u64);
        if !swapped {break};
    }

    let duration: usize = start.elapsed().as_nanos() as usize;
    pb.finish_with_message("Sorting completed!");

    SortResult {
        array,
        algorithm: String::from("Bubble Sort"),
        comparisons,
        swaps,
        shifts,
        insertions,
        moves,
        duration,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_array() {

        let array: Vec<i32> = (1..10).rev().collect();

        let result = sort(array);
        let expected: Vec<i32> = (1..10).collect();

        assert_eq!(
            result.array, 
            expected, 
            "\nAlgorithm: {}\nStatus: FAILED\nExpected: {:?}\nFound: {:?}\nMetrics: {:?}", 
            result.algorithm, expected, result.array, result
        );
    }
}