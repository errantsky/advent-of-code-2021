use std::fs::File;
use std::io::{BufRead, BufReader};

/// ToDo: rewrite in a more idiomatic and functional way
/// ToDo: solve in a single pass

fn part1(numbers: Vec<usize>) -> usize {
    let mut prev = usize::MAX;
    let mut acc = 0;
    for number in numbers {
        if number > prev {
            acc += 1;
        }
        prev = number;
    }
    acc
}

fn part2(numbers: Vec<usize>) -> usize {
    let mut triple_sums = Vec::new();
    for i in 0..(numbers.len() - 3 + 1) {
        triple_sums.push(numbers[i..(i + 3)].iter().sum())
    }
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
        let path = "/Users/sep/CLionProjects/aoc-2021/src/test_files/day1_part1_sample.txt";
        let acc = part1(read_input(path));
        assert_eq!(acc, 7);
    }

    #[test]
    fn test_submission_part1() {
        let path = "/Users/sep/CLionProjects/aoc-2021/src/test_files/day1_part1_submission.txt";
        let mut acc = part1(read_input(path));
        assert_eq!(acc, 1665);
    }

    #[test]
    fn test_sample_part2() {
        let path = "/Users/sep/CLionProjects/aoc-2021/src/test_files/day1_part1_sample.txt";
        let acc = part2(read_input(path));
        assert_eq!(acc, 5);
    }

    #[test]
    fn test_submission_part2() {
        let path = "/Users/sep/CLionProjects/aoc-2021/src/test_files/day1_part1_submission.txt";
        let mut acc = part2(read_input(path));
        assert_eq!(acc, 1702);
    }
}
