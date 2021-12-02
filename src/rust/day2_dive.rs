use std::fs::File;
use std::io::{BufRead, BufReader};

const SAMPLE_PATH: &str = "/Users/sep/CLionProjects/aoc-2021/src/test_files/day2_sample.txt";
const SUBMISSION_PATH: &str =
    "/Users/sep/CLionProjects/aoc-2021/src/test_files/day2_submission.txt";

fn part1(path: &str) -> usize {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    let (p, d) = lines.fold((0, 0), |acc, l| {
        let (horizontal_pos, depth) = acc;
        let s = l.unwrap();
        let splits: Vec<&str> = s.split(' ').collect();
        let cmd = splits[0];
        let val = splits[1].parse::<usize>().unwrap();
        match cmd {
            "forward" => (horizontal_pos + val, depth),
            "up" => (horizontal_pos, depth - val),
            "down" => (horizontal_pos, depth + val),
            _ => panic!("Bad command!"),
        }
    });
    p * d
}

fn part2(path: &str) -> usize {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    let (hpos, aim, depth) = lines.fold((0, 0, 0), |acc, l| {
        let (horizontal_pos, aim, depth) = acc;
        let s = l.unwrap();
        let splits: Vec<&str> = s.split(' ').collect();
        let cmd = splits[0];
        let val = splits[1].parse::<usize>().unwrap();
        match cmd {
            "forward" => (horizontal_pos + val, aim, depth + val * aim),
            "up" => (horizontal_pos, aim - val, depth),
            "down" => (horizontal_pos, aim + val, depth),
            _ => panic!("Bad command!"),
        }
    });
    hpos * depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test_sample_part1() {
        let res = part1(SAMPLE_PATH);
        assert_eq!(res, 150);
    }

    #[test]
    fn test_submission_part1() {
        let res = part1(SUBMISSION_PATH);
        println!("Day 2, Part 1: {}", res);
    }

    #[test]
    fn test_sample_part2() {
        let res = part2(SAMPLE_PATH);
        assert_eq!(res, 900);
    }

    #[test]
    fn test_submission_part2() {
        let mut res = part2(SUBMISSION_PATH);
        println!("Day 2, Part 2: {}", res);
    }
}
