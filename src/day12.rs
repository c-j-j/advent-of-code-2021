use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Cave {
    small: bool,
    name: String,
}

fn parse_line(line: &str) -> (Cave, Cave) {
    let mut split = line.split('-');
    let a = split.next().unwrap();
    let b = split.next().unwrap();
    let cave_a = Cave {
        name: String::from(a),
        small: !(a.to_uppercase() == a),
    };
    let cave_b = Cave {
        name: String::from(b),
        small: !(b.to_uppercase() == b),
    };
    (cave_a, cave_b)
}

fn get_neighbours(connections: &Vec<(Cave, Cave)>, current: &String) -> Vec<Cave> {
    connections
        .into_iter()
        .filter(|(a, b)| a.name == *current || b.name == *current)
        .map(|(cave_a, cave_b)| {
            if cave_a.name == *current {
                cave_b.to_owned()
            } else {
                cave_a.to_owned()
            }
        })
        .collect()
}

fn dfs(connections: &Vec<(Cave, Cave)>, current: &String, current_path: &mut Vec<String>) -> i32 {
    if current == "end" {
        return 1;
    }

    let mut sum = 0;
    for neighbour in get_neighbours(&connections, current).iter() {
        if !current_path.contains(&neighbour.name) || !neighbour.small {
            current_path.push(neighbour.name.clone());
            sum += dfs(connections, &neighbour.name, current_path);
            current_path.pop();
        }
    }
    sum
}

fn determine_if_two_small_caves_exist(path: &Vec<Cave>) -> bool {
    let mut sums: HashMap<&Cave, i32> = HashMap::new();
    for node in path {
        *sums.entry(node).or_insert(0) += 1;
    }
    sums.into_iter().find(|(k, v)| *v > 1 && k.small).is_some()
}

fn dfs2(connections: &Vec<(Cave, Cave)>, current: &String, current_path: &mut Vec<Cave>) -> i32 {
    if current == "end" {
        return 1;
    }
    let mut sum = 0;
    for neighbour in get_neighbours(&connections, current).iter() {
        let has_two_small_caves = determine_if_two_small_caves_exist(current_path);
        if neighbour.name != "start"
            && (!neighbour.small || !has_two_small_caves || !current_path.contains(&neighbour))
        {
            current_path.push(neighbour.clone());
            sum += dfs2(connections, &neighbour.name, current_path);
            current_path.pop();
        }
    }
    sum
}

pub fn part1(input: String) -> i32 {
    let connections: Vec<(Cave, Cave)> = input.lines().map(|l| parse_line(l)).collect();

    let node = String::from("start");

    dfs(&connections, &node, &mut vec![String::from("start")])
}

pub fn part2(input: String) -> i32 {
    let connections: Vec<(Cave, Cave)> = input.lines().map(|l| parse_line(l)).collect();

    let node = String::from("start");

    dfs2(
        &connections,
        &node,
        &mut vec![Cave {
            name: String::from("start"),
            small: true,
        }],
    )
}

#[test]
fn testing_part_1() {
    let input = String::from(
        "start-A
start-b
A-c
A-b
b-d
A-end
b-end",
    );
    let answer = part1(input);
    assert_eq!(answer, 10);

    let input = String::from(
        "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
    );
    let answer = part1(input);
    assert_eq!(answer, 19);
}

#[test]
fn testing_part_2() {
    let input = String::from(
        "start-A
start-b
A-c
A-b
b-d
A-end
b-end",
    );
    let answer = part2(input);
    assert_eq!(answer, 36);

    let input = String::from(
        "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
    );
    let answer = part2(input);
    assert_eq!(answer, 103);
}
