use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let input = fs::read_to_string("./src/day4-input").unwrap();
    let answer = day4::part2(input);
    println!("Answer: {}", answer);
}
