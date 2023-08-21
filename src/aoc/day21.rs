#![allow(unused)]
use std::collections::HashMap;

use crate::aoc::read;

#[derive(Debug)]
enum Monkey {
    Value(i64),
    Binary(String, Operation, String),
}

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn parse(ch: char) -> Self {
        match ch {
            '+' => Self::Add,
            '-' => Self::Sub,
            '*' => Self::Mul,
            '/' => Self::Div,
            _ => panic!("not an operator: '{ch}'"),
        }
    }
}

fn eval(start: &String, tree: &HashMap<String, Monkey>) -> i64 {
    let mut node = tree.get(start).unwrap();
    match node {
        Monkey::Value(v) => *v,
        Monkey::Binary(left, op, right) => {
            let l = eval(left, tree);
            let r = eval(right, tree);
            match op {
                Operation::Add => l + r,
                Operation::Sub => l - r,
                Operation::Mul => l * r,
                Operation::Div => l / r,
            }
        }
    }
}

fn find_human<'a>(loc: &'a String, tree: &'a HashMap<String, Monkey>) -> Option<Vec<&'a String>> {
    if loc == "humn" {
        return Some(vec![loc]);
    }

    if let Some(Monkey::Binary(l, _, r)) = tree.get(loc) {
        if let Some(mut vec) = find_human(l, tree) {
            vec.push(loc);
            return Some(vec);
        }
        if let Some(mut vec) = find_human(r, tree) {
            vec.push(loc);
            return Some(vec);
        }
    }

    None
}

pub fn solve_part1(filename: &str) -> i64 {
    let lines = read::read_lines(filename);
    let mut tree = HashMap::new();
    for line in lines {
        let mut datas = line.split(": ").collect::<Vec<_>>();
        let name = datas[0].to_string();
        if datas[1].contains(' ') {
            let mut values = datas[1].split(' ').collect::<Vec<_>>();
            let mut left = values[0].to_string();
            let mut right = values[2].to_string();
            let op = Operation::parse(values[1].chars().next().unwrap());
            tree.insert(name, Monkey::Binary(left, op, right));
        } else {
            tree.insert(name, Monkey::Value(datas[1].parse::<i64>().unwrap()));
        }
    }
    eval(&"root".to_string(), &tree)
}

pub fn solve_part2(filename: &str) -> i64 {
    let lines = read::read_lines(filename);
    let mut tree = HashMap::new();

    let mut l_monkey = String::new();
    let mut r_monkey = String::new();

    for line in lines {
        let mut datas = line.split(": ").collect::<Vec<_>>();
        let name = datas[0].to_string();
        if datas[1].contains(' ') {
            let mut values = datas[1].split(' ').collect::<Vec<_>>();
            let mut left = values[0].to_string();
            let mut right = values[2].to_string();
            if name.contains("root") {
                l_monkey = left.clone();
                r_monkey = right.clone();
            }
            let op = Operation::parse(values[1].chars().next().unwrap());
            tree.insert(name, Monkey::Binary(left, op, right));
        } else {
            tree.insert(name, Monkey::Value(datas[1].parse::<i64>().unwrap()));
        }
    }

    let root = "root".to_string();
    let path = find_human(&root, &tree).unwrap();
    let path = path.iter().rev().copied().collect::<Vec<_>>();

    let correct_val = if l_monkey.contains(path[1]) {
        eval(&r_monkey, &tree)
    } else {
        eval(&l_monkey, &tree)
    };

    find_adjustment(&path, 1, &tree, correct_val)
}

fn find_adjustment(
    path: &Vec<&String>,
    index: usize,
    tree: &HashMap<String, Monkey>,
    cv: i64,
) -> i64 {
    use Operation::*;
    match tree.get(path[index]).unwrap() {
        Monkey::Value(_) => cv,
        Monkey::Binary(l, op, r) => {
            let left = eval(l, tree);
            let right = eval(r, tree);
            let new_cv = if l == path[index + 1] {
                match op {
                    Add => cv - right,
                    Sub => cv + right,
                    Mul => cv / right,
                    Div => cv * right,
                }
            } else {
                match op {
                    Add => cv - left,
                    Sub => left - cv,
                    Mul => cv / left,
                    Div => left / cv,
                }
            };
            find_adjustment(path, index + 1, tree, new_cv)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day21/test.txt"), 152);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day21/test.txt"), 301);
    }
}
