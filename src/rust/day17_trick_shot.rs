use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const SAMPLE_PATH: &str = "/Users/sep/CLionProjects/aoc-2021/src/test_files/day17_sample.txt";
const SUBMISSION_PATH: &str =
    "/Users/sep/CLionProjects/aoc-2021/src/test_files/day17_submission.txt";

// fn part1(path: &str) -> usize {
//     
// }


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_sample_part1() {
        let res = part1(SAMPLE_PATH);
        assert_eq!(res, 45);
    }

    #[test]
    fn test_submission_part1() {
        let res = part1(SUBMISSION_PATH);
        assert_eq!(res, 319329);
    }

    // #[test]
    // fn test_sample_part2() {
    //     let res = part2(SAMPLE_PATH);
    //     assert_eq!(res, 288957);
    // }
    //
    // #[test]
    // fn test_submission_part2() {
    //     let res = part2(SUBMISSION_PATH);
    //     assert_eq!(res, 3515583998);
    // }
}
