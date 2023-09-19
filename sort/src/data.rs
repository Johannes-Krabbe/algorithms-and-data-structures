use rand::seq::SliceRandom;
use std::path::Path;
use std::{
    fs::File,
    io::{self, Write},
    io::{prelude::*, BufReader},
    time::Instant,
};

fn gen_strings() -> io::Result<()> {
    let chars: Vec<char> = (b'a'..=b'z').map(|c| c as char).collect();
    let mut file = File::create("strings.txt")?;

    let mut strings = Vec::new();

    for &a in &chars {
        for &b in &chars {
            strings.push(format!("{}{}", a, b));
        }
    }

    let mut rng = rand::thread_rng();
    strings.shuffle(&mut rng);

    for s in &strings {
        println!("{}", s);
        writeln!(file, "{}", s)?;
    }

    Ok(())
}

pub fn get_strings() -> Vec<String> {
    let start = Instant::now();

    if Path::new("./strings.txt").is_file() == false {
        let _ = gen_strings();
    }

    let file = File::open("./strings.txt").expect("no such file");
    let buf = BufReader::new(file);
    let result = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let duration = start.elapsed();
    println!("Read strings from file in: {:?}", duration);

    result
}
