use hashbrown::HashSet;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r#"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"#)
            .unwrap();
}

fn distance(x: &(isize, isize), y: &(isize, isize)) -> isize {
    (x.0 - y.0).abs() + (x.1 - y.1).abs()
}

pub fn part_one(input: &str) -> Option<usize> {
    const LINE: isize = 4000000;
    let mut set = HashSet::new();
    let mut beacons = HashSet::new();
    for line in input.lines() {
        let (x1, y1, x2, y2) = RE
            .captures(line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().parse::<isize>().unwrap())
            .next_tuple()
            .unwrap();
        if y2 == LINE {
            beacons.insert(x2);
        }
        let d = (x1 - x2).abs() + (y1 - y2).abs();
        if (y1 - LINE).abs() <= d {
            let extra = d - (y1 - LINE).abs();
            for i in x1 - extra..=x1 + extra {
                set.insert(i);
            }
        }
    }
    Some(set.difference(&beacons).count())
}

fn walk(p: (isize, isize), r: isize, limit: isize) -> Vec<(isize, isize)> {
    let mut v = vec![(p.0, p.1 - r)];
    for d in [(-1, 1), (1, 1), (1, -1), (-1, -1)].iter() {
        for _ in 0..r {
            let l = v.last().unwrap();
            let p = (l.0 + d.0, l.1 + d.1);
            if p.0 < 0 || p.0 > limit || p.1 < 0 || p.1 > limit {
                continue;
            }
            v.push((l.0 + d.0, l.1 + d.1));
        }
    }
    v.pop();
    v
}

pub fn part_two(input: &str) -> Option<isize> {
    const LIMIT: isize = 4000000;
    let mut pairs = HashSet::new();
    for line in input.lines() {
        let (x1, y1, x2, y2) = RE
            .captures(line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().parse::<isize>().unwrap())
            .next_tuple()
            .unwrap();
        pairs.insert(((x1, y1), (x2, y2)));
    }
    for (s, b) in pairs.iter() {
        for p in walk(*s, distance(s, b) + 1, LIMIT) {
            if !pairs
                .iter()
                .filter(|(x, y)| (x, y) != (s, b))
                .any(|(s, b)| distance(s, &p) <= distance(s, b))
            {
                return Some(p.0 * 4000000 + p.1);
            }
        }
    }
    None
}
fn main() {
    let input = &aoc::read_file("inputs", 15);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 15);
        assert_eq!(part_one(&input), Some(0));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
