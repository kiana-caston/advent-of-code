use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let br = BufReader::new(file);

    let mut v: Vec<i32> = Vec::new();

    for line in br.lines() {
        v.push(line.unwrap().parse::<i32>().unwrap());
    }

    let mut index = 0;
    let mut v_sums: Vec<i32> = Vec::new();

    while index < v.len() - 2 {
        let sum = v[index] + v[index + 1] + v[index + 2];
        v_sums.push(sum);

        index = index + 1;
    }

    let mut count = 0;
    index = 0;

    while index < v_sums.len() - 1 {
        if v_sums[index] < v_sums[index + 1] {
            count = count + 1;
        }

        index = index + 1;
    }

    println!("answer: {}", count);

    Ok(())
}
