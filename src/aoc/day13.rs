#![allow(unused)]
use crate::aoc::read;
use std::{cmp::Ordering, str::Chars};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Val {
    Num(i32),
    List(Vec<Self>),
}

impl Val {
    fn parse(s: &str) -> Self {
        let mut c = s.chars();
        if c.next().unwrap() != '[' {
            panic!("bad input");
        }
        Self::parse_into(&mut c)
    }

    fn parse_into(c: &mut Chars) -> Self {
        let mut result = Vec::new();
        let mut num = -1;
        while let Some(ch) = c.next() {
            match ch {
                '[' => result.push(Self::parse_into(c)),
                ',' => {
                    if num >= 0 {
                        result.push(Self::Num(num));
                        num = -1;
                    }
                }
                ']' => {
                    if num >= 0 {
                        result.push(Self::Num(num))
                    }
                    return Self::List(result);
                }
                '0'..='9' => {
                    if num == -1 {
                        num = (ch as u8 - b'0') as i32;
                    } else {
                        num = (num * 10) + (ch as u8 - b'0') as i32;
                    }
                }
                _ => panic!("bad char '{ch}'"),
            }
        }
        Self::List(result)
    }

    fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Val::List(left), Val::List(right)) => {
                let mut idx = 0;
                while idx < left.len() && idx < right.len() {
                    match (&left[idx], &right[idx]) {
                        (Val::Num(l), Val::Num(r)) => {
                            if l != r {
                                return l.cmp(r);
                            }
                        }
                        (Val::List(_), Val::Num(r)) => {
                            let check = left[idx].compare(&Val::List(vec![Val::Num(*r)]));
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                        (Val::Num(l), Val::List(_)) => {
                            let check = Val::List(vec![Val::Num(*l)]).compare(&right[idx]);
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                        (Val::List(_), Val::List(_)) => {
                            let check = left[idx].compare(&right[idx]);
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                    }
                    idx += 1;
                }
                left.len().cmp(&right.len())
            }
            _ => panic!("bad input"),
        }
    }
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for Val {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn solve_part1(filename: &str) -> i32 {
    let mut sum = 0;
    let lines = read::read_lines(filename);
    let pairs = lines.split(|line| line.is_empty()).collect::<Vec<_>>();

    for (i, pair) in pairs.iter().enumerate() {
        let left = Val::parse(&pair[0]);
        let right = Val::parse(&pair[1]);
        if left < right {
            sum += (i + 1) as i32;
        }
    }

    sum
}

pub fn solve_part2(filename: &str) -> i32 {
    let lines = read::read_lines(filename);

    let mut res = 1;

    let d2 = Val::parse("[[2]]");
    let d6 = Val::parse("[[6]]");

    let mut list = vec![d2.clone(), d6.clone()];
    for line in lines {
        if !line.is_empty() {
            list.push(Val::parse(&line));
        }
    }

    list.sort();
    for (i, val) in list.iter().enumerate() {
        if *val == d2 || *val == d6 {
            res *= (i + 1) as i32;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day13/test.txt"), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day13/test.txt"), 140);
    }
}
