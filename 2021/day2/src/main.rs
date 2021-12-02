use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let br = BufReader::new(file);

    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in br.lines() {
        let l = line.unwrap();
        let words: Vec<_> = l.split_whitespace().collect();

        let direction = words[0];
        let distance = words[1].parse::<i32>().unwrap();

        match direction {
            "forward" => {
                pos = pos + distance;
                depth = depth + aim * distance;
            },
            "down" => aim = aim + distance,
            "up" => aim = aim - distance,
            _ => println!("invalid key"),
        }
    }

    let ans = pos * depth;

    println!("answer: {}", ans);

    Ok(())
}
