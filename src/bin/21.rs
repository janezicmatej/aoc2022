use hashbrown::HashMap;
use MonkeyValue::*;
use Operation::*;

#[derive(Debug, Clone)]
enum MonkeyValue {
    Said(u64),
    Heard(String),
}

impl From<&str> for MonkeyValue {
    fn from(value: &str) -> Self {
        match value.parse::<u64>() {
            Ok(x) => Said(x),
            Err(_) => Heard(value.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Say(MonkeyValue),
    Add(MonkeyValue, MonkeyValue),
    Substract(MonkeyValue, MonkeyValue),
    Multiply(MonkeyValue, MonkeyValue),
    Divide(MonkeyValue, MonkeyValue),
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        if let Some((a, b)) = value.split_once(" + ") {
            return Add(MonkeyValue::from(a), MonkeyValue::from(b));
        }
        if let Some((a, b)) = value.split_once(" - ") {
            return Substract(MonkeyValue::from(a), MonkeyValue::from(b));
        }
        if let Some((a, b)) = value.split_once(" * ") {
            return Multiply(MonkeyValue::from(a), MonkeyValue::from(b));
        }
        if let Some((a, b)) = value.split_once(" / ") {
            return Divide(MonkeyValue::from(a), MonkeyValue::from(b));
        }

        Say(MonkeyValue::from(value))
    }
}

fn monkey_bfs(start: &MonkeyValue, actions: &HashMap<String, Operation>) -> Option<u64> {
    match start {
        Said(x) => Some(*x),
        Heard(heard) => match actions.get(heard) {
            Some(value) => match value {
                Say(x) => monkey_bfs(x, actions),
                Add(x, y) => Some(monkey_bfs(x, actions)? + monkey_bfs(y, actions)?),
                Substract(x, y) => Some(monkey_bfs(x, actions)? - monkey_bfs(y, actions)?),
                Divide(x, y) => Some(monkey_bfs(x, actions)? / monkey_bfs(y, actions)?),
                Multiply(x, y) => Some(monkey_bfs(x, actions)? * monkey_bfs(y, actions)?),
            },
            None => None,
        },
    }
}

fn reverse_monkey_bfs(
    start: &MonkeyValue,
    target: u64,
    actions: &HashMap<String, Operation>,
) -> u64 {
    let left_right_rec = |x, y, left_f: fn(u64, u64) -> u64, right_f: fn(u64, u64) -> u64| match (
        monkey_bfs(x, actions),
        monkey_bfs(y, actions),
    ) {
        (Some(n), None) => reverse_monkey_bfs(y, left_f(n, target), actions),
        (None, Some(n)) => reverse_monkey_bfs(x, right_f(n, target), actions),
        (_, _) => unreachable!(),
    };

    match start {
        Said(_) => target,
        Heard(heard) => match actions.get(heard) {
            Some(value) => match value {
                Say(x) => reverse_monkey_bfs(x, target, actions),
                Add(x, y) => left_right_rec(x, y, |x, t| t - x, |x, t| t - x),
                Substract(x, y) => left_right_rec(x, y, |x, t| x - t, |x, t| t + x),
                Multiply(x, y) => left_right_rec(x, y, |x, t| t / x, |x, t| t / x),
                Divide(x, y) => left_right_rec(x, y, |x, t| x / t, |x, t| t * x),
            },
            None => target,
        },
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut actions = HashMap::new();
    for (monke, action) in input.lines().map(|x| x.split_once(": ").unwrap()) {
        actions.insert(monke.to_string(), Operation::from(action));
    }
    monkey_bfs(&Heard("root".to_string()), &actions)
}
pub fn part_two(input: &str) -> Option<u64> {
    let mut actions = HashMap::new();
    for (monke, action) in input.lines().map(|x| x.split_once(": ").unwrap()) {
        actions.insert(monke.to_string(), Operation::from(action));
    }

    actions.remove("humn");

    match &actions["root"] {
        Add(x, y) | Substract(x, y) | Multiply(x, y) | Divide(x, y) => {
            match (monkey_bfs(x, &actions), monkey_bfs(y, &actions)) {
                (Some(n), None) => Some(reverse_monkey_bfs(y, n, &actions)),
                (None, Some(n)) => Some(reverse_monkey_bfs(x, n, &actions)),
                (_, _) => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}
fn main() {
    let input = &aoc::read_file("inputs", 21);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 21);
        assert_eq!(part_one(&input), Some(152));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
