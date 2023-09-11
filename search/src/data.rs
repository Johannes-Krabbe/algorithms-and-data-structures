use std::{
    fs::File,
    io::{prelude::*, BufReader},
    time::Instant,
};

pub fn get_strings() -> Vec<String> {
    let start = Instant::now();

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

/*
    use std::io::{self, Write},
pub fn gen_strings() -> io::Result<()> {
    let chars: Vec<char> = (b'a'..=b'z').map(|c| c as char).collect();
    let mut file = File::create("strings.txt")?;

    for &a in &chars {
        for &b in &chars {
            for &c in &chars {
                for &d in &chars {
                    println!("{}{}{}{}", a, b, c, d);
                    writeln!(file, "{}{}{}{}", a, b, c, d)?;
                }
            }
        }
    }

    Ok(())
}
*/
