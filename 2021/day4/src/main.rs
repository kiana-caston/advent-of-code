use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("example_input.txt")?;
    let br = BufReader::new(file);

    let mut numbers = Vec::new();
    let mut boards = Vec::new();
    let mut temp_board = Vec::new();

    let mut first = true;

    for line in br.lines() {
        let l = line.unwrap();

        if first {
            numbers = l.split(',').map(String::from).collect();
            first = false;
        } else {
            if !l.is_empty() {
                let row: Vec<_> = l.split_whitespace().map(String::from).collect();

                temp_board.push(row);

                if temp_board.len() == 5 {
                    boards.push(temp_board);
                    temp_board = Vec::new();
                }
            }
        }
    }

    let mut call = 0;
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    let mut win = false;

    while call < numbers.len() {
        while i < boards.len() {
            let board = boards[i].clone();

            while j < board.len() {
                let row = board[j].clone();

                while k < row.len() {
                    let x = row[k].clone();

                    if numbers[call] == x {
                        boards[i][j][k] = "x".to_string();
                          if call > 3 {
                              win = check_win(&boards[i]);
                          }

                          if win {
                              break;
                          }
                    }
                    k = k + 1;
                }

                k = 0;
                j = j + 1;
            }

            if win {
                break;
            }

            k = 0;
            j = 0;
            i = i + 1;
        }

        if win {
            break;
        }

        call = call + 1;
        i = 0;
        j = 0;
        k = 0;
    }

    let score = score_board(&boards[i]);
    let answer = score * numbers[call].parse::<i32>().unwrap();

    println!("answer part 1: {}", answer);

    Ok(())
}

fn check_win(board: &Vec<Vec<String>>) -> bool {
    for row in board.clone() {
        if row == vec!["x", "x", "x", "x", "x"] {
            return true;
        }
    }

    let mut col = 0;
    let mut i = 0;
    let mut j = 0;

    while i < board.len() {
        if board[j][col] == "x" {
            j = j + 1;

            if j == 4 {
                return true;
            }
        } else {
            j = 0;
            i = i + 1;
            col = 0;
        }
    }

    return false;
}

fn score_board(board: &Vec<Vec<String>>) -> i32 {
    let mut score = 0;

    for row in board.clone() {
        let x = row.clone();

        for x in row {
            if x != "x" {
                score = score + x.parse::<i32>().unwrap();
            }
        }
    }

    return score;
}
