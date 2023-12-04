use std::fs;

pub fn main() {
    println!("Day Four");
    println!("------------ Part One Example ------------");
    let example_input = fs::read_to_string("src/inputs/day_four_example.txt")
        .expect("Failed to read file");

    let example_answer = part_one(&example_input);
    println!("Example answer: {}", example_answer);

    println!("------------ Part One ------------");

    let input = fs::read_to_string("src/inputs/day_four.txt")
        .expect("Failed to read file");

    let answer = part_one(&input);
    println!("Answer: {}", answer);

    println!("------------ Part Two Example ------------");

    let part_two_example_answer = part_two(&example_input);
    println!("Part Two Example Answer: {}", part_two_example_answer);

    println!("------------ Part Two ------------");

    let part_two_answer = part_two(&input);
    println!("Part Two Answer: {}", part_two_answer);
}

fn part_two(input: &str) -> i32 {
    let input_arr: Vec<&str> = input.split('\n').collect();

    let total_cards = get_winning_cards(&input_arr, &input_arr, 0);

    total_cards
}

fn get_winning_cards(base_cards: &Vec<&str>, input_arr: &Vec<&str>, mut total_cards: i32) -> i32 {
    total_cards = total_cards + input_arr.len() as i32;

    if input_arr.len() == 0 {
        return total_cards;
    }

    let mut won_cards: Vec<&str> = Vec::new();

    for (i, line) in input_arr.iter().enumerate() {
        let winning_count_res = get_winning_count(line);

        let winning_count = match winning_count_res {
            Ok(winning_count) => winning_count,
            Err(_) => {
                continue;
            }
        };

        if winning_count == 0 {
            continue;
        }

        let card_base_index_str = line.split_whitespace().nth(1).unwrap().split(':').nth(0).unwrap();
        let card_base_index = match card_base_index_str.parse::<i32>() {
            Ok(num) => num as usize - 1,
            Err(_) => {
                println!("Failed to parse '{}' as integer", card_base_index_str);
                println!("error on line {}", line);
                continue; // Skip this iteration if parsing fails
            }
        };

        for j in 1..winning_count + 1 {
            won_cards.push(base_cards[card_base_index + j as usize])
        }
    }

    let updated_total_cards = get_winning_cards(
        base_cards,
        &won_cards,
        total_cards,
    );

    return updated_total_cards;
}

fn part_one(input: &str) -> i32 {
    let mut ret_val = 0;

    for line in input.split('\n') {
        let winning_count_res = get_winning_count(line);

        let winning_count = match winning_count_res {
            Ok(winning_count) => winning_count,
            Err(_) => {
                continue;
            }
        };

        let mut points = 0;

        for _i in 0..winning_count {
            if points == 0 {
                points = 1;
            } else {
                points = points * 2;
            }
        }

        ret_val = ret_val + points;
    }

    ret_val
}

fn get_winning_count(line: &str) -> Result<i32, String> {
    // remove Card x: prefix
    let line_opt = line.split(':').nth(1);

    let line = match line_opt {
        Some(line) => line,
        None => {
            return Err("Failed to get line".to_string());
        }
    };

    let line = line.trim_start().trim_end();

    let candidate_numbers_opt = line.split('|').nth(0);
    let winning_numbers_opt = line.split('|').nth(1);

    let candidate_numbers_str= match candidate_numbers_opt {
        Some(candidate_numbers) => candidate_numbers,
        None => {
            return Err("Failed to get candidate numbers".to_string());
        }
    };

    let candidate_numbers_str = candidate_numbers_str.trim_end().trim_start();

    let winning_numbers_str = match winning_numbers_opt {
        Some(winning_numbers) => winning_numbers,
        None => {
            return Err("Failed to get winning numbers".to_string());
        }
    };

    let winning_numbers_str = winning_numbers_str.trim_end().trim_start();

    let mut candidate_numbers: Vec<i32> = Vec::new();

    for candidate_number in candidate_numbers_str.split(' ') {
        let candidate_number = candidate_number.trim_start().trim_end();

        let candidate_number = match candidate_number.parse::<i32>() {
            Ok(candidate_number) => candidate_number,
            Err(_) => {
                continue;
            }
        };

        candidate_numbers.push(candidate_number);
    }

    let mut winning_numbers: Vec<i32> = Vec::new();

    for winning_number in winning_numbers_str.split(' ') {
        let winning_number = winning_number.trim_start().trim_end();

        let winning_number = match winning_number.parse::<i32>() {
            Ok(winning_number) => winning_number,
            Err(_) => {
                continue;
            }
        };

        winning_numbers.push(winning_number);
    }

    let mut matches = 0;
    for candidate in candidate_numbers {
        for winning in &winning_numbers {
            if candidate == *winning {
                matches += 1;
            }
        }
    }

    Ok(matches)
}

#[test]
fn test_part_one_example() {
    let example_input = fs::read_to_string("src/inputs/day_four_example.txt")
        .expect("Failed to read file");

    assert_eq!(part_one(&example_input), 13);
}

#[test]
fn test_part_one() {
    let input = fs::read_to_string("src/inputs/day_four.txt")
        .expect("Failed to read file");

    assert_eq!(part_one(&input), 25010);
}

#[test]
fn test_part_two_example() {
    let example_input = fs::read_to_string("src/inputs/day_four_example.txt")
        .expect("Failed to read file");

    assert_eq!(part_two(&example_input), 30);
}
