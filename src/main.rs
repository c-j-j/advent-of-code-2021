#![allow(dead_code)]

use std::fs;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let input = fs::read_to_string("./src/day15-input").unwrap();
    let answer = day15::part2(input);
    println!("Answer: {:?}", answer);
}
