use crate::aoc::read;

pub fn solve_part1() {
    let lines = read::read_lines("input/day1/input.txt");
    let mut sum = 0;
    let mut max = 0;

    for line in lines {
        if line == "" {
            if sum > max {
                max = sum;
            }
            sum = 0;
            continue;
        }
        sum += line.parse::<i32>().unwrap();
    }
    println!("Day 1, Part 1: {}", max)
}

pub fn solve_part2()  {
    let lines = read::read_lines("input/day1/input.txt");
    let mut sum = 0;
    let mut sums = vec![];
    for line in lines {
        if line == "" {
            sums.push(sum);
            sum = 0;
            continue;
        }
        sum += line.parse::<i32>().unwrap();
    }
    sums.sort_by(
        |a, b| b.partial_cmp(a).unwrap()
    );
    let mut result = 0;
    for i in 0..3 {
        result += sums[i];
    }
    println!("Day 1, Part 2: {}", result)
}