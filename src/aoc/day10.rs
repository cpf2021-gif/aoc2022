#![allow(unused)]
use crate::aoc::read;

pub fn solve_part1(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    let mut sum = 0;

    let mut score = vec![0_i32; 2 * lines.len() + 1];
    score[0] = 1;

    let mut cur_idx: usize = 1;

    let mut target = vec![20, 60, 100, 140, 180, 220];

    for line in lines {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        match words[0] {
            "noop" => {
                score[cur_idx] = score[cur_idx - 1];
                cur_idx += 1;
            }
            "addx" => {
                score[cur_idx] = score[cur_idx - 1];
                cur_idx += 1;
                score[cur_idx] = score[cur_idx - 1] + words[1].parse::<i32>().unwrap();
                cur_idx += 1;
            }
            _ => panic!("Unknown instruction"),
        }
    }

    for i in target {
        println!("{}: {}", i, score[i - 1]);
        sum += score[i - 1] * i as i32;
    }
    sum
}

pub fn solve_part2(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    let mut sum = 0;

    let mut map = vec![vec![' '; 40]; 6];

    let mut score = vec![0_i32; 2 * lines.len() + 1];
    score[0] = 1;

    let mut cur_idx: usize = 1;

    for line in lines {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        match words[0] {
            "noop" => {
                score[cur_idx] = score[cur_idx - 1];
                cur_idx += 1;
            }
            "addx" => {
                score[cur_idx] = score[cur_idx - 1];
                cur_idx += 1;
                score[cur_idx] = score[cur_idx - 1] + words[1].parse::<i32>().unwrap();
                cur_idx += 1;
            }
            _ => panic!("Unknown instruction"),
        }
    }

    for i in 0..6 {
        for j in 0..40 {
            let idx = i * 40 + j;
            if (score[idx] - j as i32).abs() > 1 {
                map[i][j] = '.';
            } else {
                map[i][j] = '#';
            }
        }
    }

    for (_, item) in map.iter().enumerate() {
        let string = item.iter().collect::<String>();
        println!("{}", string);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day10/test.txt"), 13140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day10/test.txt"), 0);
    }
}
