#![allow(unused)]
use std::collections::HashSet;

use crate::aoc::read;

struct Snake {
    pos: Vec<(i32, i32)>,
    set: HashSet<(i32, i32)>,
}

impl Snake {
    fn new(len: usize) -> Snake {
        Snake {
            pos: vec![(0_i32, 0_i32); len],
            set: HashSet::new(),
        }
    }

    fn snake_move(&mut self, dir: &str) {
        match dir {
            "U" => {
                self.pos[0].1 += 1;
            }
            "D" => {
                self.pos[0].1 -= 1;
            }
            "L" => {
                self.pos[0].0 -= 1;
            }
            "R" => {
                self.pos[0].0 += 1;
            }
            _ => {}
        }
        for i in 1..self.pos.len() {
            let rowdiff = self.pos[i - 1].0 - self.pos[i].0;
            let coldiff = self.pos[i - 1].1 - self.pos[i].1;

            if rowdiff == 0 && coldiff.abs() > 1 {
                self.pos[i].1 += coldiff.signum();
            } else if coldiff == 0 && rowdiff.abs() > 1 {
                self.pos[i].0 += rowdiff.signum();
            } else if rowdiff.abs() > 1 || coldiff.abs() > 1 {
                self.pos[i].0 += rowdiff.signum();
                self.pos[i].1 += coldiff.signum();
            }
        }
        self.set.insert((
            self.pos[self.pos.len() - 1].0,
            self.pos[self.pos.len() - 1].1,
        ));
    }

    fn get_set_num(&self) -> i32 {
        self.set.len() as i32
    }
}

pub fn solve_part1(filename: &str) -> i32 {
    let lines = read::read_lines(filename);

    let mut snake = Snake::new(2);

    for line in lines {
        let command = line.split(' ').collect::<Vec<&str>>();
        let step = command[1].parse::<i32>().unwrap();
        match command[0] {
            "U" => {
                for _ in 0..step {
                    snake.snake_move("U");
                }
            }
            "D" => {
                for _ in 0..step {
                    snake.snake_move("D");
                }
            }
            "L" => {
                for _ in 0..step {
                    snake.snake_move("L");
                }
            }
            "R" => {
                for _ in 0..step {
                    snake.snake_move("R");
                }
            }
            _ => {}
        }
    }
    snake.get_set_num()
}

pub fn solve_part2(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    let mut snake = Snake::new(10);

    for line in lines {
        let command = line.split(' ').collect::<Vec<&str>>();
        let step = command[1].parse::<i32>().unwrap();
        match command[0] {
            "U" => {
                for _ in 0..step {
                    snake.snake_move("U");
                }
            }
            "D" => {
                for _ in 0..step {
                    snake.snake_move("D");
                }
            }
            "L" => {
                for _ in 0..step {
                    snake.snake_move("L");
                }
            }
            "R" => {
                for _ in 0..step {
                    snake.snake_move("R");
                }
            }
            _ => {}
        }
    }
    snake.get_set_num()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day9/test1.txt"), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day9/test2.txt"), 36);
    }
}
