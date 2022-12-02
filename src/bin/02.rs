use itertools::Itertools;
use Outcome::*;
use Shape::*;

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl From<&str> for Shape {
    fn from(code: &str) -> Self {
        match code {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => unreachable!(),
        }
    }
}

impl From<u32> for Shape {
    fn from(code: u32) -> Self {
        match code {
            1 => Rock,
            2 => Paper,
            3 => Scissors,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn value(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }

    fn play(me: &Shape, oponnent: &Shape) -> Self {
        match (me.value() as i32 - oponnent.value() as i32).rem_euclid(3) {
            0 => Draw,
            1 => Win,
            2 => Lose,
            _ => unreachable!(),
        }
    }

    fn get_shape(&self, shape: &Shape) -> Shape {
        match self {
            Draw => *shape,
            Win => Shape::from(shape.value().rem_euclid(3) + 1),
            Lose => Shape::from((shape.value() - 2).rem_euclid(3) + 1),
        }
    }
}

impl From<&str> for Outcome {
    fn from(code: &str) -> Self {
        match code {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => unreachable!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score = 0;
    for line in input.trim().split('\n') {
        let (in_oponnent, in_me) = line.split(' ').tuple_windows().next().unwrap();
        let oponnent = Shape::from(in_oponnent);
        let me = Shape::from(in_me);
        score += Outcome::play(&me, &oponnent).value() + me.value();
    }
    Some(score)
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut score = 0;
    for line in input.trim().split('\n') {
        let (in_oponnent, in_order) = line.split(' ').tuple_windows().next().unwrap();
        let oponnent = Shape::from(in_oponnent);
        let order = Outcome::from(in_order);
        score += order.value() + order.get_shape(&oponnent).value();
    }
    Some(score)
}
fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 2);
        assert_eq!(part_one(&input), Some(15));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
