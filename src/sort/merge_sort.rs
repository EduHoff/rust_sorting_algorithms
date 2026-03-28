use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};

use crate::aux::sort_result::SortResult;


fn merge <T: PartialOrd + Copy>(array: &mut [T], index_left: usize, index_right: usize, index_middle: usize, comparisons:  &mut usize, moves:  &mut usize, pb: &ProgressBar){

    let left_array_len: usize = index_middle - index_left + 1;
    let right_array_len: usize = index_right - index_middle;

    let mut array_left: Vec<T> = Vec::with_capacity(left_array_len);
    let mut array_right: Vec<T> = Vec::with_capacity(right_array_len);

    for left_i in 0..left_array_len{
        array_left.push(array[index_left + left_i]);
    }

    for right_i in 0..right_array_len{
        array_right.push(array[index_middle + 1 + right_i]);
    }


    let mut left_i: usize = 0;
    let mut right_i: usize = 0;
    let mut merged_i: usize = index_left;

    while left_i < left_array_len && right_i < right_array_len {

        *comparisons += 1;

        if array_left[left_i] <= array_right[right_i]{
            array[merged_i] = array_left[left_i];
            left_i += 1;
        }else {
            array[merged_i] = array_right[right_i];
            right_i += 1;
        }

        *moves += 1;
        pb.set_position(merged_i as u64);

        merged_i += 1;
    }

    

    while left_i < left_array_len {
        array[merged_i] = array_left[left_i];

        *moves += 1;
        pb.set_position(merged_i as u64);

        left_i += 1;
        merged_i += 1;
    }
    
    while right_i < right_array_len {
        array[merged_i] = array_right[right_i];

        *moves += 1;
        pb.set_position(merged_i as u64);

        right_i += 1;
        merged_i += 1;
    }
}

fn merge_sort_recursive <T: PartialOrd + Copy>(array: &mut Vec<T>, index_left: usize, index_right: usize, comparisons:  &mut usize, moves:  &mut usize, pb: &ProgressBar){

    if index_left < index_right {

        let index_middle: usize = index_left + (index_right - index_left)/2;
        merge_sort_recursive(array, index_left, index_middle, comparisons, moves, pb);
        merge_sort_recursive(array, index_middle+1, index_right, comparisons, moves, pb);

        merge(array, index_left, index_right, index_middle, comparisons, moves, pb);
    }
}

pub fn sort<T: PartialOrd + Copy>(mut array: Vec<T>) -> SortResult<T> {

    let pb = ProgressBar::new(array.len() as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let mut comparisons: usize = 0;
    let swaps: usize = 0;
    let shifts: usize = 0;
    let insertions: usize = 0;
    let mut moves: usize = 0;

    let start = Instant::now();

    let index_left: usize = 0;
    let index_right: usize = array.len()-1;
    merge_sort_recursive(&mut array, index_left, index_right, &mut comparisons, &mut moves, &pb);

    let duration: usize = start.elapsed().as_nanos() as usize;
    pb.finish_with_message("Sorting completed!");

    SortResult {
        array,
        algorithm: String::from("Merge Sort"),
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