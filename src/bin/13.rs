use itertools::Itertools;
use std::{
    cmp::Ordering,
    iter::{from_fn, once},
};
use Packet::*;

#[derive(Eq, Debug, Clone)]
enum Packet {
    Literal(u32),
    List(Vec<Packet>),
}

impl From<&str> for Packet {
    fn from(value: &str) -> Self {
        let mut stack: Vec<Vec<Packet>> = Vec::new();
        let mut chars = value.chars();
        let mut buf = None;
        while let Some(c) = buf.take().or_else(|| chars.next()) {
            match c {
                ',' => (),
                '0'..='9' => {
                    let literal = once(c)
                        .chain(from_fn(|| {
                            buf = chars.next();
                            buf.filter(|cc| ('0'..='9').contains(cc))
                        }))
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    stack.last_mut().unwrap().push(Literal(literal));
                }
                '[' => stack.push(Vec::new()),
                ']' => {
                    let list = List(stack.pop().unwrap());
                    match stack.last_mut() {
                        None => return list,
                        Some(x) => x.push(list),
                    }
                }
                _ => unreachable!(),
            }
        }
        unreachable!()
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Literal(x), Literal(y)) => x.cmp(y),
            (Literal(x), List(_)) => List(vec![Literal(*x)]).cmp(other),
            (List(_), Literal(y)) => self.cmp(&List(vec![Literal(*y)])),
            (List(x), List(y)) => {
                for (xx, yy) in x.iter().zip(y.iter()) {
                    if xx == yy {
                        continue;
                    }
                    return xx.cmp(yy);
                }
                x.len().cmp(&y.len())
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(|x| x.split_once('\n').unwrap())
            .map(|(a, b)| Packet::from(a) < Packet::from(b))
            .enumerate()
            .filter(|(_, x)| *x)
            .map(|(x, _)| x as u32 + 1)
            .sum(),
    )
}
pub fn part_two(input: &str) -> Option<u32> {
    let div_a = List(vec![List(vec![Literal(2)])]);
    let div_b = List(vec![List(vec![Literal(6)])]);
    let mut all = input
        .split('\n')
        .filter(|x| x != &"")
        .map(Packet::from)
        .collect_vec();
    all.append(&mut vec![div_a.clone(), div_b.clone()]);
    all.sort();

    Some(
        (all.iter().position(|x| x == &div_a).unwrap() as u32 + 1)
            * (all.iter().position(|x| x == &div_b).unwrap() as u32 + 1),
    )
}
fn main() {
    let input = &aoc::read_file("inputs", 13);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 13);
        assert_eq!(part_one(&input), Some(13));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 13);
        assert_eq!(part_two(&input), Some(140));
    }
    #[test]
    fn test_ordering() {
        assert_eq!(Packet::from("[1,2,3,[1,2]]"), Packet::from("[1,2,3,[1,2]]"));
        assert!(Packet::from("[1,2,3,[1,2]]") < Packet::from("[1,2,3,[1,2,3]]"));
        assert!(Packet::from("[1,2,3,[1,[[2]]]]") == Packet::from("[1,2,3,[1,[2]]]"));
    }
}
