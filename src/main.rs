use clearscreen::clear;
use rust_sorting_algorithms::sort::bubble_sort;


fn main() {
    clear().expect("clearscreen should be able to clear screen");

    
    let array_teste: Vec<i32> = (1..=10).rev().collect();

    println!("Original: {:?}\n", array_teste);

    let result = bubble_sort::sort(array_teste);

    result.print_full();
}