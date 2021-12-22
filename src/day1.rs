pub fn part1(input: String) -> i32 {
    let depths: Vec<i32> = input.lines().map(|r| r.parse().unwrap()).collect();

    let mut prev = depths.first().unwrap();
    let mut total_increases = 0;
    for depth in &depths {
        if depth - prev > 0 {
            total_increases = total_increases + 1;
        }
        prev = depth;
    }
    return total_increases;
}

pub fn part2(input: String) -> i32 {
    let depths: Vec<i32> = input.lines().map(|r| r.parse().unwrap()).collect();

    let depth_sums: Vec<i32> = depths.windows(3).map(|group| group.iter().sum()).collect();

    depth_sums.windows(2).fold(0, |acc, next|
        if next[1] > next[0] {
            acc + 1
        } else {
            acc
        }
    )
}


#[test]
fn test_part1() {
    let input = String::from("199
200
208
210
200
207
240
269
260
263");
    let answer = part1(input);
    assert_eq!(answer, 7);
}

#[test]
fn test_part2() {
    let input = String::from("199
200
208
210
200
207
240
269
260
263");
    let answer = part2(input);
    assert_eq!(answer, 5);
}
