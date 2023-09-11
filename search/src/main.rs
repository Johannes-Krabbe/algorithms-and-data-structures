use data::get_strings;
use std::cmp::Ordering;
use std::io;
use std::time::Instant;

mod data;

fn simple_search(list: &Vec<String>, item: &str) -> Option<usize> {
    for (pos, e) in list.iter().enumerate() {
        if e == item {
            return Some(pos);
        }
    }
    None
}

fn binary_search(list: &[String], item: &String) -> Option<usize> {
    if list.len() == 0 {
        return None;
    }
    let order = list[list.len() / 2].cmp(item);
    println!("{}", list[list.len() / 2]);

    match order {
        Ordering::Equal => return Some(list.len() / 2),
        Ordering::Less => {
            let index = binary_search(&list[list.len() / 2..], item);
            match index {
                Some(i) => return Some(list.len() / 2 + i),
                None => return None,
            }
        }
        Ordering::Greater => {
            let index = binary_search(&list[..list.len() / 2], item);
            match index {
                Some(i) => return Some(i),
                None => return None,
            }
        }
    }
}

fn main() {
    let strings = get_strings();
    let item = String::from("wxyz");

    let start = Instant::now();
    let index = binary_search(&strings, &item);
    let duration = start.elapsed();

    println!("---binary_search---");
    match index {
        Some(i) => println!("Found {} at index {}. Search took {:?}", item, i, duration),
        None => println!("{} not found. Search took {:?}", item, duration),
    }

    let start = Instant::now();
    let index = simple_search(&strings, &item);
    let duration = start.elapsed();

    println!("---simple_search---");
    match index {
        Some(i) => println!("Found {} at index {}. Search took {:?}", item, i, duration),
        None => println!("{} not found. Search took {:?}", item, duration),
    }
}
