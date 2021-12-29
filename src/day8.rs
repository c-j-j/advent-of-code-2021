use std::collections::HashMap;

pub fn part1(input: String) -> i64 {
    let mut total = 0;
    for line in input.lines() {
        let output = line.split('|').last().unwrap();
        let words: Vec<&str> = output.split_ascii_whitespace().collect();

        for word in words {
            let len = word.len();
            if len == 2 || len == 3 || len == 4 || len == 7 {
                total = total + 1;
            }
        }
    }
    total
}

fn contains_letters(word: &str, letters: &str) -> bool {
    letters.chars().all(|c| word.contains(c))
}

fn find_word_containing(words: &Vec<String>, num_letters: usize, word: &String) -> String {
    let filtered_words: Vec<String> = words
        .iter()
        .filter(|w| w.len() == num_letters && contains_letters(w, word))
        .map(|w| w.to_string())
        .collect();

    if filtered_words.len() == 0 {
        panic!("Word not found in words {}", word);
    }

    if filtered_words.len() > 1 {
        panic!("Multiple words found {}", word);
    }
    filtered_words.first().unwrap().to_string()
}

fn count_letters(words: &Vec<String>) -> HashMap<char, i64> {
    let mut result: HashMap<char, i64> = HashMap::new();

    for word in words {
        for char in word.chars() {
            *result.entry(char).or_insert(0) += 1;
        }
    }
    result
}

fn get_output_value(output: &String, mapping: HashMap<char, char>) -> i64 {
    let mut numbers = HashMap::new();
    numbers.insert("abcefg", "0");
    numbers.insert("cf", "1");
    numbers.insert("acdeg", "2");
    numbers.insert("acdfg", "3");
    numbers.insert("bcdf", "4");
    numbers.insert("abdfg", "5");
    numbers.insert("abdefg", "6");
    numbers.insert("acf", "7");
    numbers.insert("abcdefg", "8");
    numbers.insert("abcdfg", "9");
    let mapped_output: Vec<String> = output
        .split_ascii_whitespace()
        .map(|word| {
            word.chars()
                .map(|c| mapping.get(&c).unwrap().clone())
                .collect::<String>()
        })
        .collect();

    let result: Vec<&str> = mapped_output
        .iter()
        .map(|word| {
            let mut key: Vec<char> = word.chars().collect();
            key.sort_by(|a, b| a.cmp(b));
            let sorted_key = key.into_iter().collect::<String>();
            numbers.get(sorted_key.as_str()).unwrap().clone()
        })
        .collect();
    result.join("").parse().unwrap()
}

pub fn part2(input: String) -> i64 {
    let mut total = 0;
    for line in input.lines() {
        let mut line_input = line.split('|').into_iter();
        let input = line_input.next().unwrap();
        let output = String::from(line_input.next().unwrap().trim());

        let words: Vec<String> = input
            .split_ascii_whitespace()
            .map(|w| w.to_string())
            .collect();

        let one = words.iter().find(|w| w.len() == 2).unwrap().clone();
        let four = words.iter().find(|w| w.len() == 4).unwrap().clone();
        let seven = words.iter().find(|w| w.len() == 3).unwrap().clone();

        let counts = count_letters(&words);

        let a = seven.chars().find(|c| !one.contains(*c)).unwrap();
        let b = counts.iter().find(|(_k, v)| **v == 6).unwrap().0.clone();
        let c = counts
            .iter()
            .find(|(k, v)| **v == 8 && **k != a)
            .unwrap()
            .0
            .clone();
        let d = counts
            .iter()
            .filter(|(_k, v)| **v == 7)
            .find(|(k, _v)| four.contains(**k))
            .unwrap()
            .0
            .clone();
        let e = counts.iter().find(|(_k, v)| **v == 4).unwrap().0.clone();
        let f = counts.iter().find(|(_k, v)| **v == 9).unwrap().0.clone();
        let g = counts
            .iter()
            .find(|(k, v)| **v == 7 && **k != d)
            .unwrap()
            .0
            .clone();
        let mut mapping = HashMap::new();
        mapping.insert(a, 'a');
        mapping.insert(b, 'b');
        mapping.insert(c, 'c');
        mapping.insert(d, 'd');
        mapping.insert(e, 'e');
        mapping.insert(f, 'f');
        mapping.insert(g, 'g');

        total = total + get_output_value(&output, mapping);
    }
    total
}

#[test]
fn testing_part1() {
    let input = String::from(
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
fcgedb cgb dgebacf gc",
    );
    let answer = part1(input);
    assert_eq!(answer, 5);
}

// 0 - 6
// 1 - 2
// 2 - 5
// 3 - 5
// 4 - 4
// 5 - 5
// 6 - 6
// 7 - 3
// 8 - 7
// 9 - 6

/**
a - 8 - done
b - 6
c - 8
d - 7
e - 4
f - 9
g - 7
*/

/**
a - 02356789
b - 045689
c - 01234789
d - 2345689
e - 0248
f - easy
g - 0235689
 */

#[test]
fn testing_part2() {
    let input = String::from(
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
    );
    let answer = part2(input);
    assert_eq!(answer, 5353);
}
