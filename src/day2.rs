use std::fmt::Formatter;

#[derive(Debug)]
pub struct Position {
    pub depth: i32,
    pub horizontal: i32,
    aim: i32,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "h:{} d:{}", self.horizontal, self.depth)
    }
}

fn parse_instruction(instruction: &str) -> Instruction {
    let input: Vec<&str> = instruction.split(' ').collect();
    let dir: &str = input.first().unwrap();
    let amount: i32 = input.last().unwrap().parse().unwrap();

    return match dir {
        "up" => Instruction::Up(amount),
        "down" => Instruction::Down(amount),
        "forward" => Instruction::Forward(amount),
        _ => panic!("Invalid instruction {}", dir),
    };
}

pub fn part1(input: String) -> Position {
    let directions: Vec<Instruction> = input.lines().map(|l| parse_instruction(l)).collect();

    directions
        .iter()
        .fold(Position::default(), |acc, next| match next {
            Instruction::Forward(amount) => Position {
                horizontal: acc.horizontal + amount,
                ..acc
            },
            Instruction::Down(amount) => Position {
                depth: acc.depth + amount,
                ..acc
            },
            Instruction::Up(amount) => Position {
                depth: acc.depth - amount,
                ..acc
            },
        })
}

pub fn part2(input: String) -> Position {
    let directions: Vec<Instruction> = input.lines().map(|l| parse_instruction(l)).collect();

    directions
        .iter()
        .fold(Position::default(), |acc, next| match next {
            Instruction::Forward(amount) => Position {
                horizontal: acc.horizontal + amount,
                depth: acc.depth + (acc.aim * amount),
                ..acc
            },
            Instruction::Down(amount) => Position {
                aim: acc.aim + amount,
                ..acc
            },
            Instruction::Up(amount) => Position {
                aim: acc.aim - amount,
                ..acc
            },
        })
}

#[test]
fn test_part1() {
    let input = String::from(
        "forward 5
down 5
forward 8
up 3
down 8
forward 2",
    );
    let answer = part1(input);
    assert_eq!(answer.horizontal, 15);
    assert_eq!(answer.depth, 10);
}

#[test]
fn test_part2() {
    let input = String::from(
        "forward 5
down 5
forward 8
up 3
down 8
forward 2",
    );
    let answer = part2(input);
    assert_eq!(answer.horizontal, 15);
    assert_eq!(answer.depth, 60);
}
