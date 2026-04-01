use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};

use crate::aux::sort_result::SortResult;

fn heapify_down<T: PartialOrd + Copy>(
    array: &mut [T],
    limit_tree: usize,
    root: usize,
    comparisons: &mut u64,
    swaps: &mut u64,
) {
    let mut largest = root;
    let left_child = 2 * root + 1;
    let right_child = 2 * root + 2;

    if left_child < limit_tree {
        *comparisons += 1;

        if array[left_child] > array[largest] {
            largest = left_child;
        }
    }

    if right_child < limit_tree {
        *comparisons += 1;

        if array[right_child] > array[largest] {
            largest = right_child;
        }
    }

    if largest != root {
        array.swap(root, largest);

        *swaps += 1;

        heapify_down(array, limit_tree, largest, comparisons, swaps);
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

    let limit_tree = array.len();

    for i in (0..limit_tree / 2).rev() {
        heapify_down(&mut array, limit_tree, i, &mut comparisons, &mut swaps);
    }

    for i in (1..limit_tree).rev() {
        array.swap(0, i);

        swaps += 1;
        pb.set_position((limit_tree - i) as u64);

        heapify_down(&mut array, i, 0, &mut comparisons, &mut swaps);
    }

    let duration: u128 = start.elapsed().as_nanos();
    pb.finish_with_message("Sorting completed!");

    SortResult {
        array,
        algorithm: String::from("Heap Sort"),
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
