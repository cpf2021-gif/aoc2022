#![allow(unused)]
use crate::aoc::read;

fn contain_other(l: &str, r: &str) -> i32 {
    let (l1, l2) = get_num(l);
    let (r1, r2) = get_num(r);
    if l1 <= r1 && r2 <= l2 || r1 <= l1 && l2 <= r2 {
        1
    } else {
        0
    }
}

fn overlep_other(l: &str, r: &str) -> i32 {
    let (l1, l2) = get_num(l);
    let (r1, r2) = get_num(r);
    if l2 < r1 || r2 < l1 {
        0
    } else {
        1
    }
}

fn get_num(l: &str) -> (i32, i32) {
    let words: Vec<&str> = l.split('-').collect();
    (
        words[0].parse::<i32>().unwrap(),
        words[1].parse::<i32>().unwrap(),
    )
}

pub fn solve_part1(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    lines
        .iter()
        .map(|line| {
            let words: Vec<&str> = line.split(',').collect();
            (words[0], words[1])
        })
        .fold(0, |res, (l, r)| res + contain_other(l, r))
}

pub fn solve_part2(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    lines
        .iter()
        .map(|line| {
            let words: Vec<&str> = line.split(',').collect();
            (words[0], words[1])
        })
        .fold(0, |res, (l, r)| res + overlep_other(l, r))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day4/test.txt"), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day4/test.txt"), 4);
    }
}
