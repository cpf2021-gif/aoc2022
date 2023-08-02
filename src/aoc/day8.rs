#![allow(unused)]
use crate::aoc::read;

pub fn solve_part1(filename: &str) -> i32 {
    let lines = read::read_lines(filename);

    let mut result = 0;

    let m = lines.len();
    let n = lines[0].len();

    let mut map = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        map.push(row);
    }

    let mut grid = vec![vec![false; n]; m];
    // 边缘都是true
    for i in 0..m {
        for j in 0..n {
            if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                grid[i][j] = true;
            }
        }
    }

    // 从上到下看
    for j in 1..n - 1 {
        let mut max = map[0][j];
        for i in 1..m - 1 {
            if map[i - 1][j] > max {
                max = map[i - 1][j];
            }
            if max < map[i][j] {
                grid[i][j] = true;
            }
        }
    }

    // 从下到上看
    for j in 1..n - 1 {
        let mut max = map[m - 1][j];
        for i in (1..m - 1).rev() {
            if map[i + 1][j] > max {
                max = map[i + 1][j];
            }
            if max < map[i][j] {
                grid[i][j] = true;
            }
        }
    }

    // 从左到右看
    for i in 1..m - 1 {
        let mut max = map[i][0];
        for j in 1..n - 1 {
            if map[i][j - 1] > max {
                max = map[i][j - 1];
            }
            if max < map[i][j] {
                grid[i][j] = true;
            }
        }
    }

    // 从右到左看
    for i in 1..m - 1 {
        let mut max = map[i][n - 1];
        for j in (1..n - 1).rev() {
            if map[i][j + 1] > max {
                max = map[i][j + 1];
            }
            if max < map[i][j] {
                grid[i][j] = true;
            }
        }
    }

    // 计算true的个数
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] {
                result += 1;
            }
        }
    }

    result
}

pub fn solve_part2(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    let mut result = 0;

    let m = lines.len();
    let n = lines[0].len();

    let mut map = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        map.push(row);
    }

    let dir = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    for i in 1..m - 1 {
        for j in 1..n - 1 {
            let mut score = 1;
            for step in &dir {
                let mut walk = (i as i32, j as i32);
                let my_height = map[i][j];
                walk.0 += step.0;
                walk.1 += step.1;
                let mut count = 0;
                while walk.0 >= 0 && walk.0 < m as i32 && walk.1 >= 0 && walk.1 < n as i32 {
                    count += 1;
                    if map[walk.0 as usize][walk.1 as usize] >= my_height {
                        break;
                    }
                    walk.0 += step.0;
                    walk.1 += step.1;
                }
                score *= count;
            }
            result = result.max(score);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day8/test.txt"), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day8/test.txt"), 8);
    }
}
