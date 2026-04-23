use std::{
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::Instant,
};

use indicatif::{ProgressBar, ProgressStyle};

use crate::aux::sort_result::SortResult;

const THREADS_LIMIT: usize = 100_000;

fn merge<T: PartialOrd + Copy + Send + Sync>(
    array: &mut [T],
    buffer: &mut [T],
    index_middle: usize,
    atomic_comparisons: &AtomicU64,
    atomic_moves: &AtomicU64,
    pb: &ProgressBar,
) {
    buffer.copy_from_slice(array);

    let (buffer_left, buffer_right) = buffer.split_at(index_middle);

    let left_array_len = buffer_left.len();
    let right_array_len = buffer_right.len();

    let mut left_i: usize = 0;
    let mut right_i: usize = 0;
    let mut merged_i: usize = 0;

    while left_i < left_array_len && right_i < right_array_len {
        atomic_comparisons.fetch_add(1, Ordering::SeqCst);

        if buffer_left[left_i] <= buffer_right[right_i] {
            array[merged_i] = buffer_left[left_i];
            left_i += 1;
        } else {
            array[merged_i] = buffer_right[right_i];
            right_i += 1;
        }

        atomic_moves.fetch_add(1, Ordering::SeqCst);

        merged_i += 1;
    }

    while left_i < left_array_len {
        array[merged_i] = buffer_left[left_i];

        atomic_moves.fetch_add(1, Ordering::SeqCst);

        left_i += 1;
        merged_i += 1;
    }

    while right_i < right_array_len {
        array[merged_i] = buffer_right[right_i];

        atomic_moves.fetch_add(1, Ordering::SeqCst);

        right_i += 1;
        merged_i += 1;
    }

    pb.inc(array.len() as u64);
}

fn merge_sort_parallel<T: PartialOrd + Copy + Send + Sync>(
    array: &mut [T],
    buffer: &mut [T],
    atomic_comparisons: &AtomicU64,
    atomic_moves: &AtomicU64,
    pb: &ProgressBar,
) {
    let size_array = array.len();
    if size_array <= 1 {
        return;
    }
    let middle = size_array / 2;

    let (left_part, right_part) = array.split_at_mut(middle);
    let (left_buffer, right_buffer) = buffer.split_at_mut(middle);

    if size_array > THREADS_LIMIT {
        thread::scope(|s| {
            s.spawn(|| {
                merge_sort_parallel(left_part, left_buffer, atomic_comparisons, atomic_moves, pb);
            });
            s.spawn(|| {
                merge_sort_parallel(
                    right_part,
                    right_buffer,
                    atomic_comparisons,
                    atomic_moves,
                    pb,
                );
            });
        });
    } else {
        merge_sort_parallel(left_part, left_buffer, atomic_comparisons, atomic_moves, pb);
        merge_sort_parallel(
            right_part,
            right_buffer,
            atomic_comparisons,
            atomic_moves,
            pb,
        );
    }

    merge(array, buffer, middle, atomic_comparisons, atomic_moves, pb);
}

pub fn sort<T: PartialOrd + Copy + Send + Sync>(mut array: Vec<T>) -> SortResult<T> {
    let pb = ProgressBar::new(array.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {human_pos}/{human_len} ({eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    //comparisons: u64;
    let swaps: u64 = 0;
    let shifts: u64 = 0;
    let insertions: u64 = 0;
    //moves: u64;
    let attempts: u64 = 0;

    let atomic_comparisons: AtomicU64 = AtomicU64::new(0);
    let atomic_moves: AtomicU64 = AtomicU64::new(0);

    let start = Instant::now();

    let mut buffer = array.clone();

    merge_sort_parallel(
        &mut array,
        &mut buffer,
        &atomic_comparisons,
        &atomic_moves,
        &pb,
    );

    let duration: u128 = start.elapsed().as_nanos();
    pb.finish_with_message("Sorting completed!");

    let comparisons = atomic_comparisons.into_inner();
    let moves = atomic_moves.into_inner();

    SortResult {
        array,
        algorithm: String::from("Merge Sort"),
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
