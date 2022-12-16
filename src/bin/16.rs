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
    // bitwise or on n-th bit
    fn or(&self, n: u32) -> Self;

    // bitwise and with n-th bit
    fn and(&self, n: u32) -> Self;

    // check if bit n is on
    fn contains(&self, n: u32) -> bool;

    // max flow for part 2 optimization
    fn max_flow(mask_len: u32) -> Self;
}

// type alias for integer, where bit n represents valve opened valve n u16, u32 and u64 perform
// with same speed while u128 takes around 2x longer (u8 doesn't have enough bits for all valves
// with flow > 0)
type Mask = u32;

// interestingly rust compiler does not optimize 2.pow(n) as (1 << n) and (1 << n) gains
// significant performance compared to 2.pow(n). (around 33%)
impl BitMask for Mask {
    fn or(&self, n: u32) -> Self {
        self | (1 << n)
    }

    fn and(&self, n: u32) -> Self {
        self & (1 << n)
    }

    fn contains(&self, n: u32) -> bool {
        self & (1 << n) != 0
    }

    fn max_flow(mask_len: u32) -> Self {
        (0..mask_len).map(|x| (1 << x)).sum()
    }
}

fn parse_graph(input: &str) -> (HashMap<u32, Valve>, (u32, u32)) {
    let mut inner = HashMap::new();
    let mut counter_flow = 0;
    let mut counter_other = Mask::BITS;

    let mut graph = HashMap::new();

    // first passthrough to rename vertices from strings to ints and sort them 0..Mask::BITS for
    // those with flow > 0 and Mask::BITS.. for others
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

    // second passthrough to generate graph as HashMap<u32, Valve>
    for line in input.lines() {
        let cap = RE.captures(line).unwrap();
        let from = cap[1].to_string();
        let flow: u32 = cap[2].parse().unwrap();
        let to = cap[3]
            .split(", ")
            .map(|x| *inner.get(x).unwrap())
            .collect_vec();

        graph.insert(*inner.get(&from).unwrap(), Valve { flow, to });
    }

    (graph, (*inner.get("AA").unwrap(), counter_flow - 1))
}

// calculate flow for mask on current graph, mask_len is important as stoping iteration as soon as
// possible reduces time significantly
fn get_flow(mask: Mask, mask_len: u32, graph: &HashMap<u32, Valve>) -> u32 {
    (0..=mask_len)
        .filter(|x| mask.contains(*x))
        .filter_map(|x| graph.get(&x))
        .map(|x| x.flow)
        .sum()
}

// smart dfs with memoization where we open valve if not opened then try moving in all direction
// and let memoization take care of stoping a dfs branch early
pub fn part_one(input: &str) -> Option<u32> {
    let (graph, (start, mask_len)) = parse_graph(input);

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

        // we open
        if graph.get(&me).unwrap().flow > 0 && !mask.contains(me) {
            let new_mask = mask.or(me);
            let new_score = score + get_flow(new_mask, mask_len, &graph);

            queue.push((time + 1, me, new_score, new_mask));
        }

        // we move
        for n in graph.get(&me).unwrap().to.iter() {
            let new_score = score + get_flow(mask, mask_len, &graph);
            queue.push((time + 1, *n, new_score, mask));
        }
    }

    Some(best)
}

// smart dfs with memoization where we take care of all 4 possibilities that can occur (we can open
// the valve or move and elephant can open the valve or move 2x2 = 4). We let memoization take care
// of stoping early like before
pub fn part_two(input: &str) -> Option<u32> {
    let (graph, (start, mask_len)) = parse_graph(input);

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

        // some extra optimization as all valves will be open in some cases
        if max_flow == mask {
            let mut new_score = score;
            let mut timer = time;
            let increase = get_flow(mask, mask_len, &graph);

            while timer < 26 {
                new_score += increase;
                timer += 1;
            }
            queue.push((timer, me, you, new_score, mask));
        }

        // we open
        if graph.get(&me).unwrap().flow > 0 && !mask.contains(me) {
            let new_mask = mask.or(me);

            // elephant opens
            if graph.get(&you).unwrap().flow > 0 && !mask.contains(you) {
                let new_mask = new_mask.or(you);
                let new_score = score + get_flow(new_mask, mask_len, &graph);

                queue.push((time + 1, me, you, new_score, new_mask));
            }

            // elephant goes
            for n in graph.get(&you).unwrap().to.iter() {
                let new_score = score + get_flow(new_mask, mask_len, &graph);

                queue.push((time + 1, me, *n, new_score, new_mask));
            }
        }

        // we go
        for n in graph.get(&me).unwrap().to.iter() {
            // elephant opens
            if graph.get(&you).unwrap().flow > 0 && !mask.contains(you) {
                let new_mask = mask.or(you);
                let new_score = score + get_flow(new_mask, mask_len, &graph);

                queue.push((time + 1, *n, you, new_score, new_mask));
            }

            // elephant goes
            for m in graph.get(&you).unwrap().to.iter() {
                let new_score = score + get_flow(mask, mask_len, &graph);

                queue.push((time + 1, *n, *m, new_score, mask));
            }
        }
    }

    Some(best)
}
fn main() {
    // further optimization ideas would be to analize graph and compress it as much as possible
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
