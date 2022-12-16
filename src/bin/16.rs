use hashbrown::HashMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r#"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)"#).unwrap();
}

#[derive(Debug, Hash)]
struct Valve {
    flow: u32,
    to: Vec<u32>,
}

fn parse_graph(input: &str) -> (HashMap<u32, Valve>, (u32, u32)) {
    let mut inner = HashMap::new();
    let mut counter_flow = 0;
    let mut counter_other = 32;

    let mut map = HashMap::new();

    // 2 passes, get all that have any flow and unber them with 0 - 15 (there is less then 16
    // in my input so i can use u16 bitmask)
    for line in input.lines() {
        let cap = RE.captures(line).unwrap();
        let from = cap[1].to_string();
        let flow: u32 = cap[2].parse().unwrap();

        if flow > 0 {
            inner.insert(from, counter_flow);
            counter_flow += 1;
        } else {
            inner.insert(from, counter_other);
            counter_other += 1;
        }
    }

    for line in input.lines() {
        let cap = RE.captures(line).unwrap();
        let from = cap[1].to_string();
        let flow: u32 = cap[2].parse().unwrap();
        let to = cap[3]
            .split(", ")
            .map(|x| *inner.get(x).unwrap())
            .collect_vec();

        map.insert(*inner.get(&from).unwrap(), Valve { flow, to });
    }

    (map, (*inner.get("AA").unwrap(), counter_flow))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, (start, mlen)) = parse_graph(input);

    let mut queue: Vec<(u32, u32, u32, u32)> = vec![(1, start, 0, 0)];
    let mut memo: HashMap<(u32, u32), u32> = HashMap::new();
    let mut best = 0;

    while !queue.is_empty() {
        let (time, location, score, opened_mask) = queue.pop().unwrap();

        if let Some(x) = memo.get(&(time, location)) {
            if x >= &score {
                continue;
            }
        }

        memo.insert((time, location), score);

        if time == 30 {
            best = max(best, score);
            continue;
        }

        // open here
        if map.get(&location).unwrap().flow > 0 && 2_u32.pow(location) & opened_mask == 0 {
            let next_mask = opened_mask | 2_u32.pow(location);
            // println!("{}\n{}", opened_mask, next_mask);
            let next_score = score
                + (0..mlen)
                    .filter(|x| 2_u32.pow(*x) & next_mask == 2_u32.pow(*x))
                    .map(|x| map.get(&x).unwrap().flow)
                    .sum::<u32>();
            queue.push((time + 1, location, next_score, next_mask));
        }

        // neighbours
        for n in map.get(&location).unwrap().to.iter() {
            let next_score = score
                + (0..mlen)
                    .filter(|x| 2_u32.pow(*x) & opened_mask == 2_u32.pow(*x))
                    .map(|x| map.get(&x).unwrap().flow)
                    .sum::<u32>();
            queue.push((time + 1, *n, next_score, opened_mask));
        }
    }

    Some(best)
}
pub fn part_two(input: &str) -> Option<u32> {
    let (map, (start, mlen)) = parse_graph(input);

    let mut queue: Vec<(u32, u32, u32, u32, u32)> = vec![(1, start, start, 0, 0)];
    let mut memo: HashMap<(u32, u32, u32), u32> = HashMap::new();
    let mut best = 0;

    let max_flow: u32 = (0..mlen)
        .filter_map(|x| map.get(&(x as u32)))
        .map(|x| 2_u32.pow(x.flow as u32))
        .sum();

    while !queue.is_empty() {
        let (time, lm, le, score, opened_mask) = queue.pop().unwrap();

        if let Some(x) = memo.get(&(time, lm, le)) {
            if x >= &score {
                continue;
            }
        }

        memo.insert((time, lm, le), score);

        if time == 26 {
            best = max(best, score);
            continue;
        }
        let increase = (0..mlen)
            .filter(|x| 2_u32.pow(*x) & opened_mask == 2_u32.pow(*x))
            .map(|x| map.get(&(x as u32)).unwrap().flow)
            .sum::<u32>();

        // some optimization
        if max_flow == opened_mask {
            // everything is open, increase time and score only
            let mut new_score = score;
            let mut timer = time;

            while timer < 26 {
                new_score += increase;
                timer += 1;
            }
            queue.push((timer + 1, lm, le, new_score, opened_mask));
        }

        // i open
        if map.get(&lm).unwrap().flow > 0 && 2_u32.pow(lm) & opened_mask == 0 {
            let next_mask = opened_mask | 2_u32.pow(lm);

            // elephant also opens
            if map.get(&le).unwrap().flow > 0 && 2_u32.pow(le) & next_mask == 0 {
                let next_mask = next_mask | 2_u32.pow(le);
                let next_score = score
                    + (0..mlen)
                        .filter(|x| 2_u32.pow(*x) & next_mask == 2_u32.pow(*x))
                        .map(|x| map.get(&x).unwrap().flow)
                        .sum::<u32>();
                queue.push((time + 1, lm, le, next_score, next_mask));
            }

            // elephant goes
            for n in map.get(&le).unwrap().to.iter() {
                let next_score = score
                    + (0..mlen)
                        .filter(|x| 2_u32.pow(*x) & next_mask == 2_u32.pow(*x))
                        .map(|x| map.get(&x).unwrap().flow)
                        .sum::<u32>();
                queue.push((time + 1, lm, *n, next_score, next_mask));
            }
        }

        // i go
        for n in map.get(&lm).unwrap().to.iter() {
            // elephant opens
            if map.get(&le).unwrap().flow > 0 && 2_u32.pow(le) & opened_mask == 0 {
                let next_mask = opened_mask | 2_u32.pow(le);
                let next_score = score
                    + (0..mlen)
                        .filter(|x| 2_u32.pow(*x) & next_mask == 2_u32.pow(*x))
                        .map(|x| map.get(&x).unwrap().flow)
                        .sum::<u32>();
                queue.push((time + 1, *n, le, next_score, next_mask));
            }

            // elephant goes
            for m in map.get(&le).unwrap().to.iter() {
                let next_score = score
                    + (0..mlen)
                        .filter(|x| 2_u32.pow(*x) & opened_mask == 2_u32.pow(*x))
                        .map(|x| map.get(&x).unwrap().flow)
                        .sum::<u32>();
                queue.push((time + 1, *n, *m, next_score, opened_mask));
            }
        }
    }

    Some(best)
}
fn main() {
    let input = &aoc::read_file("inputs", 16);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 16);
        assert_eq!(part_one(&input), Some(1651));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
