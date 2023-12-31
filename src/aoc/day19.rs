#![allow(unused)]
use crate::aoc::read;

fn parse_string(strs: &[String]) -> Vec<[[u16; 4]; 4]> {
    let mut res = Vec::new();
    for s in strs {
        let datas = s.split(" Each ").collect::<Vec<&str>>();

        let mut blueprint = [[0; 4]; 4];

        for i in 1..=4 {
            let data = datas[i].split(' ').collect::<Vec<&str>>();
            match i {
                1..=2 => {
                    let num = data[3].parse::<i32>().unwrap();
                    blueprint[i - 1] = [num as u16, 0, 0, 0]
                }
                3 => {
                    let num1 = data[3].parse::<i32>().unwrap();
                    let num2 = data[6].parse::<i32>().unwrap();
                    blueprint[i - 1] = [num1 as u16, num2 as u16, 0, 0]
                }
                4 => {
                    let num1 = data[3].parse::<i32>().unwrap();
                    let num2 = data[6].parse::<i32>().unwrap();
                    blueprint[i - 1] = [num1 as u16, 0, num2 as u16, 0]
                }
                _ => panic!("Invalid index"),
            }
        }
        res.push(blueprint);
    }

    res
}

pub fn solve_part1(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    let blueprint = parse_string(&lines);
    return blueprint
        .iter()
        .map(|blueprint| max_geodes(blueprint, 24))
        .enumerate()
        .map(|(idx, geodes)| (idx + 1) * usize::from(geodes))
        .sum::<usize>() as i32;
}

pub fn solve_part2(filename: &str) -> i32 {
    let lines = read::read_lines(filename);
    let blueprint = parse_string(&lines);
    return blueprint
        .iter()
        .take(3)
        .map(|blueprint| usize::from(max_geodes(blueprint, 32)))
        .product::<usize>() as i32;
}

struct State {
    // [ore, clay, obsidian, geode]
    inventory: [u16; 4],
    // [ore_bots, clay_bots, obsidian_bots, geode_bots]
    bots: [u16; 4],
    // elapsed time in minutes
    elapsed: u16,
}

fn recurse(
    blueprint: &[[u16; 4]; 4],
    max_time: u16,
    max_robots: &[u16; 4],
    state: State,
    max_geodes: &mut u16,
) {
    let State {
        inventory,
        bots,
        elapsed,
    } = state;
    let mut recursed = false;
    // for every bot cost, run simulation
    for i in 0..blueprint.len() {
        // if we already have enough of this bot type, skip
        if bots[i] == max_robots[i] {
            continue;
        }

        let costs = &blueprint[i];

        // Find the limiting resource type for the costs.
        let wait_time = (0..3)
            .map(|idx| {
                match costs[idx] {
                    // state has enough of current resource in inventory to cover that part of the target bot cost. 0 wait time
                    cost if cost <= inventory[idx] => 0,
                    // no target bot type made yet
                    // we can't build it (it takes more than max_time to build it).
                    _ if bots[idx] == 0 => max_time + 1,
                    _ => (costs[idx] - inventory[idx] + bots[idx] - 1) / bots[idx],
                }
            })
            .max()
            .unwrap();

        // if that choice would cause the time limit be to exceeded, skip
        // the + 1 is so the built bot has the chance to do something, it merely being built is not enough
        let new_elapsed = elapsed + wait_time + 1;
        if new_elapsed >= max_time {
            continue;
        }

        // gather ores with previously available bots
        let mut new_inventory = [0; 4];
        for idx in 0..bots.len() {
            new_inventory[idx] = inventory[idx] + bots[idx] * (wait_time + 1) - costs[idx];
        }

        // increase bot type for the bot we just built
        let mut new_bots = bots;
        new_bots[i] += 1;

        // extra optimization:
        // if we theoretically only built geode bots every turn, and we still don't beat the maximum, skip
        let remaining_time = max_time - new_elapsed;
        if ((remaining_time - 1) * remaining_time) / 2
            + new_inventory[3]
            + remaining_time * new_bots[3]
            < *max_geodes
        {
            continue;
        }

        let new_state = State {
            inventory: new_inventory,
            bots: new_bots,
            elapsed: new_elapsed,
        };
        recursed = true;
        recurse(blueprint, max_time, max_robots, new_state, max_geodes)
    }
    if !recursed {
        let geodes = inventory[3] + bots[3] * (max_time - elapsed);
        *max_geodes = geodes.max(*max_geodes);
    }
}
fn max_geodes(blueprint: &[[u16; 4]; 4], max_time: u16) -> u16 {
    // calculate the maximum amount for every type of bot so that the creation of a new bot of any type is never bottlenecked
    // it doesn't make sense to build more bots than that maximum if the resources a bot type generates are
    // enough to cover that type (ore, clay, obsidian) cost for any possible bot (per question, you can only build 1 bot per turn)
    // for geode bots, there is no logical maximum amount
    // [ore, clay, obsidian, geode]
    let mut max_robots = [u16::MAX; 4];
    for i in 0..3 {
        max_robots[i] = blueprint.iter().map(|cost| cost[i]).max().unwrap();
    }
    let mut max_geodes = 0;

    let state = State {
        inventory: [0, 0, 0, 0],
        bots: [1, 0, 0, 0],
        elapsed: 0,
    };

    recurse(blueprint, max_time, &max_robots, state, &mut max_geodes);

    max_geodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("input/day19/test.txt"), 33);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input/day19/test.txt"), 3472);
    }
}
