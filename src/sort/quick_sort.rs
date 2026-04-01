use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};

use crate::aux::sort_result::SortResult;

fn quick_sort<T: PartialOrd + Copy>(
    array: &mut Vec<T>,
    index_left: usize,
    index_right: usize,
    comparisons: &mut u64,
    swaps: &mut u64,
    pb: &ProgressBar,
) {
    let mut left_i: usize = index_left;
    let mut right_i: usize = index_right;
    let index_middle: usize = (left_i + right_i) / 2;
    let pivot = array[index_middle];

    while right_i > left_i {
        while array[left_i] < pivot {
            *comparisons += 1;
            left_i += 1;
        }
        *comparisons += 1;

        while right_i > 0 && array[right_i] > pivot {
            *comparisons += 1;
            right_i -= 1;
        }
        *comparisons += 1;

        if left_i <= right_i {
            array.swap(left_i, right_i);

            *swaps += 1;
            pb.set_position(left_i as u64);

            left_i += 1;
            right_i = right_i.saturating_sub(1);
        }
    }

    if index_left < left_i {
        quick_sort(array, index_left, right_i, comparisons, swaps, pb);
    }

    if index_right > right_i {
        quick_sort(array, left_i, index_right, comparisons, swaps, pb);
    }
}

pub fn sort<T: PartialOrd + Copy>(mut array: Vec<T>) -> SortResult<T> {
    let pb = ProgressBar::new(array.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {human_pos}/{human_len} ({eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    let mut comparisons: u64 = 0;
    let mut swaps: u64 = 0;
    let shifts: u64 = 0;
    let insertions: u64 = 0;
    let moves: u64 = 0;
    let attempts: u64 = 0;

    let start = Instant::now();

    let index_left: usize = 0;
    let index_right: usize = array.len() - 1;

    quick_sort(
        &mut array,
        index_left,
        index_right,
        &mut comparisons,
        &mut swaps,
        &pb,
    );

    let duration: u128 = start.elapsed().as_nanos();
    pb.finish_with_message("Sorting completed!");

    SortResult {
        array,
        algorithm: String::from("Quick Sort"),
        comparisons,
        swaps,
        shifts,
        insertions,
        moves,
        attempts,
        duration,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_array() {
        let array: Vec<u64> = (1..=10).rev().collect();

        let result = sort(array);
        let expected: Vec<u64> = (1..=10).collect();

        assert_eq!(
            result.array, expected,
            "\nAlgorithm: {}\nStatus: FAILED\nExpected: {:?}\nFound: {:?}\nMetrics: {:?}",
            result.algorithm, expected, result.array, result
        );
    }
}
