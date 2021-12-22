fn add_bits(arr: Vec<i32>, next: Vec<char>) -> Vec<i32> {
    (0..arr.len())
        .map(|i| {
            let amount = if next[i].eq(&'1') { 1 } else { -1 };
            arr[i] + amount
        })
        .collect()
}

fn get_epsilon(running_total: &Vec<i32>) -> i32 {
    let binary: String = running_total
        .iter()
        .map(|next| if next > &0 { '1' } else { '0' })
        .collect();

    i32::from_str_radix(binary.as_str(), 2).unwrap()
}

fn get_gamma(running_total: &Vec<i32>) -> i32 {
    let binary: String = running_total
        .iter()
        .map(|next| if next > &0 { '0' } else { '1' })
        .collect();
    i32::from_str_radix(binary.as_str(), 2).unwrap()
}

pub fn part1(input: String) -> i32 {
    let first = input.lines().next().unwrap();
    let length = first.len();
    let mut running_total: Vec<i32> = vec![0; length];

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        running_total = add_bits(running_total, chars);
    }
    let epsilon = get_epsilon(&running_total);
    let gamma = get_gamma(&running_total);

    epsilon * gamma
}

fn filter_down(input: &String, find_most_common: bool) -> Vec<u32> {
    let mut candidate_codes: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut bit_index = 0;
    while candidate_codes.len() > 1 {
        let mut num_ones = 0;
        let mut num_zeros = 0;

        for code in &candidate_codes {
            let next = code.iter().nth(bit_index).unwrap();

            if next > &0 {
                num_ones = num_ones + 1;
            } else {
                num_zeros = num_zeros + 1;
            }
        }

        let num_to_filter = if find_most_common {
            if num_ones >= num_zeros {
                1
            } else {
                0
            }
        } else {
            if num_ones >= num_zeros {
                0
            } else {
                1
            }
        };

        candidate_codes = candidate_codes
            .into_iter()
            .filter(|c| c.iter().nth(bit_index).unwrap().eq(&num_to_filter))
            .collect();
        bit_index = bit_index + 1;
    }

    candidate_codes.iter().next().unwrap().to_owned()
}

fn get_decimal_value(bits: Vec<u32>) -> i32 {
    let binary: String = bits
        .iter()
        .map(|next| if next > &0 { '1' } else { '0' })
        .collect();

    i32::from_str_radix(binary.as_str(), 2).unwrap()
}

fn get_oxygen_generator_rating(input: &String) -> i32 {
    let answer = filter_down(input, true);
    println!("oxygen_bits:{:?}", answer);
    get_decimal_value(answer)
}

fn get_co2_scrubber_rating(input: &String) -> i32 {
    let answer = filter_down(input, false);
    println!("co2_bits:{:?}", answer);
    get_decimal_value(answer)
}

pub fn part2(input: String) -> i32 {
    let oxygen_generator_rating = get_oxygen_generator_rating(&input);
    println!("oxygen {}", oxygen_generator_rating);
    let co2_scrubber_rating = get_co2_scrubber_rating(&input);
    println!("co2_scrubber_rating {}", co2_scrubber_rating);
    oxygen_generator_rating * co2_scrubber_rating
}

#[test]
fn test_part_one() {
    let input = String::from(
        "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
    );
    assert_eq!(part1(input), 198);
}

#[test]
fn test_part_two() {
    let input = String::from(
        "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
    );
    assert_eq!(part2(input), 230);
}
