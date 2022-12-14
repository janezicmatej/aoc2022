use std::cmp::{max, min};

use hashbrown::HashSet;
use itertools::Itertools;

fn throw_sand(
    start: (isize, isize),
    points: &HashSet<(isize, isize)>,
    floor: Option<isize>,
) -> Option<(isize, isize)> {
    if let Some(limit) = floor {
        if start.1 == limit - 1 {
            return Some(start);
        }
    } else if start.1 > points.iter().map(|x| x.1).max().unwrap() {
        return None;
    }

    if !points.contains(&(start.0, start.1 + 1)) {
        return throw_sand((start.0, start.1 + 1), points, floor);
    }

    if !points.contains(&(start.0 - 1, start.1 + 1)) {
        return throw_sand((start.0 - 1, start.1 + 1), points, floor);
    }

    if !points.contains(&(start.0 + 1, start.1 + 1)) {
        return throw_sand((start.0 + 1, start.1 + 1), points, floor);
    }

    Some(start)
}

fn create_caves(input: &str) -> HashSet<(isize, isize)> {
    let mut points = HashSet::new();
    for line in input.lines() {
        for ((y1, x1), (y2, x2)) in line
            .split(" -> ")
            .map(|x| {
                let (a, b) = x.split_once(',').unwrap();
                (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
            })
            .tuple_windows()
        {
            for dx in min(y1, y2)..=max(y1, y2) {
                for dy in min(x1, x2)..=max(x1, x2) {
                    points.insert((dx as isize, dy as isize));
                }
            }
        }
    }
    points
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut points = create_caves(input);
    let mut count = 0;
    while let Some(point) = throw_sand((500, 0), &points, None) {
        count += 1;
        points.insert(point);
    }
    Some(count)
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut points = create_caves(input);
    let floor = points.iter().map(|x| x.1).max().unwrap() + 2;
    let mut count = 0;
    while let Some(point) = throw_sand((500, 0), &points, Some(floor)) {
        count += 1;
        if !points.insert(point) {
            break;
        }
    }
    Some(count - 1)
}
fn main() {
    let input = &aoc::read_file("inputs", 14);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 14);
        assert_eq!(part_one(&input), Some(24));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
