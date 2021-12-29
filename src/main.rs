#![allow(dead_code)]

use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let input = fs::read_to_string("./src/day7-input").unwrap();
    let answer = day7::part2(input);
    println!("Answer: {}", answer);
}
