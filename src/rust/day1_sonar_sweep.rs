use std::fs::File;
use std::io::{BufRead, BufReader};

const SAMPLE_PATH: &str = "/Users/sep/CLionProjects/aoc-2021/src/test_files/day1_part1_sample.txt";
const SUBMISSION_PATH: &str =
    "/Users/sep/CLionProjects/aoc-2021/src/test_files/day1_part1_submission.txt";

fn part1(numbers: Vec<usize>) -> usize {
    let mut ni = numbers.into_iter();
    let mut prev = ni.next().unwrap();
    ni.fold(0, move |acc, n| {
        if prev < n {
            prev = n;
            acc + 1
        } else {
            prev = n;
            acc
        }
    })
}

fn part2(numbers: Vec<usize>) -> usize {
    let triple_sums = (0..(numbers.len() - 3 + 1))
        .map(move |i| numbers[i..(i + 3)].iter().sum())
        .collect();
    part1(triple_sums)
}

fn read_input(path: &str) -> Vec<usize> {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    lines
        .map(|s| s.unwrap().parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test_sample_part1() {
        let acc = part1(read_input(SAMPLE_PATH));
        assert_eq!(acc, 7);
    }

    #[test]
    fn test_submission_part1() {
        let mut acc = part1(read_input(SUBMISSION_PATH));
        assert_eq!(acc, 1665);
    }

    #[test]
    fn test_sample_part2() {
        let acc = part2(read_input(SAMPLE_PATH));
        assert_eq!(acc, 5);
    }

    #[test]
    fn test_submission_part2() {
        let mut acc = part2(read_input(SUBMISSION_PATH));
        assert_eq!(acc, 1702);
    }
}
