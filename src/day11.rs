type Stack = Vec<char>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn get_neighbours(x: usize, y: usize, width: usize, height: usize) -> Vec<Point> {
    let diffs: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    diffs
        .into_iter()
        .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < width as i32 && *y < height as i32)
        .map(|(x, y)| Point {
            x: x as usize,
            y: y as usize,
        })
        .collect()
}

fn run_step(grid: &mut Vec<Vec<u32>>) -> usize {
    let height = grid.len();
    let width = grid.first().unwrap().len();

    for y in 0..height {
        for x in 0..width {
            grid[y][x] += 1;
        }
    }

    let mut num_flashes: usize = 0;

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] > 9 {
                grid[y][x] = 0;
                num_flashes += 1;
                let mut octopuses_to_increase = get_neighbours(x, y, width, height);

                while !&octopuses_to_increase.is_empty() {
                    let current_neighbours = octopuses_to_increase.clone();
                    for neighbour in current_neighbours {
                        let index = octopuses_to_increase
                            .iter()
                            .position(|o| *o == neighbour)
                            .unwrap();
                        octopuses_to_increase.remove(index);
                        if grid[neighbour.y][neighbour.x] > 0 {
                            grid[neighbour.y][neighbour.x] += 1;
                        }

                        if grid[neighbour.y][neighbour.x] > 9 {
                            num_flashes += 1;
                            grid[neighbour.y][neighbour.x] = 0;
                            octopuses_to_increase.append(&mut get_neighbours(
                                neighbour.x,
                                neighbour.y,
                                width,
                                height,
                            ));
                        }
                    }
                }
            }
        }
    }

    num_flashes
}

pub fn part1(input: String, num_steps: u32) -> usize {
    let mut grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|i| i.to_digit(10).unwrap()).collect())
        .collect();

    let mut total_flashes = 0;
    for _ in 0..num_steps {
        total_flashes += run_step(&mut grid);
    }
    total_flashes
}

pub fn part2(input: String) -> usize {
    let mut grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|i| i.to_digit(10).unwrap()).collect())
        .collect();

    let total_octopuses = grid.len() * grid.first().unwrap().len();
    for step in 0.. {
        let num_booms = run_step(&mut grid);
        if num_booms == total_octopuses {
            return step + 1;
        }
    }
    panic!("No step found");
}

#[test]
fn testing_get_neighbours() {
    assert_eq!(
        get_neighbours(0, 0, 4, 4),
        vec!(
            Point { x: 0, y: 1 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 }
        )
    );
    assert_eq!(
        get_neighbours(1, 1, 4, 4),
        vec!(
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 0 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 2 }
        )
    )
}

#[test]
fn testing_part_1() {
    let input = String::from(
        "11111
19991
19191
19991
11111
",
    );
    let answer = part1(input, 1);
    assert_eq!(answer, 9);

    let input = String::from(
        "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
    );
    let answer = part1(input, 100);
    assert_eq!(answer, 1656);
}

#[test]
fn testing_part_2() {
    let input = String::from(
        "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
    );
    let answer = part2(input);
    assert_eq!(answer, 195);
}
