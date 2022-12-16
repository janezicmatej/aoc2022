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

trait BitMask {
    // bitwise or
    fn or(&self, n: u32) -> Self;

    // bitwise and
    fn and(&self, n: u32) -> Self;

    fn contains(&self, n: u32) -> bool;

    fn max_flow(mask_len: u32) -> Self;
}

type Mask = u64;

impl BitMask for Mask {
    fn or(&self, n: u32) -> Self {
        self | (2 as Mask).pow(n)
    }

    fn and(&self, n: u32) -> Self {
        self & (2 as Mask).pow(n)
    }

    fn contains(&self, n: u32) -> bool {
        self.and(n) == 0.or(n)
    }

    fn max_flow(mask_len: u32) -> Self {
        (0..mask_len).map(|x| (2 as Mask).pow(x)).sum()
    }
}

fn parse_graph(input: &str) -> (HashMap<u32, Valve>, (u32, u32)) {
    let mut inner = HashMap::new();
    let mut counter_flow = 0;
    let mut counter_other = Mask::BITS;

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

    (map, (*inner.get("AA").unwrap(), counter_flow - 1))
}

fn get_flow(mask: Mask, mask_len: u32, map: &HashMap<u32, Valve>) -> u32 {
    (0..=mask_len)
        .filter(|x| mask.contains(*x))
        .filter_map(|x| map.get(&x))
        .map(|x| x.flow)
        .sum::<u32>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, (start, mask_len)) = parse_graph(input);

    let mut queue = vec![(1, start, 0, 0)];
    let mut memo = HashMap::new();
    let mut best = 0;

    while !queue.is_empty() {
        let (time, me, score, mask) = queue.pop().unwrap();

        if let Some(x) = memo.get(&(time, me)) {
            if x >= &score {
                continue;
            }
        }

        memo.insert((time, me), score);

        if time == 30 {
            best = max(best, score);
            continue;
        }

        // open here
        if map.get(&me).unwrap().flow > 0 && !mask.contains(me) {
            let new_mask = mask.or(me);
            let new_score = score + get_flow(new_mask, mask_len, &map);

            queue.push((time + 1, me, new_score, new_mask));
        }

        // neighbours
        for n in map.get(&me).unwrap().to.iter() {
            let new_score = score + get_flow(mask, mask_len, &map);
            queue.push((time + 1, *n, new_score, mask));
        }
    }

    Some(best)
}
pub fn part_two(input: &str) -> Option<u32> {
    let (map, (start, mask_len)) = parse_graph(input);

    let mut queue = vec![(1, start, start, 0, 0)];
    let mut memo = HashMap::new();
    let mut best = 0;

    let max_flow = Mask::max_flow(mask_len);

    while !queue.is_empty() {
        let (time, me, you, score, mask) = queue.pop().unwrap();

        if let Some(x) = memo.get(&(time, me, you)) {
            if x >= &score {
                continue;
            }
        }

        memo.insert((time, me, you), score);

        if time == 26 {
            best = max(best, score);
            continue;
        }

        let increase = get_flow(mask, mask_len, &map);

        if max_flow == mask {
            let mut new_score = score;
            let mut timer = time;

            while timer < 26 {
                new_score += increase;
                timer += 1;
            }
            queue.push((timer + 1, me, you, new_score, mask));
        }

        if map.get(&me).unwrap().flow > 0 && !mask.contains(me) {
            let new_mask = mask.or(me);

            if map.get(&you).unwrap().flow > 0 && !mask.contains(you) {
                let new_mask = new_mask.or(you);
                let new_score = score + get_flow(new_mask, mask_len, &map);

                queue.push((time + 1, me, you, new_score, new_mask));
            }

            for n in map.get(&you).unwrap().to.iter() {
                let new_score = score + get_flow(new_mask, mask_len, &map);

                queue.push((time + 1, me, *n, new_score, new_mask));
            }
        }

        for n in map.get(&me).unwrap().to.iter() {
            if map.get(&you).unwrap().flow > 0 && !mask.contains(you) {
                let new_mask = mask.or(you);
                let new_score = score + get_flow(new_mask, mask_len, &map);

                queue.push((time + 1, *n, you, new_score, new_mask));
            }

            for m in map.get(&you).unwrap().to.iter() {
                let new_score = score + get_flow(mask, mask_len, &map);

                queue.push((time + 1, *n, *m, new_score, mask));
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
