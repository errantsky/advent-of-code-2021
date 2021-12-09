use std::fs::{read_to_string, File};

const SAMPLE_PATH: &str = "/Users/sep/CLionProjects/aoc-2021/src/test_files/day6_sample.txt";
const SUBMISSION_PATH: &str =
    "/Users/sep/CLionProjects/aoc-2021/src/test_files/day6_submission.txt";

// ToDo: Accessing and mutating variables outside a map
pub fn part1(path: &str, days: usize) -> usize {
    let s = read_to_string(path).unwrap();
    let inp = s.lines().next().unwrap();
    let mut fish_state: Vec<u8> = inp.split(",").map(|x| x.parse::<u8>().unwrap()).collect();
    let mut new_fish: Vec<u8> = Vec::new();
    for _ in 0..days {
        for i in 0..fish_state.len() {
            if fish_state[i] == 0 {
                new_fish.push(8);
                fish_state[i] = 6;
            } else {
                fish_state[i] -= 1;
            }
        }
        fish_state.append(&mut new_fish);
    }

    fish_state.len()
}

// fn part2(path: &str) -> usize {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_sample_part1() {
        let res = part1(SAMPLE_PATH, 80);
        assert_eq!(res, 5934);
    }

    #[test]
    fn test_submission_part1() {
        let res = part1(SUBMISSION_PATH, 80);
        assert_eq!(res, 387413);
    }

    #[test]
    fn test_sample_part2() {
        let res = part1(SAMPLE_PATH, 256);
        assert_eq!(res, 26984457539);
    }

    #[test]
    fn test_submission_part2() {
        let mut res = part1(SUBMISSION_PATH, 256);
        assert_eq!(res, 7075);
        // println!("day 6, Part 2: {}", res);
    }
}
