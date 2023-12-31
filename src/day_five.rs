use std::fs;

pub fn main() {
    println!("------------ Part One Example ------------");
    let example_input = fs::read_to_string("src/inputs/day_five_example.txt")
        .expect("Failed to read file");

    let example_answer = part_one(&example_input);
    println!("Example answer: {}", example_answer);

    println!("------------ Part One ------------");

    let input = fs::read_to_string("src/inputs/day_five.txt")
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
struct MapEntry {
    dest: i64,
    src: i64,
    range: i64,
}

fn part_two(input: &str) -> i64 {
    let seeds = &get_seeds(&input);
    let maps = get_maps(&input);

    let mut lowest = std::f64::INFINITY;

    for (i, seed) in seeds.iter().enumerate() {
        if i % 2 == 1 {
            continue;
        }

        println!("seed i: {} of {}", i, seeds.len());

        let range = seeds[i + 1];

        for j in 0..range {
            let expanded_seed = seed + j;
            let destination = get_seed_destination(&expanded_seed, &maps);

            if destination < lowest as i64 {
                lowest = destination as f64;
            };
        }
    }

    // return lowest value in destinations
    lowest as i64

}

fn part_one(input: &str) -> i64 {
    // get vec of seeds
    let seeds = get_seeds(&input);

    // get vec of maps
    let maps = get_maps(&input);

    let mut lowest = std::f64::INFINITY;

    for seed in seeds {
        let destination = get_seed_destination(&seed, &maps);

        if destination < lowest as i64 {
            lowest = destination as f64;
        };
    }

    // return lowest value in destinations
    lowest as i64
}

fn get_seeds(input: &str) -> Vec<i64> {
    let seed_strs = input
        .lines()
        .collect::<Vec<&str>>()[0]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>();

    let mut seeds: Vec<i64> = Vec::new();

    for str in seed_strs {
        let seed = match str.parse::<i64>() {
            Ok(n) => n,
            Err(_) => {
                println!("Failed to parse seed: {}", str);
                continue
            },
        };

        seeds.push(seed);
    }

    seeds
}

fn get_maps(input: &str) -> Vec<Vec<MapEntry>> {
    let mut maps: Vec<Vec<MapEntry>> = Vec::new();
    for (i, map) in input.split("\n\n").enumerate() {
        if i == 0 {
            // skip seed line
            continue;
        }
        maps.push(parse_map(map));
    }

    maps
}

fn get_seed_destination(seed: &i64, maps: &Vec<Vec<MapEntry>>) -> i64 {
    let mut buffer = *seed;

    for map in maps.iter() {
        for map_entry in map.iter() {
            if buffer >= map_entry.src && buffer < map_entry.src + map_entry.range {
                buffer = buffer + map_entry.dest - map_entry.src;
                break;
            }

        }
    }

    buffer
}

fn parse_map(raw_map: &str) -> Vec<MapEntry> {
    let mut map_entries: Vec<MapEntry> = Vec::new();

    for (i, line) in raw_map.lines().enumerate() {
        if i == 0 {
            // skip map name line
            continue;
        }

        let dest_res = line.split(" ").collect::<Vec<&str>>()[0].parse::<i64>();
        let src_res = line.split(" ").collect::<Vec<&str>>()[1].parse::<i64>();
        let range = line.split(" ").collect::<Vec<&str>>()[2].parse::<i64>();

        let dest = match dest_res {
            Ok(n) => n,
            Err(_) => {
                println!("Failed to parse dest: {}", line);
                continue
            },
        };

        let src = match src_res {
            Ok(n) => n,
            Err(_) => {
                println!("Failed to parse src: {}", line);
                continue
            },
        };

        let range = match range {
            Ok(n) => n,
            Err(_) => {
                println!("Failed to parse range: {}", line);
                continue
            },
        };

        map_entries.push(
            MapEntry {
                dest,
                src,
                range,
            }
        );
    }

    return map_entries;
}

#[test]
fn test_part_one_example() {
    let example_input = fs::read_to_string("src/inputs/day_five_example.txt")
        .expect("Failed to read file");

    assert_eq!(part_one(&example_input), 35);
}

#[test]
fn test_part_one() {
    let input = fs::read_to_string("src/inputs/day_five.txt")
        .expect("Failed to read file");

    assert_eq!(part_one(&input), 331445006);
}

#[test]
fn test_part_two_example() {
    let example_input = fs::read_to_string("src/inputs/day_five_example.txt")
        .expect("Failed to read file");

    assert_eq!(part_two(&example_input), 46);
}

// #[test]
// fn test_part_two() {
//     let input = fs::read_to_string("src/inputs/day_five.txt")
//         .expect("Failed to read file");

//     assert_eq!(part_two(&input), 9924412);
// }