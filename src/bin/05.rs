use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"move (\d+) from (\d+) to (\d+)"#).unwrap();
    static ref SKIP: Vec<char> = vec!['[', ' ', ']'];
}

fn parse_towers(text_towers: &str) -> Vec<Vec<char>> {
    let mut rows = text_towers.split('\n').rev();
    let num_towers = rows.next().unwrap().trim().split("   ").count();
    let mut towers = Vec::new();
    for _ in 0..num_towers {
        towers.push(Vec::new());
    }

    for row in rows {
        for (i, c) in row.chars().enumerate() {
            if !SKIP.contains(&c) {
                towers[i / 4].push(c);
            }
        }
    }

    towers
}

fn move_crates(input: &str, reverse: bool) -> Option<String> {
    let (text_towers, instructions) = input.split("\n\n").next_tuple().unwrap();
    let mut towers = parse_towers(text_towers);

    for line in instructions.trim().split('\n') {
        let captures = RE.captures(line).unwrap();
        let (amount, from, to): (usize, usize, usize) = (1..=3)
            .map(|x| captures[x].parse().unwrap())
            .tuple_windows()
            .next()
            .unwrap();

        let mut aux = Vec::new();
        for _ in 0..amount {
            let aaux = towers[from - 1].pop().unwrap();
            aux.push(aaux);
        }
        if reverse {
            aux.reverse();
        }

        towers[to - 1].append(&mut aux);
    }

    Some(String::from_iter(
        towers.iter().map(|x| x.iter().rev().next().unwrap()),
    ))
}

pub fn part_one(input: &str) -> Option<String> {
    move_crates(input, false)
}
pub fn part_two(input: &str) -> Option<String> {
    move_crates(input, true)
}
fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
