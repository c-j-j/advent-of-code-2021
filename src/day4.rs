type Line = Vec<u32>;
type Board = Vec<u32>;

fn parse_input(input: String) -> (Vec<u32>, Vec<Board>) {
    let chunks: Vec<&str> = input.split("\n\n").collect();
    let mut chunk_iter = chunks.iter();

    let number_to_drawn = chunk_iter
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let boards: Vec<Board> = chunk_iter
        .map(|chunk| {
            chunk
                .split_ascii_whitespace()
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect();
    (number_to_drawn, boards)
}

fn get_winning_line(drawn_numbers: &Vec<u32>, board: &Board) -> Option<Line> {
    let rows: Vec<Line> = get_rows(board);
    let cols: Vec<Line> = get_cols(board);

    let row = rows
        .iter()
        .find(|r| r.iter().all(|n| drawn_numbers.contains(n)));
    if row.is_some() {
        return row.map(|r| r.to_owned());
    } else {
        let col = cols
            .iter()
            .find(|r| r.iter().all(|n| drawn_numbers.contains(n)));

        if col.is_some() {
            return col.map(|c| c.to_owned());
        }
    }
    None
}

fn get_rows(board: &Board) -> Vec<Line> {
    board.chunks(5).map(|c| c.to_owned()).collect()
}

fn get_cols(board: &Board) -> Vec<Line> {
    let mut cols: Vec<Vec<u32>> = vec![];
    for r in 0..5 {
        let mut next_col: Vec<u32> = vec![];
        for c in 0..5 {
            let index = r + (c * 5);
            let next = board[index];
            next_col.push(next)
        }
        cols.push(next_col);
    }
    cols
}

fn find_winning_board<'a>(drawn_numbers: &Vec<u32>, boards: &'a Vec<Board>) -> Option<&'a Board> {
    boards
        .iter()
        .find(|board| get_winning_line(drawn_numbers, board).is_some())
}

pub fn part1(input: String) -> u32 {
    let (numbers_to_draw, mut boards) = parse_input(input);

    let mut drawn: Vec<u32> = vec![];

    let mut last_won_board: Option<&Board> = None;
    for next_number in numbers_to_draw {
        drawn.push(next_number);

        for board in boards.iter() {
            if get_winning_line(&drawn, board).is_some() {
                last_won_board = Some(board);
            }
        }

        if last_won_board.is_some() {
            break;
        }
    }

    let unmarked: Vec<u32> = last_won_board
        .map(|board| {
            board
                .iter()
                .filter(|n| !drawn.contains(*n))
                .map(|n| *n)
                .collect()
        })
        .unwrap();

    return unmarked.iter().sum::<u32>() * drawn.last().unwrap();
}

pub fn part2(input: String) -> u32 {
    let (numbers_to_draw, mut boards) = parse_input(input);

    let mut drawn: Vec<u32> = vec![];
    let mut won_boards: Vec<bool> = vec![false; boards.len()];
    let mut last_won_board: Option<&Board> = None;
    for next_number in numbers_to_draw {
        drawn.push(next_number);

        for (i, board) in boards.iter().enumerate() {
            if let Some(false) = won_boards.get(i) {
                if get_winning_line(&drawn, board).is_some() {
                    last_won_board = Some(board);
                    won_boards[i] = true;
                }
            }
        }

        if won_boards.iter().all(|won| *won) {
            break;
        }
    }

    let unmarked: Vec<u32> = last_won_board
        .map(|board| {
            board
                .iter()
                .filter(|n| !drawn.contains(*n))
                .map(|n| *n)
                .collect()
        })
        .unwrap_or(vec![]);

    return unmarked.iter().sum::<u32>() * drawn.last().unwrap();
}

#[test]
fn testing_part1() {
    let input = String::from(
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7",
    );
    let answer = part1(input);
    assert_eq!(answer, 4512);
}

#[test]
fn testing_part2() {
    let input = String::from(
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7",
    );
    let answer = part2(input);
    assert_eq!(answer, 1924);
}
