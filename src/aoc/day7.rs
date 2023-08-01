#![allow(unused)]
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::aoc::read;

struct Diretory {
    name: String,
    children: HashMap<String, Rc<RefCell<Diretory>>>,
    weight: i32,
    parent: Option<Rc<RefCell<Diretory>>>,
}

impl Diretory {
    fn new(name: String, weight: i32) -> Self {
        Self {
            name,
            children: HashMap::new(),
            weight,
            parent: None,
        }
    }

    fn add_child(&mut self, child: Rc<RefCell<Diretory>>) {
        self.children
            .insert(child.borrow().name.clone(), child.clone());
    }

    fn set_parent(&mut self, parent: Rc<RefCell<Diretory>>) {
        self.parent = Some(parent);
    }

    fn get_all_weight(&self) -> i32 {
        let mut result = self.weight;
        for child in self.children.values() {
            result += child.borrow().get_all_weight();
        }
        result
    }

    fn print(&self) {
        println!("{}: {}", self.name, self.weight);
        for child in self.children.values() {
            child.borrow().print();
        }
    }

    fn add_child_weight(&mut self) -> i32 {
        if self.children.is_empty() {
            return self.weight;
        } else {
            let mut child_weight = 0;
            for child in self.children.values() {
                child_weight += child.borrow_mut().add_child_weight();
            }
            self.weight += child_weight;
        }
        self.weight
    }

    fn count(&self) -> i32 {
        let mut result = 0;
        if self.weight <= 100000 {
            result = self.weight;
        } else {
            result = 0;
        }
        if self.children.is_empty() {
            return result;
        }
        for child in self.children.values() {
            result += child.borrow().count();
        }
        result
    }

    fn find(&self, target: i32) -> i32 {
    let mut result = std::i32::MAX;
       if self.weight >= target {
           result = self.weight;
       }
       if !self.children.is_empty() {
            for child in self.children.values() {
                let child_result = child.borrow().find(target);
                if child_result < result {
                    result = child_result;
                }
            }
       }
       result
    }
}

fn execute1(lines: Vec<String>) -> (Rc<RefCell<Diretory>>, i32) {
    let root = Rc::new(RefCell::new(Diretory::new("/".to_string(), 0)));
    let mut cur_node = root.clone();
    for line in lines {
        if line.starts_with("$ cd ..") {
            let partent = cur_node.borrow().parent.clone().unwrap();
            cur_node = partent;
        } else if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("$ cd") {
            let name = line.split(' ').collect::<Vec<&str>>()[2];
            if name != "/" {
                let child = cur_node.borrow().children.get(name).unwrap().clone();
                cur_node = child;
            }
        } else if line.starts_with("dir") {
            let name = line.split(' ').collect::<Vec<&str>>()[1];
            let child = Rc::new(RefCell::new(Diretory::new(
                name.to_string(),
                0
            )));
            cur_node.borrow_mut().add_child(child.clone());
            child.borrow_mut().set_parent(cur_node.clone());
        } else {
            let weight = line.split(' ').collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
            cur_node.borrow_mut().weight += weight;
        }
    }
    root.borrow_mut().add_child_weight();
    let result = root.borrow().count();
    (root, result)
}

fn execute2(lines: Vec<String>) -> (Rc<RefCell<Diretory>>, i32) {
    let root = Rc::new(RefCell::new(Diretory::new("/".to_string(), 0)));
    let mut cur_node = root.clone();
    for line in lines {
        if line.starts_with("$ cd ..") {
            let partent = cur_node.borrow().parent.clone().unwrap();
            cur_node = partent;
        } else if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("$ cd") {
            let name = line.split(' ').collect::<Vec<&str>>()[2];
            if name != "/" {
                let child = cur_node.borrow().children.get(name).unwrap().clone();
                cur_node = child;
            }
        } else if line.starts_with("dir") {
            let name = line.split(' ').collect::<Vec<&str>>()[1];
            let child = Rc::new(RefCell::new(Diretory::new(
                name.to_string(),
                0
            )));
            cur_node.borrow_mut().add_child(child.clone());
            child.borrow_mut().set_parent(cur_node.clone());
        } else {
            let weight = line.split(' ').collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
            cur_node.borrow_mut().weight += weight;
        }
    }
    root.borrow_mut().add_child_weight();
    let target = 30000000 - (70000000 - root.borrow().weight);
    println!("target: {}", target);
    let result = root.borrow().find(target);
    (root, result)
}

pub fn solve_part1(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    let (root, result) = execute1(lines);
    // root.borrow().print();
    result
}

pub fn solve_part2(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    let (root, result) = execute2(lines);
    // root.borrow().print();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day7/test.txt"), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day7/test.txt"), 24933642);
    }
}
