pub fn part1(input: String) -> i64 {
    let positions: Vec<i64> = input.split(',').map(|c| c.parse().unwrap()).collect();

    let mut min_fuel = i64::MAX;
    for (i, _) in positions.iter().enumerate() {
        let mut fuel = 0;
        for position in &positions {
            fuel = fuel + (position - (i as i64)).abs()
        }

        if fuel < min_fuel {
            min_fuel = fuel
        }
    }
    min_fuel
}

fn calculate_cost(x: i64) -> i64 {
    let mut sum = 0;
    for i in 1..x + 1 {
        sum = sum + i
    }
    sum
}

pub fn part2(input: String) -> i64 {
    let positions: Vec<i64> = input.split(',').map(|c| c.parse().unwrap()).collect();

    let mut min_fuel = i64::MAX;
    for (i, _) in positions.iter().enumerate() {
        let mut fuel = 0;
        for position in &positions {
            let cost = calculate_cost((position - (i as i64)).abs());

            fuel = fuel + cost;
        }

        if fuel < min_fuel {
            min_fuel = fuel
        }
    }
    min_fuel
}

#[test]
fn testing_part1() {
    let input = String::from("16,1,2,0,4,2,7,1,2,14");
    let answer = part1(input);
    assert_eq!(answer, 37);
}

#[test]
fn testing_part2() {
    let input = String::from("16,1,2,0,4,2,7,1,2,14");
    let answer = part2(input);
    assert_eq!(answer, 168);
}
