use std::ops::Not;

use aoc::helpers::ASCII;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn priority(c: char) -> u32 {
    1 + (c as u8 - 'a' as u8) as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score = 0;
    for rucksack in input.trim().split('\n') {
        let compartment_size = rucksack.len() / 2;
        let set: HashSet<char> = rucksack[..compartment_size].chars().collect();
        for item in rucksack[compartment_size..].chars() {
            if set.contains(&item) {
                score += priority(item);
                break;
            }
        }
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score = 0;
    let mut map = HashMap::with_capacity(ASCII.len());
    let mut set = HashSet::with_capacity(ASCII.len());

    for rucksacks in input.trim().split('\n').collect_vec().chunks(3) {
        map.clear();
        for rucksack in rucksacks.iter() {
            set.clear();
            for c in rucksack.chars() {
                if set.contains(&c).not() {
                    *map.entry(c).or_insert(0) += 1;
                    set.insert(c);
                };
            }
        }
        map.retain(|_, &mut v| v == 3);
        let item = map.keys().next().unwrap();
        score += priority(*item);
    }
    Some(score)
}
fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 3);
        assert_eq!(part_one(&input), Some(157));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
