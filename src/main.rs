mod buble_sort;

use crate::buble_sort::buble_sort;

fn main() {
    println!("Hello, world!");

    let mut numbers = vec![4, 2, 5, 3, 1];

    buble_sort(&mut numbers);

    println!("Array ordenado: {:?}", numbers);
}
