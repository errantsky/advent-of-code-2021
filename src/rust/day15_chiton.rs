
const SAMPLE_PATH: &str = "/Users/sep/CLionProjects/aoc-2021/src/test_files/day15_sample.txt";
const SUBMISSION_PATH: &str =
    "/Users/sep/CLionProjects/aoc-2021/src/test_files/day15_submission.txt";

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
        assert_eq!(res, 40);
    }

    #[test]
    fn test_submission_part1() {
        let res = part1(SUBMISSION_PATH);
        assert_eq!(res, 619);
    }

    // #[test]
    // fn test_sample_part2() {
    //     let res = part2(SAMPLE_PATH);
    //     assert_eq!(res, 315);
    // }
    //
    // #[test]
    // fn test_submission_part2() {
    //     let res = part2(SUBMISSION_PATH);
    //     assert_eq!(res, 2922);
    // }
}
