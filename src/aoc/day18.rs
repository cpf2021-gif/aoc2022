#![allow(unused)]
use crate::aoc::read;
use std::collections::{HashSet, VecDeque};

pub fn solve_part1(filename: &str) -> i32 {
    let lines = read::read_lines(filename);

    let mut result = lines.len() as i32 * 6;

    let mut set = HashSet::new();

    let mut v = Vec::new();

    for line in lines {
        let nums = line.split(',').collect::<Vec<&str>>();
        let x = nums[0].parse::<i32>().unwrap();
        let y = nums[1].parse::<i32>().unwrap();
        let z = nums[2].parse::<i32>().unwrap();
        set.insert((x, y, z));
        v.push((x, y, z));
    }

    let dist = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    for point in v {
        for d in dist.iter() {
            let x = point.0 + d.0;
            let y = point.1 + d.1;
            let z = point.2 + d.2;
            if set.contains(&(x, y, z)) {
                result -= 1;
            }
        }
    }

    result
}

pub fn solve_part2(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    let mut cubes = Vec::new();
    for line in lines {
        let nums = line.split(',').collect::<Vec<&str>>();
        let x = nums[0].parse::<i32>().unwrap();
        let y = nums[1].parse::<i32>().unwrap();
        let z = nums[2].parse::<i32>().unwrap();
        cubes.push((x, y, z));
    }

    let mut xrange = (i32::MAX, i32::MIN);
    let mut yrange = (i32::MAX, i32::MIN);
    let mut zrange = (i32::MAX, i32::MIN);

    let dist = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    for c in &cubes {
        xrange.0 = xrange.0.min(c.0);
        xrange.1 = xrange.1.max(c.0);
        yrange.0 = yrange.0.min(c.1);
        yrange.1 = yrange.1.max(c.1);
        zrange.0 = zrange.0.min(c.2);
        zrange.1 = zrange.1.max(c.2);
    }

    xrange = (xrange.0 - 1, xrange.1 + 1);
    yrange = (yrange.0 - 1, yrange.1 + 1);
    zrange = (zrange.0 - 1, zrange.1 + 1);

    let mut seen = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((xrange.0, yrange.0, zrange.0));

    let mut count = 0;
    while let Some(pos) = to_visit.pop_front() {
        if !seen.insert(pos) {
            continue;
        }

        for d in dist {
            let next_pos = (pos.0 + d.0, pos.1 + d.1, pos.2 + d.2);

            if next_pos.0 < xrange.0
                || next_pos.0 > xrange.1
                || next_pos.1 < yrange.0
                || next_pos.1 > yrange.1
                || next_pos.2 < zrange.0
                || next_pos.2 > zrange.1
            {
                continue;
            }

            if cubes.clone().contains(&next_pos) {
                count += 1;
            } else {
                to_visit.push_back(next_pos);
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day18/test.txt"), 64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day18/test.txt"), 58);
    }
}
