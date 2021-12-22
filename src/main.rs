use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let input = fs::read_to_string("./src/day5-input").unwrap();
    let answer = day5::part2(input);
    println!("Answer: {}", answer);
}
