use std::{fs, collections::HashMap};

pub fn main() {
    println!("Hello from day two!");

    println!("------------ Example ------------");
    let example_input = fs::read_to_string("src/inputs/day_two_example.txt")
        .expect("Failed to read file");

    let example_answer = solve(example_input);
    println!("Example answer: {}", example_answer);

    println!("------------ Part One ------------");

    let input = fs::read_to_string("src/inputs/day_two.txt")
        .expect("Failed to read file");

    let answer = solve(input);
    println!("Answer: {}", answer);
}

fn solve(input: String) -> i32 {
    let mut ret_val = 0;
    // set bag contents
    let bag_contents = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);

    // loop through each line of input
    for line in input.split('\n') {
        let game_number = line
            .split(' ')
            .nth(1)
            .unwrap()
            .split(':')
            .nth(0)
            .unwrap();

        // check if any number of dice shown is greater than
        // the number of dice in the bag
        let possible_res = game_possible(bag_contents.clone(), &line);

        let possible = match possible_res {
            Ok(possible) => possible,
            Err(_) => {
                println!("Error checking if game is possible");
                return 0;
            },
        };

        if possible {
            ret_val = ret_val + game_number.parse::<i32>().unwrap();
        }
    }

    ret_val
}

fn game_possible(bag_contents: HashMap<String, i32>, line: &str) -> Result<bool, String> {
    // check if the number of dice shown is greater than
    // the number of dice in the bag

    // format line, removing Game prefix

    let game_trimmed_arr: Vec<&str> = line.split(' ').collect();
    let game_trimmed_line = game_trimmed_arr[2..].join(" ");

    // split game into separate draws from the bag
    for draw in game_trimmed_line.split(';') {

        let trimmed_draw = draw.trim_start();
        let trimmed_draw = trimmed_draw.trim_end();

        let draw_is_possible_res = draw_possible(
            bag_contents.clone(),
            trimmed_draw,
        );

        let draw_is_possible = match draw_is_possible_res {
            Ok(draw_is_possible) => draw_is_possible,
            Err(_) => {
                return Err("Error checking if draw is possible".to_string());
            },
        };

        if draw_is_possible != true {
            return Ok(false);
        }
    }

    Ok(true)
}

fn draw_possible(bag_contents: HashMap<String, i32>, draw: &str) -> Result<bool, String> {
    for dice in draw.split(',') {
        let dice = dice.trim_start();
        let dice = dice.trim_end();

        let count_res = dice.split(' ')
            .nth(0)
            .unwrap()
            .parse::<i32>();

        let count = match count_res {
            Ok(count) => count,
            Err(_) => {
                println!("Error parsing count: {}", count_res.clone().unwrap_err());
                return Err(count_res.unwrap_err().to_string());
            },
        };

        let color = dice.split(' ').nth(1).unwrap();

        let bag_count = match bag_contents.get(color) {
            Some(count) => count,
            None => {
                println!("Error: color {} not found in bag", color);
                return Err(count_res.unwrap_err().to_string());
            },
        };

        if &count > bag_count {
            return Ok(false);
        }
    }

    Ok(true)
}

// ====== Possible Example Games ======
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

// ====== Impossible Example Games ======
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red

// ====== Example Answer ======
// 8

#[test]
fn test_draw_possible_success() {
    let bag_contents = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);

    // Game 1
    assert_eq!(draw_possible(bag_contents.clone(), "3 blue, 4 red"), Ok(true));
    assert_eq!(draw_possible(bag_contents.clone(), "1 blue, 2 green, 6 blue"), Ok(true));
    assert_eq!(draw_possible(bag_contents.clone(), "2 green"), Ok(true));
}

#[test]
fn test_draw_possible_failure() {
    let bag_contents = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);

    assert_eq!(draw_possible(bag_contents, "8 green, 6 blue, 20 red"), Ok(false))
}

#[test]
fn test_game_possible_success() {
    let bag_contents = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);

    assert_eq!(
        game_possible(bag_contents.clone(), "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
        Ok(true)
    );

    assert_eq!(
        game_possible(bag_contents.clone(), "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
        Ok(true)
    );

    assert_eq!(
        game_possible(bag_contents.clone(), "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        Ok(true)
    );
}

#[test]
fn test_game_possible_failure() {
    let bag_contents = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);

    assert_eq!(
        game_possible(bag_contents.clone(), "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
        Ok(false)
    );

    assert_eq!(
        game_possible(bag_contents.clone(), "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
        Ok(false)
    );
}

#[test]
fn test_solve_example() {
    let example_input = fs::read_to_string("src/inputs/day_two_example.txt")
        .expect("Failed to read file");

    assert_eq!(solve(example_input), 8);
}

#[test]
fn test_part_one() {
    let input = fs::read_to_string("src/inputs/day_two.txt")
        .expect("Failed to read file");

    assert_eq!(solve(input), 2406);
}