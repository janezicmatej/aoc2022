use itertools::Itertools;
use Rps::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn win(&self) -> Rps {
        match self {
            Self::Rock => Paper,
            Self::Paper => Scissors,
            Self::Scissors => Rock,
        }
    }

    fn lose(&self) -> Rps {
        match self {
            Self::Rock => Scissors,
            Self::Paper => Rock,
            Self::Scissors => Paper,
        }
    }

    fn get_value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn from_ascii(ascii_code: &str) -> Rps {
        match ascii_code {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => unreachable!(),
        }
    }

    fn outcome(you: &Rps, me: &Rps) -> u32 {
        if me == &you.win() {
            return 6;
        } else if me == &you.lose() {
            return 0;
        }
        3
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score = 0;
    for line in input.trim().split('\n') {
        let (you, me) = line.split(' ').tuple_windows().next().unwrap();
        let rps_y = Rps::from_ascii(you);
        let rps_m = Rps::from_ascii(me);
        score += Rps::outcome(&rps_y, &rps_m) + rps_m.get_value();
    }
    Some(score)
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut score = 0;
    for line in input.trim().split('\n') {
        let (you, me) = line.split(' ').tuple_windows().next().unwrap();
        let rps_y = Rps::from_ascii(you);
        let mut rps_m = Rps::from_ascii(me);
        match me {
            "X" => rps_m = rps_y.lose(),
            "Y" => rps_m = rps_y,
            "Z" => rps_m = rps_y.win(),
            _ => (),
        };
        let ls = Rps::outcome(&rps_y, &rps_m) + rps_m.get_value();
        score += ls;
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
