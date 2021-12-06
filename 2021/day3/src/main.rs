use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() {
    part_1();
    part_2();
}

fn part_1() -> io::Result<()> {
    let file = File::open("example_input.txt")?;
    let br = BufReader::new(file);

    let mut vec: Vec<_> = Vec::new();
    let mut first = true;

    for line in br.lines() {
        let l = line.unwrap();

        let chars: Vec<char> = l.chars().collect();

        let mut index = 0;

        while index < chars.len() - 0 {
            if first {
                vec.push(chars[index].to_string());
            } else {
                vec[index].push(chars[index]);
            }

            index = index + 1;
        }

        first = false;
    }

    let mut counts: Vec<_> = Vec::new();

    for x in &vec {
        let zero = x.matches('0').count();
        let one = x.matches('1').count();

        counts.push((zero, one));
    }

    let mut gamma_rate = String::from(""); // most common
    let mut epsilon_rate = String::from(""); // least common

    for y in &counts {
        if y.0 > y.1 {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        } else {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        }
    }

    let g = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let e = isize::from_str_radix(&epsilon_rate, 2).unwrap();

    let power_consumption = g * e;

    println!("answer part 1: {}", power_consumption);

    Ok(())
}

fn part_2() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let br = BufReader::new(file);

    let mut report: Vec<String> = Vec::new();

    for line in br.lines() {
        let l = line.unwrap();

        report.push(l);
    }

    let oxygen = &generator(report.clone(), 0, true)[0];
    let carbon = &generator(report, 0, false)[0];

    let o = isize::from_str_radix(&oxygen, 2).unwrap();
    let c = isize::from_str_radix(&carbon, 2).unwrap();

    println!("answer part 2: {}", o * c);

    Ok(())
}

fn generator(mut numbers: Vec<String>, index: usize, oxygen: bool) -> Vec<String> {
    if numbers.len() == 1{
        return numbers;
    }

    let mut zero_count = 0;
    let mut one_count = 0;

    for x in &numbers {
        let bit = x.chars().nth(index).unwrap();

        match bit {
            '0' => zero_count = zero_count + 1,
            '1' => one_count = one_count + 1,
            _ => println!("invalid"),
        }
    }

    if zero_count > one_count {
        if oxygen {
            numbers.retain(|num| num.chars().nth(index).unwrap() == '0');
        } else {
            numbers.retain(|num| num.chars().nth(index).unwrap() == '1');
        }
    } else {
        if oxygen {
            numbers.retain(|num| num.chars().nth(index).unwrap() == '1');

        } else {
            numbers.retain(|num| num.chars().nth(index).unwrap() == '0');
        }
    }

    generator(numbers, index + 1, oxygen)
}
