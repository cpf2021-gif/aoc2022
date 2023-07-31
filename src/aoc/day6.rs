#![allow(unused)]
use std::collections::HashMap;

use crate::aoc::read;

fn find_firt_difference(line: String, num: usize) -> i32 {
    let len = line.len();
    let mut idx = 0;
    let mut set = HashMap::new();
    for c in line.chars().take(num) {
        let v = set.get(&c);
        match v {
            Some(_) => set.insert(c, v.unwrap() + 1),
            None => set.insert(c, 1),
        };
        if set.len() == num {
            return idx as i32 + num as i32;
        }
    }

    for c in line.chars().skip(num) {
        let v = set.get(&line.chars().nth(idx).unwrap());
        if v.unwrap() == &1 {
            set.remove(&line.chars().nth(idx).unwrap());
        } else {
            set.insert(line.chars().nth(idx).unwrap(), v.unwrap() - 1);
        }
        idx += 1;
        let v = set.get(&c);
        match v {
            Some(_) => set.insert(c, v.unwrap() + 1),
            None => set.insert(c, 1),
        };
        if set.len() == num {
            return idx as i32 + num as i32;
        }
    }
    idx as i32 + num as i32
}

pub fn solve_part1(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    for line in lines {
        println!("{}", find_firt_difference(line, 4));
    }
    0
}

pub fn solve_part2(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    for line in lines {
        println!("{}", find_firt_difference(line, 14));
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day6/test1.txt"), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day6/test2.txt"), 0);
    }
}
