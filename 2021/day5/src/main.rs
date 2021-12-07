use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let br = BufReader::new(file);

    let mut map: Vec<_> = vec![vec![0; 1000]; 1000];

    for line in br.lines() {
        let l = line.unwrap();

        let coords: Vec<_>  = l.split(" -> ").map(String::from).collect();

        let p1: Vec<_> = coords[0].split(',').map(String::from).collect();
        let p2: Vec<_> = coords[1].split(',').map(String::from).collect();

        let x1 = p1[0].parse::<usize>().unwrap();
        let y1 = p1[1].parse::<usize>().unwrap();

        let x2 = p2[0].parse::<usize>().unwrap();
        let y2 = p2[1].parse::<usize>().unwrap();

        let mut i = x1;
        let mut j = y1;

        while true {
            map[j][i] = map[j][i] + 1;

            if i == x2 && j == y2 {
                break;
            }

            if x1 < x2 {
                i = i + 1;
            } else if x1 > x2 {
                i = i - 1;
            }

            if y1 < y2 {
                j = j + 1
            } else if y1 > y2 {
                j = j - 1;
            }
        }
    }

    let mut count = 0;

    for row in map.clone() {
        for num in row {
            if num >= 2 {
                count = count + 1;
            }
        }
    }

    println!("answer: {}", count);

    Ok(())
}
