#![allow(unused)]
use crate::aoc::read;

fn divide_lines(lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    // Find the index of the first empty line
    let index = lines.iter().position(|line| line.is_empty()).unwrap();
    (lines[..index].to_vec(), lines[index + 1..].to_vec())
}

fn init_data(data: Vec<String>) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    let size = data.len();
    let len = data.last().unwrap().len();

    for _ in 0..(len + 1) / 4 {
        result.push(Vec::new());
    }

    for i in (0..size - 1).rev() {
        let line = &data[i];
        for j in (0..(len + 1) / 4) {
            let idx = 1 + 4 * j;
            let ch = line.chars().nth(idx).unwrap();
            if let 'A'..='Z' = ch {
                result[j].push(ch);
            }
        }
    }
    result
}

fn execute1(data: &mut [Vec<char>], message: Vec<String>) -> String {
   for line in message {
        // move 1 from 1 to 2, get 3 number in the line
        let iter: Vec<&str> = line.split_whitespace().collect();
        let mut num = iter[1].parse::<usize>().unwrap(); 
        let from = iter[3].parse::<usize>().unwrap() - 1;
        let to = iter[5].parse::<usize>().unwrap() - 1;
        if num > data[from].len() {
           num = data[from].len(); 
        }
        for _ in 0..num {
            let ch = data[from].pop().unwrap();
            data[to].push(ch);
        } 
   } 
   data.iter().map(|v| v.last().unwrap()).collect::<String>()
}

fn execute2(data: &mut [Vec<char>], message: Vec<String>) -> String {
   for line in message {
        // move 1 from 1 to 2, get 3 number in the line
        let iter: Vec<&str> = line.split_whitespace().collect();
        let mut num = iter[1].parse::<usize>().unwrap(); 
        let from = iter[3].parse::<usize>().unwrap() - 1;
        let to = iter[5].parse::<usize>().unwrap() - 1;
        if num > data[from].len() {
           num = data[from].len(); 
        }
        let mut temp = Vec::new();
        for _ in 0..num {
            temp.push(data[from].pop().unwrap());
        }
        temp.reverse();
        data[to].extend(temp);
   } 
   data.iter().map(|v| v.last().unwrap()).collect::<String>()
}

pub fn solve_part1(filename: &str) -> String {
    let lines = read::read_lines(filename);
    let (data, message) = divide_lines(lines);
    let mut result = init_data(data);
    execute1(&mut result, message)
}

pub fn solve_part2(filename: &str) -> String {
    let lines = read::read_lines(filename);
    let (data, message) = divide_lines(lines);
    let mut result = init_data(data);
    execute2(&mut result, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day5/test.txt"), "CMZ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day5/test.txt"), "MCD");
    }
}
