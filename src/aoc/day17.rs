#![allow(unused)]
use std::collections::HashSet;

use crate::aoc::read;

pub fn solve_part1(filename: &str, num: i128) -> i128 {
    let pattern = &read::read_lines(filename)[0];
    let length = pattern.len();
    let mut idx = 0usize;
    let mut set: HashSet<(i128, i128)> = HashSet::new();

    let mut max_height = 0_i128;
    let mut times = 0_i128;

    while times < num {
        let mut v = Vec::new();
        let mut h = max_height + 4;
        let mut flag = false;

        let (mut left, mut right) = (0, 0);

        match times % 5 {
            //..@@@@.
            0 => {
                for i in 2..=5 {
                    v.push((i, h));
                }
                left = 2;
                right = 5;
            }
            //...@...
            //..@@@..
            //...@...
            1 => {
                for i in 2..=4 {
                    v.push((i, h + 1));
                }
                v.push((3, h));
                v.push((3, h + 2));
                left = 2;
                right = 4;
            }
            //....@..
            //....@..
            //..@@@..
            2 => {
                for i in 2..=4 {
                    v.push((i, h));
                }
                v.push((4, h + 1));
                v.push((4, h + 2));
                left = 2;
                right = 4;
            }
            //..@....
            //..@....
            //..@....
            //..@....
            3 => {
                for i in 0..=3 {
                    v.push((2, h + i));
                }
                left = 2;
                right = 2;
            }
            //..@@...
            //..@@...
            _ => {
                for i in 0..2 {
                    v.push((2, h + i));
                    v.push((3, h + i));
                }
                left = 2;
                right = 3;
            }
        }

        while !flag {
            let c = pattern.chars().nth(idx).unwrap();
            idx = (idx + 1) % length;
            let mut can_move = true;

            // 左右移动
            match c {
                '>' => {
                    if right <= 5 {
                        for (x, y) in &v {
                            if set.contains(&(*x + 1, *y)) {
                                can_move = false;
                                break;
                            }
                        }

                        if can_move {
                            for (x, y) in &mut v {
                                *x += 1;
                            }
                            left += 1;
                            right += 1;
                        }
                    }
                }
                '<' => {
                    if left > 0 {
                        for (x, y) in &v {
                            if set.contains(&(*x - 1, *y)) {
                                can_move = false;
                                break;
                            }
                        }

                        if can_move {
                            for (x, y) in &mut v {
                                *x -= 1;
                            }
                            left -= 1;
                            right -= 1;
                        }
                    }
                }
                _ => {
                    panic!("Invalid char")
                }
            }

            // 下落
            if h == 1 {
                flag = true;
            } else {
                for (x, y) in &v {
                    if set.contains(&(*x, *y - 1)) {
                        flag = true;
                        break;
                    }
                }

                if !flag {
                    for (x, y) in &mut v {
                        *y -= 1;
                    }

                    h -= 1;
                }
            }
        }

        for (x, y) in &v {
            set.insert((*x, *y));
            max_height = max_height.max(*y);
        }

        times += 1;
    }
    max_height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day17/test.txt", 2022), 3068);
    }
}
