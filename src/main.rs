#![allow(dead_code)]

use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let input = fs::read_to_string("./src/day6-input").unwrap();
    let answer = day6::part2(input, 256);
    println!("Answer: {}", answer);
}
