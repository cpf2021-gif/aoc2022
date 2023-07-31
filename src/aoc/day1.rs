#![allow(unused)]
use crate::aoc::read;

pub fn solve_part1(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    let mut sum = 0;
    let mut max = 0;

    for line in lines {
        if line.is_empty() {
            if sum > max {
                max = sum;
            }
            sum = 0;
            continue;
        }
        sum += line.parse::<i32>().unwrap();
    }
    max
}

pub fn solve_part2(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    let mut sum = 0;
    let mut sums = vec![];
    for line in lines {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
            continue;
        }
        sum += line.parse::<i32>().unwrap();
    }

    if sum != 0 {
        sums.push(sum);
    }

    sums.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut result = 0;
    for sum in sums.iter().take(3) {
        result += sum;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day1/test.txt"), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day1/test.txt"), 45000);
    }
}
