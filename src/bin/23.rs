use std::cmp::{max, min};

use hashbrown::{HashMap, HashSet};

const MOVE_ORDER: [[(isize, isize); 3]; 4] = [
    // north
    [(-1, 0), (-1, -1), (-1, 1)],
    // south
    [(1, 0), (1, -1), (1, 1)],
    // west
    [(0, -1), (1, -1), (-1, -1)],
    // east
    [(0, 1), (1, 1), (-1, 1)],
];

fn parse_input(input: &str) -> HashSet<(isize, isize)> {
    let mut elves = HashSet::new();

    for (idl, line) in input.lines().enumerate() {
        for (idc, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                '#' => {
                    elves.insert((idl as isize, idc as isize));
                }
                _ => unreachable!(),
            }
        }
    }

    elves
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut elves = parse_input(input);

    for i in 0..10 {
        let mut proposals = HashMap::new();
        let mut moves = HashMap::new();

        // first half of turn
        for e in elves.iter() {
            if MOVE_ORDER
                .iter()
                .all(|x| x.iter().all(|y| !elves.contains(&(e.0 + y.0, e.1 + y.1))))
            {
                continue;
            }

            for direction in MOVE_ORDER.iter().cycle().skip(i).take(4) {
                if direction
                    .iter()
                    .all(|x| !elves.contains(&(e.0 + x.0, e.1 + x.1)))
                {
                    proposals.insert(*e, (e.0 + direction[0].0, e.1 + direction[0].1));
                    *moves
                        .entry((e.0 + direction[0].0, e.1 + direction[0].1))
                        .or_insert(0) += 1;
                    break;
                }
            }
        }

        // second half of turn
        let mut new_elves = HashSet::new();
        for e in elves.iter() {
            if proposals.contains_key(e) && moves[&proposals[e]] == 1 {
                new_elves.insert(proposals[e]);
            } else {
                new_elves.insert(*e);
            }
        }

        elves = new_elves;
    }

    let mut x = (isize::max_value(), isize::min_value());
    let mut y = (isize::max_value(), isize::min_value());

    for e in elves.iter() {
        x = (min(x.0, e.0), max(x.1, e.0));
        y = (min(y.0, e.1), max(y.1, e.1));
    }

    Some(((x.0 - x.1).abs() + 1) * ((y.0 - y.1).abs() + 1) - elves.len() as isize)
}
pub fn part_two(input: &str) -> Option<usize> {
    let mut elves = parse_input(input);

    for i in 0..usize::max_value() {
        let mut proposals = HashMap::new();
        let mut moves = HashMap::new();

        // first half of turn
        for e in elves.iter() {
            if MOVE_ORDER
                .iter()
                .all(|x| x.iter().all(|y| !elves.contains(&(e.0 + y.0, e.1 + y.1))))
            {
                continue;
            }

            for direction in MOVE_ORDER.iter().cycle().skip(i).take(4) {
                if direction
                    .iter()
                    .all(|x| !elves.contains(&(e.0 + x.0, e.1 + x.1)))
                {
                    proposals.insert(*e, (e.0 + direction[0].0, e.1 + direction[0].1));
                    *moves
                        .entry((e.0 + direction[0].0, e.1 + direction[0].1))
                        .or_insert(0) += 1;
                    break;
                }
            }
        }

        // second half of turn
        let mut new_elves = HashSet::new();
        for e in elves.iter() {
            if proposals.contains_key(e) && moves[&proposals[e]] == 1 {
                new_elves.insert(proposals[e]);
            } else {
                new_elves.insert(*e);
            }
        }

        if elves == new_elves {
            return Some(i + 1);
        }
        elves = new_elves;
    }
    None
}
fn main() {
    let input = &aoc::read_file("inputs", 23);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 23);
        assert_eq!(part_one(&input), Some(110));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
