use clearscreen::clear;
use rust_sorting_algorithms::sort::*;

fn main() {
    clear().expect("clearscreen should be able to clear screen");

    
    let mut array_teste: Vec<i32> = (1..=10).rev().collect();

    println!("Original: {:?}", array_teste);
    bubble_sort::sort(&mut array_teste);
    println!("Ordenado: {:?}", array_teste);
}