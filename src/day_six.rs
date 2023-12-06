use std::fs;

pub fn main() {
    // println!("------------ Part One Example ------------");
    let example_input = fs::read_to_string("src/inputs/day_six_example.txt")
        .expect("Failed to read file");

    let example_answer = part_one(&example_input);
    println!("Example answer: {}", example_answer);

    println!("------------ Part One ------------");

    let input = fs::read_to_string("src/inputs/day_six.txt")
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

#[derive(Debug)]
struct Race {
    duration: i64,
    distance: i64,
}

fn part_two(input: &str) -> i64 {
    let time = input
        .lines()
        .collect::<Vec<&str>>()[0]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    let dist_to_win = input
        .lines()
        .collect::<Vec<&str>>()[1]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    println!("{}", time);
    println!("{}", dist_to_win);

    part_one(&format!("Time: {}\nDistance: {}", time, dist_to_win))
}

fn get_distance(hold_time: i64, race_duration: i64) -> i64 {
    let run_time = race_duration - hold_time;

    let mut distance = hold_time * run_time;

    distance
}

fn part_one(input: &str) -> i64 {
    let races = get_races(&input);
    let mut num_wins: Vec<i64> = Vec::new();

    for race in races {
        let mut wins: i64 = 0;
        for i in 0..race.duration + 1 {
            let distance = get_distance(i, race.duration);

            if distance > race.distance {
                wins += 1;
            }

        }

        num_wins.push(wins);
    }

    let mut wins_multiplied = 1;

    for win in num_wins {
        wins_multiplied *= win;
    }

    wins_multiplied
}

fn get_races(input: &str) -> Vec<Race> {
    let times = input
        .lines()
        .collect::<Vec<&str>>()[0]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>();

    let distances = input
        .lines()
        .collect::<Vec<&str>>()[1]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>();

    let mut races: Vec<Race> = Vec::new();
    for i in 0..times.len() {
        races.push(Race{
            duration: times[i].parse::<i64>().unwrap(),
            distance: distances[i].parse::<i64>().unwrap(),
        })
    }

    races
}

#[test]
fn test_part_one_example() {
    let example_input = fs::read_to_string("src/inputs/day_six_example.txt")
        .expect("Failed to read file");

    assert_eq!(part_one(&example_input), 288);
}

#[test]
fn test_part_two_example() {
    let example_input = fs::read_to_string("src/inputs/day_six_example.txt")
        .expect("Failed to read file");

    assert_eq!(part_two(&example_input), 71503);
}