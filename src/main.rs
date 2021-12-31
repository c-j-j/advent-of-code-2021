#![allow(dead_code)]

use std::fs;

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let input = fs::read_to_string("./src/day10-input").unwrap();
    let answer = day10::part2(input);
    println!("Answer: {}", answer);
}
