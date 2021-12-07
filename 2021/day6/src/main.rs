use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let br = BufReader::new(file);

    let mut fish: Vec<_> = Vec::new();

    for line in br.lines() {
        let l = line.unwrap();

        fish = l.split(",").map(|x| x.parse().unwrap()).collect();
    }

    let mut day = 0;

    while day < 80 {
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

    // println!("fish: {:?}", fish);
    println!("answer: {}", fish.len());

    Ok(())
}
