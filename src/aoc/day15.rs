#![allow(unused)]
use std::collections::HashSet;

use crate::aoc::read;

fn parse_point(s: &str) -> (i64, i64) {
    let datas = s.split(", ").collect::<Vec<&str>>();
    let x = datas[0].split('=').last().unwrap().parse::<i64>().unwrap();
    let y = datas[1].split('=').last().unwrap().parse::<i64>().unwrap();
    (x, y)
}

pub fn solve_part1(filename: &str) -> i64 {
    let lines = read::read_lines(filename);
    let mut set = HashSet::new();
    for line in lines {
        let datas = line.split(": ").collect::<Vec<&str>>();
        let p1 = datas[0].split("at ").last().unwrap();
        let p2 = datas[1].split("at ").last().unwrap();
        let (x1, y1) = parse_point(p1);
        let (x2, y2) = parse_point(p2);
        let d = (x1 - x2).abs() + (y1 - y2).abs();

        let dx = d - (y1 - 2000000).abs();
        if dx > 0 {
            for i in (x1 - dx)..(x1 + dx + 1) {
                if !(y2 == 2000000 && i == x2) {
                    set.insert(i);
                }
            }
        }
    }
    set.len() as i64
}

pub fn solve_part2(filename: &str) -> i64 {
    let lines = read::read_lines(filename);

    let mut sensor = Vec::new();

    for line in lines {
        let datas = line.split(": ").collect::<Vec<&str>>();
        let p1 = datas[0].split("at ").last().unwrap();
        let p2 = datas[1].split("at ").last().unwrap();

        let (x1, y1) = parse_point(p1);
        let (x2, y2) = parse_point(p2);

        let d = (x1 - x2).abs() + (y1 - y2).abs();
        sensor.push((x1, y1, x2, y2, d));
    }

    for row in 0..=20 {
        let mut rowdata = vec![0..=20];
        for s in &sensor {
            let radius = s.4;
            let top = 0.max(s.1 - radius);
            let bottom = 20.min(s.1 + radius);
            if top > row || bottom < row {
                continue;
            }

            let dist = (s.1 - row).abs();
            let min_x = 0.max(s.0 - (radius - dist));
            let max_x = 20.min(s.0 + (radius - dist));

            let mut new_range = Vec::new();

            for r in &rowdata {
                let start = *r.start();
                if start > max_x {
                    new_range.push(r.clone());
                    continue;
                }

                let end = *r.end();
                if end < min_x {
                    new_range.push(r.clone());
                    continue;
                }

                if start < min_x {
                    new_range.push(start..=min_x - 1);
                }

                if end > max_x {
                    new_range.push(max_x + 1..=end);
                }
            }
            rowdata = new_range;
        }

        if !rowdata.is_empty() {
            let x = *rowdata[0].start();
            println!("{}", x * 4000000 + row);
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day15/test.txt"), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day15/test.txt"), 1);
    }
}
