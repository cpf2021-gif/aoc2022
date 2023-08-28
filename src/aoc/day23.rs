#![allow(unused)]
use std::collections::HashMap;

use crate::aoc::read;

pub fn solve_part1(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    0
}

pub fn solve_part2(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day23/test.txt"), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day1/test.txt"), 0);
    }
}
