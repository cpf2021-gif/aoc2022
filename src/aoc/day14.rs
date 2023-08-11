#![allow(unused)]
use std::cmp::Ordering;
use std::collections::HashSet;

use crate::aoc::read;

pub fn solve_part1(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    let mut set = HashSet::new();
    let mut max_depth = 0;
    let mut result = 0;

    // add rock
    for line in lines {
        let datas = line.split(" -> ").collect::<Vec<_>>();
        for i in 0..datas.len() - 1 {
            let (px, py) = datas[i]
                .split_once(',')
                .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                .unwrap();
            let (lx, ly) = datas[i + 1]
                .split_once(',')
                .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                .unwrap();

            max_depth = max_depth.max(ly);
            max_depth = max_depth.max(py);

            match (px.cmp(&lx), py.cmp(&ly)) {
                (Ordering::Less, Ordering::Equal) => {
                    for x in px..lx + 1 {
                        set.insert((x, py));
                    }
                }
                (Ordering::Greater, Ordering::Equal) => {
                    for x in lx..px + 1 {
                        set.insert((x, py));
                    }
                }
                (Ordering::Equal, Ordering::Less) => {
                    for y in py..ly + 1 {
                        set.insert((px, y));
                    }
                }
                (Ordering::Equal, Ordering::Greater) => {
                    for y in ly..py + 1 {
                        set.insert((px, y));
                    }
                }
                (_, _) => {
                    panic!("Invalid input")
                }
            }
        }
    }

    // add sand
    loop {
        let mut x = 500;
        let mut y = 0;

        loop {
            let mut flag = false;

            if !set.contains(&(x, y + 1)) {
                y += 1;
                flag = true;
            } else if !set.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
                flag = true;
            } else if !set.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
                flag = true;
            }

            if y > max_depth {
                break;
            }

            if !flag {
                break;
            }
        }

        if y > max_depth {
            break;
        }

        result += 1;
        set.insert((x, y));
    }

    result
}

pub fn solve_part2(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    let mut set = HashSet::new();
    let mut max_depth = 0;
    let mut result = 0;

    // add rock
    for line in lines {
        let datas = line.split(" -> ").collect::<Vec<_>>();
        for i in 0..datas.len() - 1 {
            let (px, py) = datas[i]
                .split_once(',')
                .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                .unwrap();
            let (lx, ly) = datas[i + 1]
                .split_once(',')
                .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                .unwrap();

            max_depth = max_depth.max(ly);
            max_depth = max_depth.max(py);

            match (px.cmp(&lx), py.cmp(&ly)) {
                (Ordering::Less, Ordering::Equal) => {
                    for x in px..lx + 1 {
                        set.insert((x, py));
                    }
                }
                (Ordering::Greater, Ordering::Equal) => {
                    for x in lx..px + 1 {
                        set.insert((x, py));
                    }
                }
                (Ordering::Equal, Ordering::Less) => {
                    for y in py..ly + 1 {
                        set.insert((px, y));
                    }
                }
                (Ordering::Equal, Ordering::Greater) => {
                    for y in ly..py + 1 {
                        set.insert((px, y));
                    }
                }
                (_, _) => {
                    panic!("Invalid input")
                }
            }
        }
    }

    max_depth += 2;

    // add sand
    while !set.contains(&(500, 0)) {
        let mut x = 500;
        let mut y = 0;

        loop {
            let mut flag = false;

            if !set.contains(&(x, y + 1)) {
                y += 1;
                flag = true;
            } else if !set.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
                flag = true;
            } else if !set.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
                flag = true;
            }

            if y == max_depth - 1 {
                break;
            }

            if !flag {
                break;
            }
        }

        result += 1;
        set.insert((x, y));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day14/test.txt"), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day14/test.txt"), 93);
    }
}
