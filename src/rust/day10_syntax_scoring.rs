use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const SAMPLE_PATH: &str = "/Users/sep/CLionProjects/aoc-2021/src/test_files/day10_sample.txt";
const SUBMISSION_PATH: &str =
    "/Users/sep/CLionProjects/aoc-2021/src/test_files/day10_submission.txt";

fn find_corruption_score(line: String) -> usize {
    let openings: HashSet<char> = HashSet::from(['(', '[', '{', '<']);
    let closings: HashSet<char> = HashSet::from([')', ']', '}', '>']);
    let pairs: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let corruption_score_map: HashMap<char, usize> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        if openings.contains(&c) {
            stack.push(c);
        } else if closings.contains(&c) {
            // check if the closing char matches the opening char on top of the stack
            let top = stack.pop().unwrap();
            if c != *pairs.get(&top).unwrap() {
                return *corruption_score_map.get(&c).unwrap();
            }
        } else {
            panic!("{}", format!("Invalid character: {}", c));
        }
    }
    0
}

fn part1(path: &str) -> usize {
    let f = File::open(path).unwrap();
    let lines = BufReader::new(f).lines();

    lines
        .map(|line_res| {
            let line = line_res.unwrap();
            find_corruption_score(line)
        })
        .sum()
}

fn find_incomplete_score(line: String) -> usize {
    let openings: HashSet<char> = HashSet::from(['(', '[', '{', '<']);
    let closings: HashSet<char> = HashSet::from([')', ']', '}', '>']);
    let incomplete_score_map: HashMap<char, usize> =
        HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        if openings.contains(&c) {
            stack.push(c);
        } else if closings.contains(&c) {
            stack.pop();
        }
    }

    stack
        .into_iter()
        .rev()
        .fold(0, |acc, c| acc * 5 + incomplete_score_map.get(&c).unwrap())
}

fn part2(path: &str) -> usize {
    let f = File::open(path).unwrap();
    let mut lines = BufReader::new(f).lines();

    let incomplete_lines: Vec<String> = lines
        .into_iter()
        .filter_map(|line| {
            let line = line.unwrap();
            if find_corruption_score(line.clone()) == 0 {
                Some(line)
            } else {
                None
            }
        })
        .collect();
    let mut incomplete_scores: Vec<usize> = incomplete_lines
        .into_iter()
        .map(find_incomplete_score)
        .collect();
    // I'm sorting primitives, so sorting stability is not needed
    incomplete_scores.sort_unstable();
    incomplete_scores[incomplete_scores.len() / 2 as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_sample_part1() {
        let res = part1(SAMPLE_PATH);
        assert_eq!(res, 26397);
    }

    #[test]
    fn test_submission_part1() {
        let res = part1(SUBMISSION_PATH);
        assert_eq!(res, 319329);
    }

    #[test]
    fn test_sample_part2() {
        let res = part2(SAMPLE_PATH);
        assert_eq!(res, 288957);
    }

    #[test]
    fn test_submission_part2() {
        let res = part2(SUBMISSION_PATH);
        assert_eq!(res, 3515583998);
    }
}
