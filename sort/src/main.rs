use data::get_strings;
use std::cmp::Ordering;

mod data;

fn bubble_sort(list: &Vec<String>) -> Vec<String> {
    let mut sorted = list.clone();

    let mut swapped = true;
    while swapped == true {
        swapped = false;
        let mut i = 0;
        while i < sorted.len() - 1 {
            let order = sorted[i].cmp(&sorted[i + 1]);
            if order == Ordering::Greater {
                sorted.swap(i, i + 1);
                swapped = true;
            };
            i += 1;
        }
    }
    sorted
}

fn main() {
    let strings = get_strings();
    let sorted = bubble_sort(&strings);

    let mut i = 0;
    while i < sorted.len() {
        println!("{}", sorted[i]);
        i += 1;
    }

    println!("{:?}", sorted[0]);
}
