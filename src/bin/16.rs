use hashbrown::HashMap;
use itertools::Itertools;
use std::cmp::max;

#[derive(Debug, Hash)]
struct Valve {
    flow: u32,
    to: Vec<u32>,
}

fn parse_graph(input: &str) -> (HashMap<u32, Valve>, u32) {
    let mut inner = HashMap::new();
    let mut counter = 0;

    let mut map = HashMap::new();
    for line in input.lines() {
        let (from_flow, to_flow) = line.split_once(';').unwrap();
        let from = &from_flow[6..8];
        inner.entry(from).or_insert(counter);
        counter += 1;

        let flow = from_flow[23..].parse().unwrap();
        let slice = match to_flow.contains("tunnels") {
            true => 24,
            false => 23,
        };
        let to = to_flow[slice..]
            .split(", ")
            .map(|x| {
                inner.entry(x).or_insert(counter);
                counter += 1;
                *inner.get(x).unwrap()
            })
            .collect_vec();
        map.insert(*inner.get(from).unwrap(), Valve { flow, to });
    }
    (map, *inner.get("AA").unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    // parse input
    let (map, start) = parse_graph(input);

    // smart dfs
    let mut queue: Vec<(u32, u32, u32, u128)> = vec![(1, start, 0, 0)];
    // memo: HashMap<(u32: time, u32: location), u32: score)
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
            // println!("{score}");
            best = max(best, score);
            continue;
        }

        // open here
        if map.get(&location).unwrap().flow > 0 && 2_u128.pow(location) & opened_mask == 0 {
            let next_mask = opened_mask | 2_u128.pow(location);
            // println!("{}\n{}", opened_mask, next_mask);
            let next_score = score
                + (0..128)
                    .filter(|x| 2_u128.pow(*x) & next_mask == 2_u128.pow(*x))
                    .map(|x| map.get(&x).unwrap().flow)
                    .sum::<u32>();
            queue.push((time + 1, location, next_score, next_mask));
        }

        // neighbours
        for n in map.get(&location).unwrap().to.iter() {
            let next_score = score
                + (0..128)
                    .filter(|x| 2_u128.pow(*x) & opened_mask == 2_u128.pow(*x))
                    .map(|x| map.get(&x).unwrap().flow)
                    .sum::<u32>();
            queue.push((time + 1, *n, next_score, opened_mask));
        }
    }

    // Some(max_dfs(&"AA".to_string(), 30, vec![], &map))
    Some(best)
}
pub fn part_two(input: &str) -> Option<u32> {
    // parse input
    let (map, start) = parse_graph(input);

    let mut queue: Vec<(u32, u32, u32, u32, u128)> = vec![(1, start, start, 0, 0)];
    // memo: HashMap<(u32: time, u32: location me, u32: location elephant), u32: score)
    let mut memo: HashMap<(u32, u32, u32), u32> = HashMap::new();
    let mut best = 0;
    let max_flow: u128 = (0..128)
        .filter_map(|x| map.get(&x))
        .map(|x| 2_u128.pow(x.flow))
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

        // some optimization
        if max_flow == opened_mask {
            // everything is open, increase time and score only
            let increase = (0..128)
                    .filter(|x| 2_u128.pow(*x) & opened_mask == 2_u128.pow(*x))
                    .map(|x| map.get(&x).unwrap().flow)
                    .sum::<u32>();
            let mut new_score = score;
            let mut timer = time;

            while timer < 26 {
                new_score += increase;
                timer += 1;
            }
            queue.push((timer + 1, lm, le, new_score, opened_mask));
        }

        // i open
        if map.get(&lm).unwrap().flow > 0 && 2_u128.pow(lm) & opened_mask == 0 {
            let next_mask = opened_mask | 2_u128.pow(lm);

            // elephant also opens
            if map.get(&le).unwrap().flow > 0 && 2_u128.pow(le) & next_mask == 0 {
                let next_mask = next_mask | 2_u128.pow(le);
                let next_score = score
                    + (0..128)
                        .filter(|x| 2_u128.pow(*x) & next_mask == 2_u128.pow(*x))
                        .map(|x| map.get(&x).unwrap().flow)
                        .sum::<u32>();
                queue.push((time + 1, lm, le, next_score, next_mask));
            }

            // elephant goes
            for n in map.get(&le).unwrap().to.iter() {
                let next_score = score
                    + (0..128)
                        .filter(|x| 2_u128.pow(*x) & next_mask == 2_u128.pow(*x))
                        .map(|x| map.get(&x).unwrap().flow)
                        .sum::<u32>();
                queue.push((time + 1, lm, *n, next_score, next_mask));
            }
        }

        // i go
        for n in map.get(&lm).unwrap().to.iter() {
            // elephant opens
            if map.get(&le).unwrap().flow > 0 && 2_u128.pow(le) & opened_mask == 0 {
                let next_mask = opened_mask | 2_u128.pow(le);
                let next_score = score
                    + (0..128)
                        .filter(|x| 2_u128.pow(*x) & next_mask == 2_u128.pow(*x))
                        .map(|x| map.get(&x).unwrap().flow)
                        .sum::<u32>();
                queue.push((time + 1, *n, le, next_score, next_mask));
            }

            // elephant goes
            for m in map.get(&le).unwrap().to.iter() {
                let next_score = score
                    + (0..128)
                        .filter(|x| 2_u128.pow(*x) & opened_mask == 2_u128.pow(*x))
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
