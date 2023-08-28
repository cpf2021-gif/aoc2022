#![allow(unused)]
use crate::aoc::read;

fn parse_command(cmd_str: String) -> Vec<Operation> {
    let mut idx = 0_usize;
    let mut command = Vec::new();
    while idx < cmd_str.len() {
        let start = idx;
        let mut num = 0_i64;
        match cmd_str.chars().nth(idx).unwrap() {
            'R' => {
                idx += 1;
                command.push(Operation::Turnright);
            }
            'L' => {
                idx += 1;
                command.push(Operation::Turnleft);
            }
            ch => {
                while idx < cmd_str.len() && cmd_str.chars().nth(idx).unwrap().is_ascii_digit() {
                    num = num * 10 + cmd_str.chars().nth(idx).unwrap().to_digit(10).unwrap() as i64;
                    idx += 1;
                }
                command.push(Operation::Value(num));
            }
        }
    }
    command
}

pub fn solve_part1(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    // 解析命令
    let command = parse_command(lines.last().unwrap().clone());

    // 获取行和列数
    let mut row = lines.len() - 2;
    let mut col = 0_usize;
    for item in lines.iter().take(row) {
        col = col.max(item.len());
    }

    // 记录每行每列的边界
    let mut row_range = vec![(-1, -1); row];
    let mut col_range = vec![(-1, -1); col];

    // 初始化地图
    let mut map = vec![vec![' '; col]; row];
    for (i, line) in lines.iter().take(row).enumerate() {
        for (j, ch) in line.chars().enumerate() {
            map[i][j] = ch;
            if ch != ' ' {
                // 看行界
                match row_range[i] {
                    (-2, _) => {}
                    (-1, -1) => {
                        if ch == '#' {
                            row_range[i].0 = -2;
                        } else {
                            row_range[i].0 = j as i32;
                        }
                    }
                    (idx, _) => {
                        if ch == '#' {
                            row_range[i] = (idx, -2);
                        } else {
                            row_range[i].1 = j as i32;
                        }
                    }
                }

                // 看列界
                match col_range[j] {
                    (-2, _) => {}
                    (-1, -1) => {
                        if ch == '#' {
                            col_range[j].0 = -2;
                        } else {
                            col_range[j].0 = i as i32;
                        }
                    }
                    (idx, _) => {
                        if ch == '#' {
                            col_range[j] = (idx, -2);
                        } else {
                            col_range[j].1 = i as i32;
                        }
                    }
                }
            }
        }
    }

    // 初始化方向 一开始向右
    let diraction = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut now_dir = 0_usize;
    // 初始化坐标
    let (mut x, mut y) = (0_i32, 0_i32);
    y = map[0].iter().position(|&ch| ch == '.').unwrap() as i32;

    // 开始执行命令
    for cmd in command {
        match cmd {
            Operation::Value(v) => {
                let (dx, dy) = diraction[now_dir];
                for _ in 0..v {
                    let (nx, ny) = (x + dx, y + dy);
                    if nx < 0
                        || nx >= row as i32
                        || ny < 0
                        || ny >= col as i32
                        || map[nx as usize][ny as usize] == ' '
                    {
                        if now_dir == 0 || now_dir == 2 {
                            match row_range[x as usize] {
                                (-2, _) | (_, -2) => {
                                    continue;
                                }
                                (-1, _) | (_, -1) => {
                                    continue;
                                }
                                (l, r) => {
                                    if now_dir == 0 {
                                        y = l;
                                    } else {
                                        y = r;
                                    }
                                }
                            }
                        } else {
                            match col_range[y as usize] {
                                (-2, _) | (_, -2) => {
                                    continue;
                                }
                                (-1, _) | (_, -1) => {
                                    continue;
                                }
                                (l, r) => {
                                    if now_dir == 1 {
                                        x = l;
                                    } else {
                                        x = r;
                                    }
                                }
                            }
                        }
                    } else {
                        if map[nx as usize][ny as usize] == '#' {
                            continue;
                        }
                        x = nx;
                        y = ny;
                    }
                }
            }
            Operation::Turnleft => {
                now_dir = (now_dir + 3) % 4;
            }
            Operation::Turnright => {
                now_dir = (now_dir + 1) % 4;
            }
        }
    }
    (x + 1) * 1000 + (y + 1) * 4 + now_dir as i32
}

pub fn solve_part2(filename: &str) -> i32 {
    // pass
    let lines = read::read_lines(filename);
    0
}

#[derive(Debug)]
enum Operation {
    Value(i64),
    Turnleft,
    Turnright,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day22/test.txt"), 6032);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day22/test.txt"), 0);
    }
}
