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
    fn opn(&self, n: u32) -> Self;

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
    fn opn(&self, n: u32) -> Self {
        self | (1 << n)
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
        let to = cap[3].split(", ").map(|x| inner[x]).collect_vec();

        graph.insert(inner[&from], Valve { flow, to });
    }

    (graph, (inner["AA"], counter_flow))
}

// calculate flow for mask on current graph, mask_len is important as stoping iteration as soon as
// possible reduces time significantly
fn get_flow(mask: Mask, mask_len: u32, graph: &HashMap<u32, Valve>) -> u32 {
    (0..mask_len)
        .filter(|x| mask.contains(*x))
        .map(|x| graph[&x].flow)
        .sum()
}

// smart dfs with memoization where we open valve if not opened then try moving in all direction
// and let memoization take care of stoping a dfs branch early
pub fn part_one(input: &str) -> Option<u32> {
    let (graph, (start, mask_len)) = parse_graph(input);

    let mut queue = vec![(1, start, 0, 0)];
    let mut memo_pos = HashMap::new();
    let mut best = 0;

    while !queue.is_empty() {
        let (time, me, score, mask) = queue.pop().unwrap();

        if let Some(x) = memo_pos.get(&(time, me)) {
            if x >= &score {
                continue;
            }
        }

        memo_pos.insert((time, me), score);

        if time == 30 {
            best = max(best, score);
            continue;
        }

        // calculate score
        let new_score = score + get_flow(mask, mask_len, &graph);

        // we open
        if graph[&me].flow > 0 && !mask.contains(me) {
            queue.push((time + 1, me, new_score + graph[&me].flow, mask.opn(me)));
        }

        // we move
        for n in graph[&me].to.iter() {
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

        let increase = get_flow(mask, mask_len, &graph);

        // some extra optimization as all valves will be open in some cases
        if max_flow == mask {
            let mut new_score = score + increase;
            let mut timer = time;

            while timer < 25 {
                new_score += increase;
                timer += 1;
            }
            queue.push((timer + 1, me, you, new_score, mask));
            continue;
        }

        let new_score = score + increase;

        // we open
        if graph[&me].flow > 0 && !mask.contains(me) {
            // elephant opens
            if graph[&you].flow > 0 && !mask.opn(me).contains(you) {
                let this_score = new_score + graph[&me].flow + graph[&you].flow;
                queue.push((time + 1, me, you, this_score, mask.opn(me).opn(you)));
            }

            // elephant goes
            for n in graph[&you].to.iter() {
                let this_score = new_score + graph[&me].flow;
                queue.push((time + 1, me, *n, this_score, mask.opn(me)));
            }
        }

        // we go
        for n in graph[&me].to.iter() {
            // elephant opens
            if graph[&you].flow > 0 && !mask.contains(you) {
                let this_score = new_score + graph[&you].flow;
                queue.push((time + 1, *n, you, this_score, mask.opn(you)));
            }

            // elephant goes
            for m in graph[&you].to.iter() {
                queue.push((time + 1, *n, *m, new_score, mask));
            }
        }
    }

    Some(best)
}
fn main() {
    // further optimization ideas would be to analize graph and compress it as much as possible
    // also this solution does not work for all cases, and I think it's because I don't take into
    // account which valves are already opened in memo
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
