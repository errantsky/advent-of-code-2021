use std::collections::{HashMap, HashSet};
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};

const SAMPLE_PATH: &str = "/Users/sep/CLionProjects/aoc-2021/src/test_files/day9_sample.txt";
const SUBMISSION_PATH: &str =
    "/Users/sep/CLionProjects/aoc-2021/src/test_files/day9_submission.txt";

fn input_to_grid(path: &str) -> Vec<Vec<u32>> {
    let f = File::open(path).unwrap();
    let lines = BufReader::new(f).lines();
    lines
        .map(|l| {
            let l = l.unwrap();
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn valid_neighbors(x: usize, y: usize, n_rows: usize, n_cols: usize) -> Vec<(usize, usize)> {
    let candidates = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    candidates
        .into_iter()
        .filter_map(|(dx, dy)| {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            if new_x >= 0 && new_x < n_rows as i32 && new_y >= 0 && new_y < n_cols as i32 {
                Some((new_x as usize, new_y as usize))
            } else {
                None
            }
        })
        .collect()
}

fn is_local_minima(
    grid: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    neighbors_coords: Vec<(usize, usize)>,
) -> bool {
    neighbors_coords
        .into_iter()
        .map(|(nx, ny)| grid[nx][ny] > grid[x][y])
        .all(|b| b == true)
}

fn part1(grid: Vec<Vec<u32>>) -> usize {
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let mut sum = 0;
    for i in 0..n_rows {
        for j in 0..n_cols {
            let neighbors_coords = valid_neighbors(i, j, n_rows, n_cols);
            if is_local_minima(&grid, i, j, neighbors_coords) {
                sum += grid[i][j] + 1
            }
        }
    }
    sum as usize
}

fn find_basin_sizes(grid: &Vec<Vec<u32>>, low_points: Vec<(usize, usize)>) -> usize {
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let mut basins: Vec<HashSet<(usize, usize)>> = Vec::new();
    for (lx, ly) in low_points {
        let mut basin_members: HashSet<(usize, usize)> = HashSet::from([(lx, ly)]);
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut to_visit: Vec<(usize, usize)> = vec![(lx, ly)];
        while !to_visit.is_empty() {
            let cell = to_visit.pop().unwrap();
            visited.insert(cell);
            let vn: Vec<(usize, usize)> = valid_neighbors(cell.0, cell.1, n_rows, n_cols);
            let new_members: Vec<(usize, usize)> = vn
                .into_iter()
                .filter(|(nx, ny)| {
                    !visited.contains(&(*nx, *ny))
                        && grid[*nx][*ny] != 9
                        && grid[*nx][*ny] > grid[cell.0][cell.1]
                })
                .collect();
            basin_members.extend(&new_members);
            to_visit.extend(new_members);
        }
        basins.push(basin_members);
    }
    let mut basins_lengths = basins.iter()
        .map(|v| v.len()).collect::<Vec<usize>>();
    basins_lengths.sort();

    basins_lengths
        .into_iter()
        .rev()
        .take(3)
        .reduce(|left, right| left * right)
        .unwrap()
}

fn part2(grid: Vec<Vec<u32>>) -> usize {
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    for i in 0..n_rows {
        for j in 0..n_cols {
            let neighbors_coords = valid_neighbors(i, j, n_rows, n_cols);
            if is_local_minima(&grid, i, j, neighbors_coords) {
                low_points.push((i, j));
            }
        }
    }

    find_basin_sizes(&grid, low_points)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_sample_part1() {
        let input = input_to_grid(SAMPLE_PATH);
        let res = part1(input);
        assert_eq!(res, 15);
    }

    #[test]
    fn test_submission_part1() {
        let input = input_to_grid(SUBMISSION_PATH);
        let res = part1(input);
        assert_eq!(res, 475);
    }

    #[test]
    fn test_sample_part2() {
        let input = input_to_grid(SAMPLE_PATH);
        let res = part2(input);
        assert_eq!(res, 1134);
    }

    #[test]
    fn test_submission_part2() {
        let input = input_to_grid(SUBMISSION_PATH);
        let mut res = part2(input);
        assert_eq!(res, 1092012);
    }
}
