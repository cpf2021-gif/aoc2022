#![allow(unused)]
use crate::aoc::read;

fn decrypt(data: &[(i64, i64)], rounds: usize, key: i64) -> i64 {
    let data = data
        .iter()
        .map(|(index, value)| (*index, value * key))
        .collect::<Vec<_>>();

    let mut result = data.clone();

    let len = result.len() as i64 - 1;

    for _ in 0..rounds {
        for d in &data {
            let pos = result.iter().position(|n| n == d).unwrap() as i64;
            let mut new_pos = (pos + d.1) % len;

            if new_pos < 0 {
                new_pos += len;
            }

            if new_pos >= len {
                new_pos -= len;
            }

            let val = result.remove(pos as usize);
            result.insert(new_pos as usize, val);
        }
    }

    let zero = result.iter().position(|p| p.1 == 0).unwrap();

    result[(zero + 1000) % result.len()].1
        + result[(zero + 2000) % result.len()].1
        + result[(zero + 3000) % result.len()].1
}

pub fn solve_part1(filename: &str) -> i64 {
    let lines = read::read_lines(filename);
    let mut data = lines
        .iter()
        .enumerate()
        .map(|n| (n.0 as i64, n.1.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();
    decrypt(&data, 1, 1)
}

pub fn solve_part2(filename: &str) -> i64 {
    let lines = read::read_lines(filename);
    let mut data = lines
        .iter()
        .enumerate()
        .map(|n| (n.0 as i64, n.1.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();
    decrypt(&data, 10, 811589153)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day20/test.txt"), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day20/test.txt"), 1623178306);
    }
}
