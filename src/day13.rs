#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Instruction {
    axis: char,
    amount: usize,
}

fn parse_coordinate(line: &str) -> Point {
    let mut split = line.split(',');
    let x = split.next().unwrap();
    let y = split.next().unwrap();
    Point {
        x: x.parse().unwrap(),
        y: y.parse().unwrap(),
    }
}

fn parse_instruction(line: &str) -> Instruction {
    let mut split = line.split('=');
    let axis = split.next().unwrap().chars().last().unwrap();
    let amount = split.next().unwrap();
    Instruction {
        axis,
        amount: amount.parse().unwrap(),
    }
}

pub fn part1(input: String, num_folds: Option<i32>) -> i32 {
    let mut split = input.split("\n\n");
    let coordinates: Vec<Point> = split
        .next()
        .unwrap()
        .lines()
        .map(|l| parse_coordinate(l))
        .collect();
    let instructions: Vec<Instruction> = split
        .next()
        .unwrap()
        .lines()
        .map(|l| parse_instruction(l))
        .collect();

    let max_x = coordinates
        .iter()
        .map(|coordinate| coordinate.x)
        .max()
        .map(|n| n + 1)
        .unwrap();
    let max_y = coordinates
        .iter()
        .map(|coordinate| coordinate.y)
        .max()
        .map(|n| n + 1)
        .unwrap();

    let mut grid: Vec<Vec<char>> = vec![];
    for i in 0..max_y {
        grid.push(vec![]);
        for _ in 0..max_x {
            grid[i].push('.');
        }
    }

    for coordinate in coordinates.iter() {
        let y = coordinate.y;
        let x = coordinate.x;
        grid[y][x] = '#';
    }

    for instruction in instructions.iter().take(num_folds.unwrap_or(100) as usize) {
        for y in 0..grid.len() {
            for x in 0..grid.first().unwrap().len() {
                if grid[y][x] == '#' {
                    if instruction.axis == 'y' && y >= instruction.amount {
                        let new_y = reflect_on_axis(instruction.amount, y);
                        grid[new_y][x] = '#';
                    }

                    if instruction.axis == 'x' && x >= instruction.amount {
                        let new_x = reflect_on_axis(instruction.amount, x);
                        grid[y][new_x] = '#';
                    }
                }
            }
        }
        if instruction.axis == 'y' {
            grid.drain(instruction.amount..);
        }

        if instruction.axis == 'x' {
            for i in 0..grid.len() {
                grid[i].drain(instruction.amount..);
            }
        }
    }

    // Login to print this code out for part 2
    // for line in grid.iter() {
    //     let formatted_line: String = line.into_iter().collect();
    //     println!("{}", formatted_line)

    // FPEKBEJL
    // }

    grid.iter()
        .flat_map(|col| col.iter().filter(|cell| **cell == '#'))
        .count() as i32
}

fn reflect_on_axis(line: usize, y: usize) -> usize {
    line - (y - line)
}

#[test]
fn test_determine_new_y() {
    assert_eq!(reflect_on_axis(7, 7), 7);
    assert_eq!(reflect_on_axis(7, 8), 6);
    assert_eq!(reflect_on_axis(7, 14), 0)
}

#[test]
fn testing_part_1() {
    let input = String::from(
        "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5",
    );
    let answer = part1(input.clone(), Some(1));
    assert_eq!(answer, 17);

    let answer = part1(input, Some(2));
    assert_eq!(answer, 16);
}
