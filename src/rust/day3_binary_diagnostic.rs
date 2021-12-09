// use std::collections::HashSet;
// use std::fs::{read_to_string, File};
// use std::io::{BufRead, BufReader};
//
// const SAMPLE_PATH: &str = "/Users/sep/CLionProjects/aoc-2021/src/test_files/day3_sample.txt";
// const SUBMISSION_PATH: &str =
//     "/Users/sep/CLionProjects/aoc-2021/src/test_files/day3_submission.txt";
//
// fn find_counts(lines: &Vec<&str>) -> Vec<usize> {
//     let number_len = lines[0].len();
//     lines.fold(vec![0; number_len], |counts, number| {
//         let mut vec = counts.clone();
//         for (idx, bit) in number.chars().enumerate() {
//             if bit == '1' {
//                 vec[idx] += 1;
//             }
//         }
//         vec
//     })
// }
//
// fn binary_to_decimal(bin_vec: &Vec<bool>) -> usize {
//     bin_vec.iter().rev().enumerate().fold(0, |sum, (idx, bit)| {
//         if *bit {
//             sum + 2_usize.pow(idx as u32)
//         } else {
//             sum
//         }
//     })
// }
//
// fn part1(path: &str) -> usize {
//     let s = read_to_string(path).unwrap();
//     let line_count = s.lines().count();
//
//     let counts: Vec<usize> = find_counts(&s.lines().collect());
//     let gamma_bits: Vec<bool> = counts
//         .iter()
//         .enumerate()
//         .map(|(idx, bit_count)| {
//             if *bit_count > line_count / 2 {
//                 true
//             } else {
//                 false
//             }
//         })
//         .collect();
//     let epsilon_bits: Vec<bool> = gamma_bits.clone().iter().map(|bit| !*bit).collect();
//
//     let gamma: usize = binary_to_decimal(&gamma_bits);
//     let epsilon: usize = binary_to_decimal(&epsilon_bits);
//     gamma * epsilon
// }
//
// fn part2(path: &str) -> usize {
//     2
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs::read_to_string;
//
//     #[test]
//     fn test_sample_part1() {
//         let res = part1(SAMPLE_PATH);
//         assert_eq!(res, 198);
//     }
//
//     #[test]
//     fn test_submission_part1() {
//         let res = part1(SUBMISSION_PATH);
//         // println!("Day 3, Part 1: {}", res);
//         assert_eq!(res, 2583164);
//     }
//
//     #[test]
//     fn test_sample_part2() {
//         let res = part2(SAMPLE_PATH);
//         assert_eq!(res, 230);
//     }
//
//     #[test]
//     fn test_submission_part2() {
//         let mut res = part2(SUBMISSION_PATH);
//         assert_eq!(res, 2784375);
//     }
// }
