use std::time::Instant;

use crate::aux::sort_result::SortResult;

pub fn sort<T: PartialOrd + Copy>(mut array: Vec<T>) -> SortResult<T> {

    let mut comparisons: u64 = 0;
    let mut swaps: u64 = 0;

    let start = Instant::now();
    
    for i in 0..array.len(){
        let mut swapped = false;

        for j in 0..(array.len()-(i+1)){
            comparisons += 1;

            if array[j] > array[j+1]{
                let aux = array[j];
                array[j] = array[j+1];
                array[j+1] = aux;

                swaps += 1;
                swapped=true;
            }
        }

        if !swapped {break};
    }

    let duration = start.elapsed().as_nanos() as u128;

    SortResult {
        array: array,
        algorithm: String::from("Bubble Sort"),
        comparisons: comparisons,
        swaps: swaps,
        duration: duration,
    }
}