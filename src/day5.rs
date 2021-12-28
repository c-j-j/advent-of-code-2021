#[allow(dead_code)]
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Line {
    x0: i32,
    x1: i32,
    y0: i32,
    y1: i32,
}

fn parse_line(line: &str) -> Line {
    let chunks: Vec<&str> = line.split(" -> ").collect();
    let from: Vec<i32> = chunks
        .first()
        .unwrap()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();
    let to: Vec<i32> = chunks
        .last()
        .unwrap()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();
    Line {
        x0: from[0],
        y0: from[1],
        x1: to[0],
        y1: to[1],
    }
}

fn parse_input(input: String) -> Vec<Line> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn sign(p0: i32) -> i32 {
    if p0 < 0 {
        return -1;
    } else if p0 > 0 {
        return 1;
    } else {
        return 0;
    }
}

fn is_vert_or_hor(line: &Line) -> bool {
    let dx = sign(line.x1 - line.x0);
    let dy = sign(line.y1 - line.y0);
    dx == 0 || dy == 0
}

pub fn part1(input: String) -> i32 {
    let lines = parse_input(input);
    let mut intersections: HashMap<String, i32> = HashMap::new();

    for line in lines.iter().filter(|line| is_vert_or_hor(line)) {
        let dx = sign(line.x1 - line.x0);
        let dy = sign(line.y1 - line.y0);

        let mut cx = line.x0;
        let mut cy = line.y0;

        let key = format!("{},{}", cx, cy);
        *intersections.entry(key).or_insert(0) += 1;

        while cx != line.x1 || cy != line.y1 {
            cx += dx;
            cy += dy;
            let key = format!("{},{}", cx, cy);
            *intersections.entry(key).or_insert(0) += 1;
        }
    }

    let num_intersections_gte_2 = intersections.iter().filter(|(_k, v)| **v >= 2).count();
    num_intersections_gte_2 as i32
}

pub fn part2(input: String) -> i32 {
    let lines = parse_input(input);
    let mut intersections: HashMap<String, i32> = HashMap::new();

    for line in lines.iter() {
        let dx = sign(line.x1 - line.x0);
        let dy = sign(line.y1 - line.y0);

        let mut cx = line.x0;
        let mut cy = line.y0;

        let key = format!("{},{}", cx, cy);
        *intersections.entry(key).or_insert(0) += 1;

        while cx != line.x1 || cy != line.y1 {
            cx += dx;
            cy += dy;
            let key = format!("{},{}", cx, cy);
            *intersections.entry(key).or_insert(0) += 1;
        }
    }

    let num_intersections_gte_2 = intersections.iter().filter(|(_k, v)| **v >= 2).count();
    num_intersections_gte_2 as i32
}

#[test]
fn testing_part1() {
    let input = String::from(
        "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
    );
    let answer = part1(input);
    assert_eq!(answer, 5);
    part2(String::from(""));
}
