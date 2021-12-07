use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() {
    // part_1();
    part_2();
}

fn part_1() -> io::Result<()> {
    let file = File::open("example_input.txt")?;
    let br = BufReader::new(file);

    let mut fish: Vec<_> = Vec::new();

    for line in br.lines() {
        let l = line.unwrap();

        fish = l.split(",").map(|x| x.parse().unwrap()).collect();
    }

    let mut day = 0;

    while day < 12 {
        let mut i = 0;
        let mut count = 0;

        while i < fish.len() {
            if fish[i] == 0 {
                fish[i] = 6;
                count = count + 1;
            } else {
                fish[i] = fish[i] - 1;
            }
            i = i + 1;
        }

        let mut bbs = vec![8; count];

        fish.append(&mut bbs);

        day = day + 1;
    }

    println!("answer part 1: {}", fish.len());

    Ok(())
}

fn part_2() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let br = BufReader::new(file);

    let mut initial: Vec<usize> = Vec::new();

    for line in br.lines() {
        let l = line.unwrap();

        initial = l.split(",").map(|x| x.parse().unwrap()).collect();
    }

    let mut day = 0;
    let mut fish = vec![0; 9];

    for f in initial {
        fish[f] += 1;
    }

    while day < 256 {

        let first = fish[0];

        fish.rotate_left(1);

        fish[8] = first;
        fish[6] += first;

        day += 1;
    }

    let mut count: u128 = 0;

    for x in fish.clone() {
        count += x;
    }

    println!("answer part 2: {}", count);

    Ok(())
}
