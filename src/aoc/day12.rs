#![allow(unused)]
use std::collections::VecDeque;

use crate::aoc::read;

pub fn solve_part1(filename: &str) -> i32 {
    let mut sx = 0;
    let mut sy = 0;
    let mut ex = 0;
    let mut ey = 0;

    let lines = read::read_lines(filename);
    let mut datas = Vec::new();

    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    let m = lines.len();
    let n = lines[0].len();

    let mut queue = VecDeque::new();

    for (i, str) in lines.iter().enumerate() {
        let mut data = Vec::new();
        for (j, ch) in str.chars().enumerate() {
            match ch {
                'S' => {
                    sx = i;
                    sy = j;
                    data.push(('a', 0, -1));
                }
                'E' => {
                    ex = i;
                    ey = j;
                    data.push(('z', -1, -1));
                }
                _ => data.push((ch, -1, -1)),
            }
        }
        datas.push(data);
    }

    queue.push_back((sx, sy));

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        if datas[x][y].2 == 1 || datas[x][y].1 == -1 {
            continue;
        }

        for (dx, dy) in dirs.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if datas[nx][ny].0 as i32 - datas[x][y].0 as i32 > 1 {
                continue;
            }

            if datas[nx][ny].1 == -1 {
                datas[nx][ny].1 = datas[x][y].1 + 1;
            } else {
                datas[nx][ny].1 = datas[nx][ny].1.min(datas[x][y].1 + 1);
            }
            queue.push_back((nx, ny));
        }
        datas[x][y].2 = 1;
    }
    datas[ex][ey].1
}

pub fn solve_part2(filename: &str) -> i32 {
    let mut sx = 0;
    let mut sy = 0;
    let mut ex = 0;
    let mut ey = 0;

    let lines = read::read_lines(filename);
    let mut datas = Vec::new();

    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    let m = lines.len();
    let n = lines[0].len();

    let mut queue = VecDeque::new();

    for (i, str) in lines.iter().enumerate() {
        let mut data = Vec::new();
        for (j, ch) in str.chars().enumerate() {
            match ch {
                'S' => {
                    sx = i;
                    sy = j;
                    data.push(('a', -1, -1));
                }
                'E' => {
                    ex = i;
                    ey = j;
                    data.push(('z', 0, -1));
                }
                _ => data.push((ch, -1, -1)),
            }
        }
        datas.push(data);
    }

    let mut result = std::i32::MAX;

    queue.push_back((ex, ey));

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        if datas[x][y].2 == 1 || datas[x][y].1 == -1 {
            continue;
        }

        if (datas[x][y].0 == 'a') {
            result = result.min(datas[x][y].1);
        }

        for (dx, dy) in dirs.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if datas[x][y].0 as i32 - datas[nx][ny].0 as i32 > 1 {
                continue;
            }

            if datas[nx][ny].1 == -1 {
                datas[nx][ny].1 = datas[x][y].1 + 1;
            } else {
                datas[nx][ny].1 = datas[nx][ny].1.min(datas[x][y].1 + 1);
            }
            queue.push_back((nx, ny));
        }
        datas[x][y].2 = 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day12/test.txt"), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day12/test.txt"), 29);
    }
}
