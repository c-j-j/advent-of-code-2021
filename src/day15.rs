use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: (usize, usize),
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
    }
}

pub fn part1(input: String) -> Option<i32> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let numbers: Vec<i32> = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32))
        .collect();

    let mut distances = vec![i32::MAX; width * height];
    distances[0] = 0;

    let mut queue = BinaryHeap::new();
    queue.push(State {
        cost: 0,
        position: (0, 0),
    });

    while let Some(State { cost, position }) = queue.pop() {
        if position == (width - 1, height - 1) {
            return Some(cost as i32);
        }

        // if cost optimisation can go here

        let mut edges = vec![(position.0 + 1, position.1), (position.0, position.1 + 1)];

        if position.0 > 0 {
            edges.push((position.0 - 1, position.1));
        }

        if position.1 > 0 {
            edges.push((position.0, position.1 - 1));
        }

        for edge in edges {
            if edge.0 >= width || edge.1 >= height {
                continue;
            }

            let cost_to_node = numbers[edge.1 * width + edge.0] as i32;

            let next = State {
                cost: cost + cost_to_node,
                position: edge,
            };

            let i = next.position.1 * width + next.position.0;
            if next.cost < distances[i] {
                distances[next.position.1 * width + next.position.0] = next.cost as i32;
                queue.push(next);
            }
        }
    }
    None
}

fn increment(x: i32) -> i32 {
    if x < 9 {
        x + 1
    } else {
        1
    }
}

pub fn part2(input: String) -> Option<i32> {
    let tile_width = input.lines().next().unwrap().len();
    let tile_height = input.lines().count();
    let width = tile_width * 5;
    let height = tile_height * 5;

    let number_tile: Vec<i32> = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32))
        .collect();

    let mut numbers = vec![i32::MAX; width * height];

    println!("width {} height {}", width, height);

    for i in 0..numbers.len() {
        let x = i % width;
        let y = i / height;

        if x < tile_width && y < tile_height {
            numbers[i] = number_tile[y * tile_width + x];
        } else if x >= tile_width {
            let tile_x = x - tile_width;
            let tile_y = y;
            numbers[i] = increment(numbers[tile_y * width + tile_x]);
        } else if y >= 10 {
            let tile_x = x;
            let tile_y = y - tile_height;
            numbers[i] = increment(numbers[tile_y * width + tile_x]);
        }
    }

    // print numbers as 2d array
    for x in 0..width {
        for y in 0..height {
            print!("{}", numbers[x * width + y]);
        }
        println!();
    }

    let mut distances = vec![i32::MAX; width * height];
    distances[0] = 0;

    let mut queue = BinaryHeap::new();
    queue.push(State {
        cost: 0,
        position: (0, 0),
    });

    while let Some(State { cost, position }) = queue.pop() {
        if position == (width - 1, height - 1) {
            return Some(cost as i32);
        }

        // if cost optimisation can go here

        let mut edges = vec![(position.0 + 1, position.1), (position.0, position.1 + 1)];

        if position.0 > 0 {
            edges.push((position.0 - 1, position.1));
        }

        if position.1 > 0 {
            edges.push((position.0, position.1 - 1));
        }

        for edge in edges {
            if edge.0 >= width || edge.1 >= height {
                continue;
            }

            let cost_to_node = numbers[edge.1 * width + edge.0] as i32;

            let next = State {
                cost: cost + cost_to_node,
                position: edge,
            };

            let i = next.position.1 * width + next.position.0;
            if next.cost < distances[i] {
                distances[next.position.1 * width + next.position.0] = next.cost as i32;
                queue.push(next);
            }
        }
    }
    None
}

#[test]
fn testing_part_1() {
    let input = String::from(
        "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581",
    );
    let answer = part2(input);
    assert_eq!(answer, Some(315));
}
