#![allow(unused)]

use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input/day03.txt").unwrap();

    let matrix = input.lines().map(|s| s.to_string()).collect();

    part_one(&matrix);
    part_two(&matrix);
}

fn part_one(matrix: &Vec<String>) { // takes List of strings
    let number_re = Regex::new(r"\d+").unwrap(); // Creates a regular expression to match digits
    let mut sum = 0;

    for i in 0..matrix.len() { // Iterates over each row in the matrix
        let row = &matrix[i]; // Gets current row

        'outer: for number in number_re.find_iter(row) { // // Iterates over each digit in the current row
            // Check the row above
            if i > 0 {
                let above_row = &matrix[i - 1].as_bytes();
                // Checks the symbols above the current digit
                for j in number.start().saturating_sub(1)..min(above_row.len(), number.end() + 1) 
                {
                    if above_row[j] != b'.' && !above_row[j].is_ascii_digit() {
                        // If a non-digit character is found above, add the digit to the sum and move to the next digit
                        sum += number.as_str().parse::<i32>().unwrap();
                        continue 'outer;
                    }
                }
            }

            // Check the row below
            if i < matrix.len() - 1 {
                let below_row = &matrix[i + 1].as_bytes();

                for j in number.start().saturating_sub(1)..min(below_row.len(), number.end() + 1) 
                {
                    if below_row[j] != b'.' && !below_row[j].is_ascii_digit() {
                        sum += number.as_str().parse::<i32>().unwrap();
                        continue 'outer;
                    }
                }
            }

            // Check the index to the left
            if number.start() > 0 {
                let left = row.as_bytes()[number.start() - 1];

                if left != b'.' && !left.is_ascii_digit() {
                    sum += number.as_str().parse::<i32>().unwrap();
                    continue 'outer;
                }
            }

            // Check the index to the right
            if number.end() < row.len() {
                let right = row.as_bytes()[number.end()];

                if right != b'.' && !right.is_ascii_digit() {
                    sum += number.as_str().parse::<i32>().unwrap();
                    continue 'outer;
                }
            }
        }
    }

    println!("Part 1: {}", sum);
}

fn part_two(matrix: &Vec<String>) { // Takes a list of strings
    let number_re = Regex::new(r"\d+").unwrap();
    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for i in 0..matrix.len() {
        let row = &matrix[i];

        for number in number_re.find_iter(row) {
            // Check the row above
            if i > 0 {
                let above_row = &matrix[i - 1].as_bytes();

                for j in number.start().saturating_sub(1)..min(above_row.len(), number.end() + 1) 
                {
                    if above_row[j] == b'*' {
                        gears.entry((i - 1, j))
                            .or_default()
                            .push(number.as_str().parse::<i32>().unwrap());
                    }
                }
            }

            // Check the row below
            if i < matrix.len() - 1 {
                let below_row = &matrix[i + 1].as_bytes();

                for j in number.start().saturating_sub(1)..min(below_row.len(), number.end() + 1) 
                {
                    if below_row[j] == b'*' {
                        gears.entry((i + 1, j)).or_default().push(number.as_str().parse::<i32>().unwrap());
                    }
                }
            }

            // Check the index to the left
            if number.start() > 0 {
                let left = row.as_bytes()[number.start() - 1];

                if left == b'*' {
                    gears.entry((i, number.start() - 1)).or_default().push(number.as_str().parse::<i32>().unwrap());
                }
            }

            // Check the index to the right
            if number.end() < row.len() {
                let right = row.as_bytes()[number.end()];

                if right == b'*' {
                    gears.entry((i, number.end())).or_default().push(number.as_str().parse::<i32>().unwrap());
                }
            }
        }
    }

    let mut sum = 0;
    gears.values().filter(|vec| vec.len() == 2).for_each(|vec| sum += vec[0] * vec[1]);

    println!("Part 2: {}", sum);
}