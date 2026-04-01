use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};
use num_format::{Locale, ToFormattedString};
use rand::{seq::SliceRandom, thread_rng};

use crate::aux::sort_result::SortResult;

fn is_sorted<T: PartialOrd + Copy>(array: &[T], comparisons: &mut usize) -> bool {
    for i in 0..array.len() - 1 {
        *comparisons += 1;
        if array[i] > array[i + 1] {
            return false;
        }
    }
    true
}

pub fn sort<T: PartialOrd + Copy>(mut array: Vec<T>) -> SortResult<T> {
    let pb = ProgressBar::new(array.len() as u64);

    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] | {msg}",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    let mut comparisons: usize = 0;
    let mut swaps: usize = 0;
    let shifts: usize = 0;
    let insertions: usize = 0;
    let moves: usize = 0;
    let mut attempts: usize = 0;

    let start = Instant::now();

    let mut rng = thread_rng();
    while !is_sorted(&array, &mut comparisons) {
        array.shuffle(&mut rng);

        attempts += 1;
        swaps += array.len().saturating_sub(1);
        pb.set_message(format!("Attempts: {}", attempts.to_formatted_string(&Locale::en)));
    }

    let duration: usize = start.elapsed().as_nanos() as usize;
    pb.finish_with_message("Sorting completed!");

    SortResult {
        array,
        algorithm: String::from("Bogo Sort"),
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
    fn test_is_sorted() {
        let array: Vec<i32> = (1..10).collect();
        let mut comparisons: usize = 0;

        let result = is_sorted(&array, &mut comparisons);
        assert!(result);
    }

    #[test]
    fn sort_array() {
        let array: Vec<i32> = (1..=3).rev().collect();

        let result = sort(array);
        let expected: Vec<i32> = (1..=3).collect();

        assert_eq!(
            result.array, expected,
            "\nAlgorithm: {}\nStatus: FAILED\nExpected: {:?}\nFound: {:?}\nMetrics: {:?}",
            result.algorithm, expected, result.array, result
        );
    }
}
