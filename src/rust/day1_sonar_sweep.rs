use std::fs::File;
use std::io::{BufRead, BufReader};

/// ToDo: rewrite in a more idiomatic and functional way
/// ToDo: solve in a single pass

fn part1(buffered: BufReader<File>) -> usize {
    let mut prev = usize::MAX;
    let mut acc = 0;
    for l in buffered.lines() {
        let number = l.unwrap().parse::<usize>().unwrap();
        if number > prev {
            acc += 1;
        }
        prev = number;
    }
    acc
}

fn part2(buffered: BufReader<File>) -> usize {
    let mut lines = buffered.lines();
    let mut sums: Vec<usize> = Vec::new();
    let mut sums_vec = Vec::new();
    let mut current_sum = 0;
    let mut cnt = 0;
    while let Some(Ok(s)) = lines.next() {
        let number = s.parse::<usize>().unwrap();
        sums_vec.push(number);
        if sums_vec.len() == 3 {
            sums.push(sums_vec.iter().sum());
            sums_vec.remove(0);
        }
    }

    let mut acc = 0;
    for i in 1..(sums.len()) {
        if sums[i] > sums[i - 1] {
            acc += 1;
        }
    }
    acc
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test_sample_part1() {
        let path = "/Users/sep/CLionProjects/aoc-2021/src/test_files/day1_part1_sample.txt";
        let buffered = BufReader::new(File::open(path).unwrap());
        let acc = part1(buffered);
        assert_eq!(acc, 7);
    }

    #[test]
    fn test_submission_part1() {
        let path = "/Users/sep/CLionProjects/aoc-2021/src/test_files/day1_part1_submission.txt";
        let buffered = BufReader::new(File::open(path).unwrap());
        let mut acc = part1(buffered);
        println!("Day 1, Part 1: {}", acc);
    }

    #[test]
    fn test_sample_part2() {
        let path = "/Users/sep/CLionProjects/aoc-2021/src/test_files/day1_part1_sample.txt";
        let buffered = BufReader::new(File::open(path).unwrap());
        let acc = part2(buffered);
        assert_eq!(acc, 5);
    }

    #[test]
    fn test_submission_part2() {
        let path = "/Users/sep/CLionProjects/aoc-2021/src/test_files/day1_part1_submission.txt";
        let buffered = BufReader::new(File::open(path).unwrap());
        let mut acc = part2(buffered);
        println!("Day 1, Part 2: {}", acc);
    }

}


