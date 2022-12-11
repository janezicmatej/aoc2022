use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    operation: String,
    test: i64,
    next: (usize, usize),
    inspections: i64,
}

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        let (s, o, t, n1, n2) = value.split('\n').skip(1).next_tuple().unwrap();
        let items = s
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();

        let test = t.split(' ').last().unwrap().parse().unwrap();
        let next = [n1, n2]
            .iter()
            .map(|x| x.split(' ').last().unwrap().parse().unwrap())
            .collect_tuple()
            .unwrap();

        let operation = o.split(" = ").last().unwrap().to_string();

        Monkey {
            items,
            operation,
            test,
            next,
            inspections: 0,
        }
    }
}

fn monkey_rounds(input: &str, rounds: usize, stress_relief: bool) -> Option<i64> {
    let mut monkeys = input.split("\n\n").map(Monkey::from).collect_vec();
    let mut new_items = vec![VecDeque::new(); monkeys.len()];
    let constraint: i64 = monkeys.iter().fold(1, |acc, x| acc * x.test);

    for _ in 0..rounds {
        for (idm, monke) in monkeys.iter_mut().enumerate() {
            monke.items.append(&mut new_items[idm]);
            for item in monke.items.drain(..) {
                let mut new = match monke.operation.split(' ').next_tuple().unwrap() {
                    (_, "+", x) => {
                        if let "old" = x {
                            item + item
                        } else {
                            item + x.parse::<i64>().unwrap()
                        }
                    }
                    (_, "*", x) => {
                        if let "old" = x {
                            item * item
                        } else {
                            item * x.parse::<i64>().unwrap()
                        }
                    }
                    (_, _, _) => unreachable!(),
                };

                if stress_relief {
                    new /= 3;
                }
                new %= constraint;
                monke.inspections += 1;
                if new % monke.test == 0 {
                    new_items[monke.next.0].push_back(new);
                } else {
                    new_items[monke.next.1].push_back(new);
                }
            }
        }
    }
    monkeys.sort_by_key(|m| 0 - m.inspections);
    Some(monkeys[0].inspections * monkeys[1].inspections)
}

pub fn part_one(input: &str) -> Option<i64> {
    monkey_rounds(input, 20, true)
}
pub fn part_two(input: &str) -> Option<i64> {
    monkey_rounds(input, 10000, false)
}
fn main() {
    let input = &aoc::read_file("inputs", 11);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 11);
        assert_eq!(part_one(&input), Some(10605));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
