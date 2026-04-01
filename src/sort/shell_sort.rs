use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};

use crate::aux::sort_result::SortResult;

const INTERVAL: usize = 3;

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
    let swaps: u64 = 0;
    let mut shifts: u64 = 0;
    let mut insertions: u64 = 0;
    let moves: u64 = 0;
    let attempts: u64 = 0;

    let start = Instant::now();

    let mut gap = 1;
    while gap < array.len() {
        gap = gap * INTERVAL + 1;
    }

    while gap >= 1 {
        gap /= INTERVAL;
        if gap == 0 {
            break;
        }

        for i in gap..array.len() {
            let key_array = array[i];
            let mut j = i;

            while j >= gap && array[j - gap] > key_array {
                comparisons += 1;

                array[j] = array[j - gap];

                shifts += 1;
                j -= gap;
            }

            array[j] = key_array;

            insertions += 1;
            pb.set_position(i as u64);
        }
    }

    let duration: u128 = start.elapsed().as_nanos();
    pb.finish_with_message("Sorting completed!");

    SortResult {
        array,
        algorithm: String::from("Shell Sort"),
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
