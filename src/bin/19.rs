use elves::{many_max, many_min};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"\d+"#).unwrap();
}

#[derive(Debug)]
struct Blueprint {
    ore: u32,
    cly: u32,
    obs: (u32, u32),
    geo: (u32, u32),
}

impl From<&str> for Blueprint {
    fn from(value: &str) -> Self {
        let numbers = RE
            .captures_iter(value)
            .map(|x| x[0].parse().unwrap())
            .collect_vec();
        Blueprint {
            ore: numbers[1],
            cly: numbers[2],
            obs: (numbers[3], numbers[4]),
            geo: (numbers[5], numbers[6]),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Ores {
    ore: u32,
    cly: u32,
    obs: u32,
    geo: u32,
}

#[derive(Debug, Clone, Copy)]
struct State {
    time: u32,
    ores: Ores,
    robs: Ores,
}

impl State {
    fn simulate(&self, steps: u32, max_time: u32) -> Self {
        let time = many_min!(max_time - self.time, steps);
        Self {
            time: self.time + time,
            ores: Ores {
                ore: self.ores.ore + self.robs.ore * time,
                cly: self.ores.cly + self.robs.cly * time,
                obs: self.ores.obs + self.robs.obs * time,
                geo: self.ores.geo + self.robs.geo * time,
            },
            robs: self.robs,
        }
    }
}

fn simulate(blueprint: &Blueprint, max_time: u32) -> u32 {
    let start = State {
        time: 0,
        ores: Ores {
            ore: 0,
            cly: 0,
            obs: 0,
            geo: 0,
        },
        robs: Ores {
            ore: 1,
            cly: 0,
            obs: 0,
            geo: 0,
        },
    };

    let mut best = u32::min_value();
    let mut earl = u32::max_value();

    let mut queue = vec![start];

    while !queue.is_empty() {
        let state = queue.pop().unwrap();

        // check if time has run out
        if state.time >= max_time {
            best = many_max!(best, state.ores.geo);
            continue;
        }

        // optimization: check if this branch can reach best if increase production by one each
        // round from now on
        if state.ores.geo
            + (0..max_time - state.time)
                .map(|x| x + state.robs.geo)
                .sum::<u32>()
            <= best
        {
            continue;
        }

        // create ore robot
        if state.robs.ore > 0 {
            let needed = many_max!(0, blueprint.ore as i32 - state.ores.ore as i32) as u32;
            let wait = (needed as f64 / state.robs.ore as f64).ceil() as u32 + 1;
            let mut new_state = state.clone().simulate(wait, max_time);
            if blueprint.ore <= new_state.ores.ore {
                new_state.ores.ore -= blueprint.ore;
                new_state.robs.ore += 1;
            }
            queue.push(new_state);
        }

        // create clay robot
        if state.robs.ore > 0 {
            let needed = many_max!(0, blueprint.cly as i32 - state.ores.ore as i32) as u32;
            let wait = (needed as f64 / state.robs.ore as f64).ceil() as u32 + 1;
            let mut new_state = state.clone().simulate(wait, max_time);
            if blueprint.cly <= new_state.ores.ore {
                new_state.ores.ore -= blueprint.cly;
                new_state.robs.cly += 1;
            }
            queue.push(new_state);
        }

        // create obsidian robot
        if state.robs.ore > 0 && state.robs.cly > 0 {
            let n_ore = many_max!(0, blueprint.obs.0 as i32 - state.ores.ore as i32) as u32;
            let n_cly = many_max!(0, blueprint.obs.1 as i32 - state.ores.cly as i32) as u32;
            let wait = many_max!(
                1,
                (n_ore as f64 / state.robs.ore as f64).ceil() as u32 + 1,
                (n_cly as f64 / state.robs.cly as f64).ceil() as u32 + 1
            );
            let mut new_state = state.clone().simulate(wait, max_time);
            if blueprint.obs.0 <= new_state.ores.ore && blueprint.obs.1 <= new_state.ores.cly {
                new_state.ores.ore -= blueprint.obs.0;
                new_state.ores.cly -= blueprint.obs.1;
                new_state.robs.obs += 1;
            }
            queue.push(new_state);
        }

        // create geode robot
        if state.robs.ore > 0 && state.robs.obs > 0 {
            let n_ore = many_max!(0, blueprint.geo.0 as i32 - state.ores.ore as i32) as u32;
            let n_obs = many_max!(0, blueprint.geo.1 as i32 - state.ores.obs as i32) as u32;
            let wait = many_max!(
                1,
                (n_ore as f64 / state.robs.ore as f64).ceil() as u32 + 1,
                (n_obs as f64 / state.robs.obs as f64).ceil() as u32 + 1
            );
            let mut new_state = state.clone().simulate(wait, max_time);
            if blueprint.geo.0 <= new_state.ores.ore && blueprint.geo.1 <= new_state.ores.obs {
                new_state.ores.ore -= blueprint.geo.0;
                new_state.ores.obs -= blueprint.geo.1;
                new_state.robs.geo += 1;
                earl = many_min!(earl, new_state.time);
            }
            queue.push(new_state);
        }
    }

    best
}

pub fn part_one(input: &str) -> Option<u32> {
    let blueprints = input.lines().map(Blueprint::from).collect_vec();
    Some(
        blueprints
            .iter()
            .enumerate()
            .map(|(idbp, bp)| (idbp + 1) as u32 * simulate(bp, 24))
            .sum(),
    )
}
pub fn part_two(input: &str) -> Option<u32> {
    let blueprints = input.lines().map(Blueprint::from).collect_vec();
    Some(blueprints[..3].iter().map(|bp| simulate(bp, 32)).product())
}
fn main() {
    let input = &aoc::read_file("inputs", 19);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 19);
        assert_eq!(part_one(&input), Some(33));
    }
}
