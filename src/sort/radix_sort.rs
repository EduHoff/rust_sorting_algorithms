use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};

use crate::aux::sort_result::SortResult;

fn get_significant_bytes(array: &[usize]) -> usize {
    let max = *array.iter().max().unwrap_or(&0);

    if max == 0 {
        return 1;
    }

    let bits_used = usize::BITS - max.leading_zeros();

    bits_used.div_ceil(8) as usize
}

fn counting_sort(
    array: &mut [usize],
    aux_array: &mut [usize],
    shift: usize,
    metric_shifts: &mut u64,
    moves: &mut u64,
    pb: &ProgressBar,
) {
    let mut bucket_positions = [0usize; 256];

    for &num in array.iter() {
        let byte = (num >> shift) & 0xFF;
        bucket_positions[byte] += 1;

        *metric_shifts += 1;
    }

    for i in 1..256 {
        bucket_positions[i] += bucket_positions[i - 1];
    }

    for i in (0..array.len()).rev() {
        let byte = (array[i] >> shift) & 0xFF;
        let target_index = bucket_positions[byte] - 1;
        aux_array[target_index] = array[i];
        bucket_positions[byte] -= 1;

        *metric_shifts += 1;
        *moves += 1;

        pb.inc(1);
    }

    array.copy_from_slice(aux_array);
    *moves += array.len() as u64;
}

pub fn sort(mut array: Vec<usize>) -> SortResult<usize> {
    let total_bytes_to_process = get_significant_bytes(&array);

    let pb = ProgressBar::new((array.len() * total_bytes_to_process) as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {human_pos}/{human_len} ({eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    let comparisons: u64 = 0;
    let swaps: u64 = 0;
    let mut metric_shifts: u64 = 0;
    let insertions: u64 = 0;
    let mut moves: u64 = 0;
    let attempts: u64 = 0;

    let start = Instant::now();

    let mut aux_array: Vec<usize> = vec![0; array.len()];

    for i in 0..total_bytes_to_process {
        let shift = i * 8;

        counting_sort(
            &mut array,
            &mut aux_array,
            shift,
            &mut metric_shifts,
            &mut moves,
            &pb,
        );
    }

    let duration: u128 = start.elapsed().as_nanos();
    pb.finish_with_message("Sorting completed!");

    SortResult {
        array,
        algorithm: String::from("Radix Sort"),
        comparisons,
        swaps,
        shifts: metric_shifts,
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
        let array: Vec<usize> = (1..=10).rev().collect();

        let result = sort(array);
        let expected: Vec<usize> = (1..=10).collect();

        assert_eq!(
            result.array, expected,
            "\nAlgorithm: {}\nStatus: FAILED\nExpected: {:?}\nFound: {:?}\nMetrics: {:?}",
            result.algorithm, expected, result.array, result
        );
    }
}
