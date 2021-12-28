#[allow(dead_code)]

fn determine_next_state(current_state: &Vec<i64>) -> Vec<i64> {
    let mut next = current_state.clone();

    let num_new = next.iter().filter(|n| **n == 0).count();

    for i in 0..next.len() {
        if next[i] == 0 {
            next[i] = 6;
        } else {
            next[i] = next[i] - 1;
        }
    }

    for _i in 0..num_new {
        next.push(8)
    }
    next
}

// much more performant version
fn determine_next_state_2(counts: &Vec<i64>) -> Vec<i64> {
    let mut next = counts.clone();
    let num_fish_to_reset = next.remove(0);
    next.push(0);

    next[6] = next[6] + num_fish_to_reset;
    next[8] = num_fish_to_reset;
    next
}

pub fn part1(input: String, num_days: i64) -> i64 {
    let mut initial_fish: Vec<i64> = input.split(',').map(|c| c.parse().unwrap()).collect();

    for _n in 0..num_days {
        initial_fish = determine_next_state(&initial_fish);
    }
    initial_fish.len() as i64
}

pub fn part2(input: String, num_days: i64) -> i64 {
    // we want to store an array of buckets, which is keyed by age (in days) and valued by number of fish in that day
    // {0: 4, 2, 2}... etc

    let initial_fish: Vec<i64> = input.split(',').map(|c| c.parse().unwrap()).collect();

    let mut counts: Vec<i64> = vec![0; 9];
    for fish in initial_fish {
        let index = fish as usize;
        counts[index] = counts[index] + 1
    }

    for _n in 0..num_days {
        counts = determine_next_state_2(&counts);
    }

    counts.iter().sum()
}

#[test]
fn testing_part1() {
    let input = String::from("3,4,3,1,2");
    let answer = part1(input, 18);
    assert_eq!(answer, 26);

    let input = String::from("3,4,3,1,2");
    let answer = part1(input, 80);
    assert_eq!(answer, 5934);
}

#[test]
fn testing_part2() {
    let mut a = vec![1, 2, 3];

    a.remove(0);
    let input = String::from("3,4,3,1,2");
    let answer = part2(input, 18);
    assert_eq!(answer, 26);

    let input = String::from("3,4,3,1,2");
    let answer = part2(input, 80);
    assert_eq!(answer, 5934);

    let input = String::from("3,4,3,1,2");
    let answer = part2(input, 256);
    assert_eq!(answer, 26984457539);
}
