use data::get_strings;
use std::cmp::Ordering;
use std::time::Instant;

mod data;

fn bubble_sort(list: &Vec<String>) -> Vec<String> {
    let mut sorted = list.clone();

    let mut swapped = true;
    let mut runs = 0;
    while swapped == true {
        swapped = false;
        let mut i = 0;
        while i < sorted.len() - 1 - runs {
            let order = sorted[i].cmp(&sorted[i + 1]);
            if order == Ordering::Greater {
                sorted.swap(i, i + 1);
                swapped = true;
            };
            i += 1;
        }
        runs += 1;
    }
    sorted
}

fn main() {
    let strings = get_strings();
    let start = Instant::now();
    let sorted = bubble_sort(&strings);
    let duration = start.elapsed();

    let mut i = 0;
    while i < sorted.len() {
        println!("{}", sorted[i]);
        i += 1;
    }

    println!("Time elapsed in expensive_function() is: {:?}", duration);

    println!("{:?}", sorted[0]);
}
