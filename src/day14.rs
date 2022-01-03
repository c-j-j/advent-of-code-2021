use std::collections::HashMap;
use std::str::FromStr;

// letter -> [inserted_letters]

#[derive(Debug)]
struct Letter {
    c: char,
    next_inserted: Vec<char>,
}

pub fn part1(input: String) -> i32 {
    let mut split = input.split("\n\n");
    let template_input: String = split.next().unwrap().to_string();
    let instructions: Vec<(String, char)> = split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut line_split = line.split(" -> ");
            let insertion_point = line_split.next().unwrap();
            let insert = line_split.next().unwrap();
            (
                String::from(insertion_point),
                char::from_str(insert).unwrap(),
            )
        })
        .collect();

    let mut template: Vec<Letter> = template_input
        .chars()
        .map(|c| Letter {
            c,
            next_inserted: vec![],
        })
        .collect();

    for step in 0..40 {
        println!("step AAA {} {}", step, template.len());
        for (insertion, insert) in &instructions {
            for i in 0..template.len() - 1 {
                let joined_pair: String = vec![template[i].c, template[i + 1].c].iter().collect();
                if joined_pair == *insertion {
                    template[i].next_inserted.push(*insert);
                }
            }
        }
        let new_template: Vec<Letter> = template
            .iter()
            .flat_map(|letter| {
                let c = letter.c;
                let mut next = letter.next_inserted.clone();
                let mut a = vec![c];
                a.append(&mut next);
                a.iter()
                    .map(|c| Letter {
                        c: *c,
                        next_inserted: vec![],
                    })
                    .collect::<Vec<Letter>>()
            })
            .collect::<Vec<Letter>>();
        template = new_template;
    }

    let mut counts: HashMap<char, i32> = HashMap::new();

    for letter in template.iter() {
        *counts.entry(letter.c).or_insert(0) += 1;
    }
    let formatted: String = template.iter().map(|letter| letter.c).collect();
    println!("{:?}", formatted);

    let min = counts.iter().min_by_key(|&(_, count)| count).unwrap();
    let max = counts.iter().max_by_key(|&(_, count)| count).unwrap();
    max.1 - min.1
}

#[test]
fn testing_part_1() {
    let input = String::from(
        "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C",
    );
    let answer = part1(input);
    assert_eq!(answer, 1588);
}
