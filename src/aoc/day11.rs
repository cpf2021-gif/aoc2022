#![allow(unused)]
use crate::aoc::read;

struct Monkey {
    operation: Box<dyn Fn(i64) -> i64>,
    number: i64,
    true_idx: usize,
    false_idx: usize,
    times: i64,
}

fn parse_data(str: &[String]) -> (Vec<i64>, Monkey) {
    let mut number = 0;
    let mut true_idx = 0;
    let mut false_idx = 0;

    let mut monkey = Monkey {
        operation: Box::new(|x| x),
        number,
        true_idx,
        false_idx,
        times: 0,
    };

    // parse items
    let (_, items_str) = str[1].split_once(": ").unwrap();
    let items = items_str
        .split(", ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    // parse operation
    let (_, operation_str) = str[2].split_once("old ").unwrap();
    let operation_str = operation_str.split(' ').collect::<Vec<_>>();
    match operation_str[0] {
        "*" => {
            if operation_str[1] == "old" {
                monkey.operation = Box::new(|x| x * x);
            } else {
                let num = operation_str[1].parse::<i64>().unwrap();
                monkey.operation = Box::new(move |x| (x * num));
            }
        }
        "+" => {
            if operation_str[1] == "old" {
                monkey.operation = Box::new(|x| x + x);
            } else {
                let num = operation_str[1].parse::<i64>().unwrap();
                monkey.operation = Box::new(move |x| (x + num));
            }
        }
        _ => panic!("Invalid operation"),
    }

    // parse number
    let number = str[3]
        .split(' ')
        .collect::<Vec<_>>()
        .last()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    monkey.number = number;

    // parse true_idx
    let true_idx = str[4]
        .split(' ')
        .collect::<Vec<_>>()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    monkey.true_idx = true_idx;

    // parse false_idx
    let false_idx = str[5]
        .split(' ')
        .collect::<Vec<_>>()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    monkey.false_idx = false_idx;

    (items, monkey)
}

pub fn solve_part1(filename: &str) -> i64 {
    let lines = read::read_lines(filename);
    let datas = lines.split(|line| line.is_empty()).collect::<Vec<_>>();

    let mut monkeys = Vec::new();
    let mut items = Vec::new();
    let mut times = Vec::new();

    for data in datas {
        let (item, monkey) = parse_data(data);
        monkeys.push(monkey);
        items.push(item);
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];

            if items[i].is_empty() {
                continue;
            }
            monkey.times += items[i].len() as i64;

            while !items[i].is_empty() {
                let mut num = items[i].pop().unwrap();
                num = (monkey.operation)(num);
                num /= 3;
                if num % monkey.number == 0 {
                    items[monkey.true_idx].push(num);
                } else {
                    items[monkey.false_idx].push(num);
                }
            }
        }
    }

    for monkey in monkeys {
        let num = monkey.times;
        times.push(num);
    }

    times.sort();
    times.reverse();
    times[0] * times[1]
}

pub fn solve_part2(filename: &str) -> i64 {
    let lines = read::read_lines(filename);
    let datas = lines.split(|line| line.is_empty()).collect::<Vec<_>>();

    let mut monkeys = Vec::new();
    let mut items = Vec::new();
    let mut times = Vec::new();

    for data in datas {
        let (item, monkey) = parse_data(data);
        monkeys.push(monkey);
        items.push(item);
    }

    let magic_num = monkeys.iter().map(|x| x.number).product::<i64>();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];

            if items[i].is_empty() {
                continue;
            }
            monkey.times += items[i].len() as i64;

            while !items[i].is_empty() {
                let mut num = items[i].pop().unwrap();
                num = (monkey.operation)(num);
                num %= magic_num;
                if num % monkey.number == 0 {
                    items[monkey.true_idx].push(num);
                } else {
                    items[monkey.false_idx].push(num);
                }
            }
        }
    }

    for monkey in monkeys {
        let num = monkey.times;
        times.push(num);
    }

    times.sort();
    times.reverse();
    times[0] * times[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day11/test.txt"), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day11/test.txt"), 2713310158);
    }
}
