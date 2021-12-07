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

        if x1 == x2 {
            if y1 < y2 {
                let mut i = y1;

                while i <= y2 {
                    map[i][x1] = map[i][x1] + 1;

                    i = i + 1;
                }
            } else {
                let mut i = y2;

                while i <= y1 {
                    map[i][x1] = map[i][x1] + 1;

                    i = i + 1;
                }
            }
        } else if y1 == y2 {
            if x1 < x2 {
                let mut i = x1;

                while i <= x2 {
                    map[y1][i] = map[y1][i] + 1;

                    i = i + 1;
                }
            } else {
                let mut i = x2;

                while i <= x1 {
                    map[y1][i] = map[y1][i] + 1;

                    i = i + 1;
                }
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

    // println!("map: {:?}", map);
    println!("answer: {}", count);

    Ok(())
}
