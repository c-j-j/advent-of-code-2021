type Stack = Vec<char>;

fn process_line(line: &str) -> Result<Stack, char> {
    let mut stack = vec![];
    for char in line.chars() {
        match char {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '<' => stack.push('>'),
            '{' => stack.push('}'),
            ')' => {
                if stack.pop() != Some(')') {
                    return Err(char);
                }
            }
            '}' => {
                if stack.pop() != Some('}') {
                    return Err(char);
                }
            }
            ']' => {
                if stack.pop() != Some(']') {
                    return Err(char);
                }
            }
            '>' => {
                if stack.pop() != Some('>') {
                    return Err(char);
                }
            }

            _ => {}
        }
    }
    return Ok(stack);
}

pub fn part1(input: String) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut invalid_chars = vec![];
    for line in lines {
        if let Err(char) = process_line(line) {
            invalid_chars.push(char);
        }
    }
    let scores = invalid_chars
        .iter()
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("invalid invalid char"),
        })
        .collect::<Vec<u32>>();
    scores.iter().sum()
}

pub fn part2(input: String) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    let mut scores: Vec<u64> = vec![];
    for line in lines {
        if let Ok(stack) = process_line(line) {
            let mut line_score = 0;
            let reversed_stack: Stack = stack.into_iter().rev().collect();

            for item in reversed_stack {
                let char_score = match item {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                };

                line_score = (line_score * 5) + char_score;
            }
            scores.push(line_score);
        }
    }
    scores.sort();
    scores[scores.len() / 2]
}

#[test]
fn testing_process_line() {
    assert_eq!(process_line("(())"), Ok(vec![]));
    assert_eq!(process_line("<<>>"), Ok(vec![]));
    assert_eq!(process_line("{{}}"), Ok(vec![]));
    assert_eq!(process_line("[[]]"), Ok(vec![]));
    assert_eq!(process_line("(])"), Err(']'));
    assert_eq!(process_line("{()()()>"), Err('>'));
    assert_eq!(process_line("<([]){()}[{}])"), Err(')'));
}

#[test]
fn testing_part_1() {
    let input = String::from(
        "[({(<(())[]>[[{[]{<()<>>
    [(()[<>])]({[<{<<[]>>(
    {([(<{}[<>[]}>{[]{[(<()>
    (((({<>}<{<{<>}{[]{[]{}
    [[<[([]))<([[{}[[()]]]
    [{[{({}]{}}([{[{{{}}([]
    {<[[]]>}<{[{[{[]{()[[[]
    [<(<(<(<{}))><([]([]()
    <{([([[(<>()){}]>(<<{{
    <{([{{}}[<[[[<>{}]]]>[]]",
    );
    let answer = part1(input);
    assert_eq!(answer, 26397);
}

#[test]
fn testing_part_2() {
    let input = String::from(
        "[({(<(())[]>[[{[]{<()<>>
    [(()[<>])]({[<{<<[]>>(
    {([(<{}[<>[]}>{[]{[(<()>
    (((({<>}<{<{<>}{[]{[]{}
    [[<[([]))<([[{}[[()]]]
    [{[{({}]{}}([{[{{{}}([]
    {<[[]]>}<{[{[{[]{()[[[]
    [<(<(<(<{}))><([]([]()
    <{([([[(<>()){}]>(<<{{
    <{([{{}}[<[[[<>{}]]]>[]]",
    );
    let answer = part2(input);
    assert_eq!(answer, 288957);
}
