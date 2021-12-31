fn get_xy(grid: &Vec<Vec<u32>>, x: i64, y: i64) -> u32 {
    if y < 0 || y >= grid.len() as i64 {
        return u32::max_value();
    }
    if x < 0 || x >= grid.first().unwrap().len() as i64 {
        return u32::max_value();
    }
    grid[y as usize][x as usize]
}

pub fn part1(input: String) -> u32 {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|i| i.to_digit(10).unwrap()).collect())
        .collect();

    let mut min_points: Vec<u32> = vec![];
    for y in 0..grid.len() {
        for x in 0..grid.first().unwrap().len() {
            let x = x as i64;
            let y = y as i64;
            let a: u32 = get_xy(&grid, x, y - 1);
            let b: u32 = get_xy(&grid, x - 1, y);
            let current: u32 = get_xy(&grid, x, y);
            let d: u32 = get_xy(&grid, x + 1, y);
            let e: u32 = get_xy(&grid, x, y + 1);

            let elements = vec![a, b, current, d, e];

            let min = *elements.iter().min().unwrap();

            if current == min && elements.iter().filter(|e| **e == min).count() == 1 {
                min_points.push(min);
            }
        }
    }
    min_points.iter().map(|x| x + 1).sum()
}

fn dfs(grid: &Vec<Vec<u32>>, x: i64, y: i64, visited: &mut Vec<(i64, i64)>) -> u32 {
    let current = get_xy(grid, x, y);

    if current >= 9 {
        return 0;
    }
    visited.push((x, y));

    let mut basin_size = 1;
    let neighbours = vec![(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)];

    for neighbour in neighbours {
        if !visited.contains(&neighbour) {
            basin_size = basin_size + dfs(grid, neighbour.0, neighbour.1, visited)
        }
    }

    basin_size
}

fn determine_basin(grid: &Vec<Vec<u32>>, x: i64, y: i64) -> u32 {
    let mut visited = vec![(x, y)];

    dfs(grid, x, y, &mut visited)
}

pub fn part2(input: String) -> u32 {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|i| i.to_digit(10).unwrap()).collect())
        .collect();

    let mut basin_sizes: Vec<u32> = vec![];
    for y in 0..grid.len() {
        for x in 0..grid.first().unwrap().len() {
            let x = x as i64;
            let y = y as i64;
            let a: u32 = get_xy(&grid, x, y - 1);
            let b: u32 = get_xy(&grid, x - 1, y);
            let current: u32 = get_xy(&grid, x, y);
            let d: u32 = get_xy(&grid, x + 1, y);
            let e: u32 = get_xy(&grid, x, y + 1);

            let elements = vec![a, b, current, d, e];

            let min = *elements.iter().min().unwrap();

            if current == min && elements.iter().filter(|e| **e == min).count() == 1 {
                let basin_size = determine_basin(&grid, x, y);
                basin_sizes.push(basin_size);
            }
        }
    }
    basin_sizes.sort_by(|a, b| b.cmp(a));
    basin_sizes.iter().take(3).product()
}

#[test]
fn testing_part1() {
    let input = String::from(
        "2199943210
3987894921
9856789892
8767896789
9899965678",
    );
    let answer = part1(input);
    assert_eq!(answer, 15);
}

#[test]
fn testing_part2() {
    let input = String::from(
        "2199943210
3987894921
9856789892
8767896789
9899965678",
    );
    let answer = part2(input);
    assert_eq!(answer, 1134);
}
