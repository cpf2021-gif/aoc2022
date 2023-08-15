#![allow(unused)]
use std::collections::{HashMap, HashSet};

use crate::aoc::read;

pub fn solve_part1(filename: &str) -> i128 {
    let pattern = &read::read_lines(filename)[0];
    let length = pattern.len();
    let mut idx = 0usize;
    let mut set: HashSet<(i128, i128)> = HashSet::new();

    let mut max_height = 0_i128;
    let mut times = 0_i128;

    while times < 2022 {
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

pub fn solve_part2(filename: &str) -> i128 {
    let pattern = &read::read_lines(filename)[0];

    let mut part2 = Chamber::new(pattern.chars().collect());

    let mut cycle_finder = HashMap::new();

    // map state to (height, drops)
    cycle_finder.insert((part2.piecenum, part2.jetnum, 0usize), (0usize, 0usize));

    let mut drops = 0;
    loop {
        part2.drop_one();
        drops += 1;
        let height = part2.height();
        if height < 4 {
            continue;
        }

        let shape = ((part2.rocks[height - 1] as usize) << 24)
            | ((part2.rocks[height - 2] as usize) << 16)
            | ((part2.rocks[height - 3] as usize) << 8)
            | (part2.rocks[height - 4] as usize);

        if let Some(entry) = cycle_finder.get(&(part2.piecenum, part2.jetnum, shape)) {
            // println!("piece = {}", part2.piecenum);
            // println!("jetnum = {}", part2.jetnum);
            // println!("shape = {}", shape);

            // println!("drops until start of loop = {}", entry.1);
            // println!("height of tower when the loop started = {}", entry.0);
            let delta_height = height - entry.0;
            let delta_drops = drops - entry.1;
            // println!(
            // "There is an increase of {delta_height} rows for every {delta_drops} drops"
            // );
            let remaining_drops = Chamber::OUCH - entry.1;
            // println!("There are still {remaining_drops} left to go");

            let needed_drops = remaining_drops / delta_drops;
            let leftover_drops = remaining_drops % delta_drops;
            let integral_height = entry.0 + delta_height * needed_drops;

            // println!(
            // "The height will reach {integral_height} but there are still {leftover_drops} drops left"
            // );

            for _ in 0..leftover_drops {
                part2.drop_one();
            }
            let leftover_height = part2.height() - height;
            // println!("After {leftover_drops} more drops, we added {leftover_height} rows");

            return integral_height as i128 + leftover_height as i128;
        } else {
            cycle_finder.insert((part2.piecenum, part2.jetnum, shape), (height, drops));
        }
    }
}

#[derive(Default, Clone)]
struct Chamber {
    jets: Vec<char>,
    rocks: Vec<u8>,
    piecenum: usize,
    jetnum: usize,
}

impl Chamber {
    const PIECES: [[u8; 4]; 5] = [
        [0b0000000, 0b0000000, 0b0000000, 0b0011110],
        [0b0000000, 0b0001000, 0b0011100, 0b0001000],
        [0b0000000, 0b0000100, 0b0000100, 0b0011100],
        [0b0010000, 0b0010000, 0b0010000, 0b0010000],
        [0b0000000, 0b0000000, 0b0011000, 0b0011000],
    ];

    const OUCH: usize = 1_000_000_000_000;

    fn new(jets: Vec<char>) -> Self {
        Self {
            jets,
            rocks: vec![0, 0, 0, 0, 0, 0, 0],
            piecenum: 0,
            jetnum: 0,
        }
    }

    fn drop_one(&mut self) {
        let mut piece = Self::PIECES[self.piecenum];
        self.piecenum = (self.piecenum + 1) % Self::PIECES.len();

        // make room at the top for the new piece
        let mut last = self.rocks.len() - 7;
        while self.rocks[last] != 0 {
            self.rocks.push(0);
            last += 1;
        }

        let mut bottom = self.rocks.len() - 4;

        loop {
            // start off by using the jet to move the piece left or right
            let jet = self.jets[self.jetnum];
            self.jetnum = (self.jetnum + 1) % self.jets.len();

            match jet {
                '<' => {
                    if self.can_go_left(bottom, &piece) {
                        for p in piece.iter_mut() {
                            *p <<= 1;
                        }
                    }
                }
                '>' => {
                    if self.can_go_right(bottom, &piece) {
                        for p in piece.iter_mut() {
                            *p >>= 1;
                        }
                    }
                }
                _ => panic!("bad input '{jet}'"),
            }

            // drop the piece by one if it can
            if bottom > 0 && self.can_go_to(bottom - 1, &piece) {
                bottom -= 1;
            } else {
                break;
            }
        }

        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            self.rocks[bottom] |= piece[prow];
            bottom += 1;
        }
    }

    fn can_go_left(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            if (piece[prow] & 0x40) != 0 || (self.rocks[bottom] & (piece[prow] << 1)) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn can_go_right(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            if (piece[prow] & 0x01) != 0 || (self.rocks[bottom] & (piece[prow] >> 1)) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn can_go_to(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            if (self.rocks[bottom] & piece[prow]) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn height(&self) -> usize {
        let mut top = self.rocks.len();
        while top > 0 && self.rocks[top - 1] == 0 {
            top -= 1;
        }
        top
    }

    fn _print_piece(piece: &[u8; 4]) {
        for p in piece {
            Self::_print_row(*p);
            println!();
        }
    }

    fn _print_row(row: u8) {
        let mut bit = 0x40;
        while bit > 0 {
            print!("{}", if (bit & row) != 0 { '#' } else { '.' });
            bit >>= 1;
        }
    }

    fn _draw(&self) {
        let mut top = self.rocks.len();
        while top > 0 {
            top -= 1;
            print!("|");
            Self::_print_row(self.rocks[top]);
            println!("|");
        }
        println!("+-------+");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day17/test.txt"), 3068);
    }

    #[test]
    fn test_chamber() {
        assert_eq!(solve_part2("input/day17/test.txt"), 1514285714288);
    }
}
