use std::{fs, collections::BTreeMap, vec};

pub fn main() {
    println!("------------ Part One Example ------------");
    let example_input = fs::read_to_string("src/inputs/day_three_example.txt")
        .expect("Failed to read file");

    let example_answer = part_one(&example_input);
    println!("Example answer: {}", example_answer);

    println!("------------ Part One ------------");

    let input = fs::read_to_string("src/inputs/day_three.txt")
        .expect("Failed to read file");

    let answer = part_one(&input);
    println!("Answer: {}", answer);

    println!("------------ Part Two Example ------------");

    let part_two_example_answer = part_two(&example_input);
    println!("Part Two Example Answer: {}", part_two_example_answer);

    // println!("------------ Part Two ------------");

    // let part_two_answer = solve_two(&input);
    // println!("Part Two Answer: {}", part_two_answer);
}

fn part_two(input: &str) -> i32 {
    0
}

fn part_one(input: &str) -> i32 {
    let num_cols = get_num_cols(&input);
    let input_arr = get_input_arr(&input);
    let mut part_numbers: BTreeMap<i32, i32> = BTreeMap::new();

    for (i, c) in input_arr.iter().enumerate() {
        // if c is not special char, continue
        if c.is_numeric() {
            continue;
        } else if c == &'.' {
            continue;
        }

        // if c is special char, check surrounding for digit

        let dir_indexes = get_dir_indexes(i as i32, num_cols as i32);

        for dir_index in dir_indexes {
            if valid_cell(&input_arr, dir_index) == false {
                continue;
            }

            if input_arr[dir_index as usize].is_numeric() {
                let found_number = scan_number(
                    &input_arr,
                    dir_index,
                    num_cols as usize,
                );

                for (&key, &value) in &found_number {
                    part_numbers.insert(key, value);
                }
            }
        }
    }

    let mut part_numbers_summed = 0;

    for (&_, &value) in &part_numbers {
        part_numbers_summed += value;
    }

    part_numbers_summed
}

fn get_input_arr(input: &str) -> Vec<char> {
    let input_no_newlies = input.to_string().replace(&*"\n", &*"");
    input_no_newlies.chars().collect::<Vec<char>>()
}

fn get_num_cols(input: &str, ) -> i32 {
    let first_row_opt = input.split('\n').nth(0);

    let first_row = match first_row_opt {
        Some(cols) => cols,
        None => {
            println!("number of columns is 0");
            return 0;
        }
    };

    first_row.len() as i32
}

fn get_dir_indexes(i: i32, num_cols: i32) -> Vec<i32>{
    vec! [
        i - num_cols,
        i - num_cols + 1,
        i + 1,
        i + num_cols + 1,
        i + num_cols as i32,
        i + num_cols - 1,
        i - 1,
        i - (num_cols + 1),
    ]
}

fn scan_number(input_arr: &Vec<char>, i: i32, num_cols: usize) -> BTreeMap<i32, i32> {
    let mut ret_arr: Vec<char> = Vec::new();
    let mut current_char = input_arr[i as usize];

    ret_arr.push(current_char);
    let mut starting_index = i;

    // move left
    let mut left_count = 0;
    while current_char.is_numeric() {
        left_count += 1;

        if valid_cell(&input_arr, i - left_count) == false {
            break
        }

        current_char = input_arr[i as usize - left_count as usize];

        if current_char.is_numeric() == false {
            break;
        }

        ret_arr.insert(0, current_char);
        starting_index = i - left_count;

        // Check if we've reached the start of a line
        if (i - left_count) % num_cols as i32 == 0 {
            break;
        }
    }

    current_char = input_arr[i as usize];

    // move right
    let mut right_count = 0;
    while current_char.is_numeric() {
        right_count += 1;

        if valid_cell(&input_arr, i + right_count) == false {
            break;
        }

        current_char = input_arr[i as usize + right_count as usize];

        if current_char.is_numeric() == false {
            break;
        }

        ret_arr.push(current_char);
    }

    // convert vec of chars to i32
    let num_str: String = ret_arr.into_iter().collect();
    let full_number = num_str.parse().expect("Failed to parse number");

    let mut ret_val: BTreeMap<i32, i32> = BTreeMap::new();
    ret_val.insert(starting_index, full_number);
    ret_val
}

fn valid_cell(input: &Vec<char>, i: i32) -> bool {
    if i < 0 {
        return false;
    }

    if i as usize > input.len() {
        return false;
    }

    true
}

#[test]
fn test_part_one_example() {
    let example_input = fs::read_to_string("src/inputs/day_three_example.txt")
        .expect("Failed to read file");

    assert_eq!(part_one(&example_input), 4361);
}

#[test]
fn test_part_one() {
    let input = fs::read_to_string("src/inputs/day_three.txt")
        .expect("Failed to read file");

    assert_eq!(part_one(&input), 554003);
}

#[test]
fn test_part_two_example() {
    let example_input = fs::read_to_string("src/inputs/day_three_example.txt")
        .expect("Failed to read file");

    assert_eq!(part_two(&example_input), 467835);
}