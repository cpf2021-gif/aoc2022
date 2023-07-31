#![allow(unused)]
use crate::aoc::read;

fn get_char_score(ch: char) -> i32 {
    match ch {
        'a'..='z' => ch as i32 - 'a' as i32 + 1,
        'A'..='Z' => ch as i32 - 'A' as i32 + 1 + 26,
        _ => panic!("Invalid character"),
    }
}

pub fn solve_part1(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    let mut result = 0;
    for line in lines {
        let len = line.len() / 2;
        let l = &line[..len];
        let r = &line[len..];
        for ch in l.chars() {
            let score = get_char_score(ch);
            if r.contains(ch) {
                result += score;
                break;
            }
        }
    }
    result
}

pub fn solve_part2(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    let mut chars = vec![];
    ('a'..='z').for_each(|ch| chars.push(ch));
    ('A'..='Z').for_each(|ch| chars.push(ch));
    lines
        .chunks(3)
        .map(|chunk| {
            (
                chunk.get(0).unwrap(),
                chunk.get(1).unwrap(),
                chunk.get(2).unwrap(),
            )
        })
        .fold(0, |res, (a, b, c)| {
            res + chars
                .iter()
                .filter(|&&ch| a.contains(ch) && b.contains(ch) && c.contains(ch))
                .fold(0, |add, ch| add + get_char_score(*ch))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day3/test.txt"), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day3/test.txt"), 70);
    }
}
