#![allow(dead_code)]

use std::fs;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let input = fs::read_to_string("./src/day13-input").unwrap();
    let answer = day13::part1(input, None);
    println!("Answer: {}", answer);
}
