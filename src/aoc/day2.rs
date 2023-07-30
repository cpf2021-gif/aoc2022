use std::collections::HashMap;

use crate::aoc::read;

pub fn solve_part1(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    let mut result = 0;
    let mut map = HashMap::new();
    map.insert("A X".to_string(), 4);
    map.insert("A Y".to_string(), 8);
    map.insert("A Z".to_string(), 3);
    map.insert("B X".to_string(), 1);
    map.insert("B Y".to_string(), 5);
    map.insert("B Z".to_string(), 9);
    map.insert("C X".to_string(), 7);
    map.insert("C Y".to_string(), 2);
    map.insert("C Z".to_string(), 6);
    for line in lines {
        let num = map.get(&line).unwrap();
        result += num;
    }
    result
}

pub fn solve_part2(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    let mut result = 0;
    let mut map = HashMap::new();
    map.insert("A X".to_string(), 3);
    map.insert("A Y".to_string(), 4);
    map.insert("A Z".to_string(), 8);
    map.insert("B X".to_string(), 1);
    map.insert("B Y".to_string(), 5);
    map.insert("B Z".to_string(), 9);
    map.insert("C X".to_string(), 2);
    map.insert("C Y".to_string(), 6);
    map.insert("C Z".to_string(), 7);
    for line in lines {
        let num = map.get(&line).unwrap();
        result += num;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day2/test.txt"), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day2/test.txt"), 12);
    }
}
