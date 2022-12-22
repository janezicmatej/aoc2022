use hashbrown::HashMap;
use Operation::*;
use Value::*;

enum Value {
    Said(u64),
    Heard(String),
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        match value.parse::<u64>() {
            Ok(x) => Said(x),
            Err(_) => Heard(value.to_string()),
        }
    }
}

enum Operation {
    Say(Value),
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        if let Some((a, b)) = value.split_once(" + ") {
            return Add(Value::from(a), Value::from(b));
        }
        if let Some((a, b)) = value.split_once(" - ") {
            return Sub(Value::from(a), Value::from(b));
        }
        if let Some((a, b)) = value.split_once(" * ") {
            return Mul(Value::from(a), Value::from(b));
        }
        if let Some((a, b)) = value.split_once(" / ") {
            return Div(Value::from(a), Value::from(b));
        }

        Say(Value::from(value))
    }
}

fn monkey_dfs(start: &Value, actions: &HashMap<String, Operation>) -> Option<u64> {
    match start {
        Said(x) => Some(*x),
        Heard(heard) => match actions.get(heard)? {
            Say(x) => monkey_dfs(x, actions),
            Add(x, y) => Some(monkey_dfs(x, actions)? + monkey_dfs(y, actions)?),
            Sub(x, y) => Some(monkey_dfs(x, actions)? - monkey_dfs(y, actions)?),
            Div(x, y) => Some(monkey_dfs(x, actions)? / monkey_dfs(y, actions)?),
            Mul(x, y) => Some(monkey_dfs(x, actions)? * monkey_dfs(y, actions)?),
        },
    }
}

fn reverse_monkey_dfs(start: &Value, target: u64, actions: &HashMap<String, Operation>) -> u64 {
    let left_right_rec = |x, y, left_f: fn(u64, u64) -> u64, right_f: fn(u64, u64) -> u64| match (
        monkey_dfs(x, actions),
        monkey_dfs(y, actions),
    ) {
        (Some(n), None) => reverse_monkey_dfs(y, left_f(n, target), actions),
        (None, Some(n)) => reverse_monkey_dfs(x, right_f(n, target), actions),
        (_, _) => unreachable!(),
    };

    match start {
        Said(_) => target,
        Heard(heard) => match actions.get(heard) {
            Some(value) => match value {
                Say(x) => reverse_monkey_dfs(x, target, actions),
                Add(x, y) => left_right_rec(x, y, |x, t| t - x, |x, t| t - x),
                Sub(x, y) => left_right_rec(x, y, |x, t| x - t, |x, t| t + x),
                Mul(x, y) => left_right_rec(x, y, |x, t| t / x, |x, t| t / x),
                Div(x, y) => left_right_rec(x, y, |x, t| x / t, |x, t| t * x),
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
    monkey_dfs(&Heard("root".to_string()), &actions)
}
pub fn part_two(input: &str) -> Option<u64> {
    let mut actions = HashMap::new();
    for (monke, action) in input.lines().map(|x| x.split_once(": ").unwrap()) {
        actions.insert(monke.to_string(), Operation::from(action));
    }

    actions.remove("humn");

    match &actions["root"] {
        Add(x, y) | Sub(x, y) | Mul(x, y) | Div(x, y) => {
            match (monkey_dfs(x, &actions), monkey_dfs(y, &actions)) {
                (Some(n), None) => Some(reverse_monkey_dfs(y, n, &actions)),
                (None, Some(n)) => Some(reverse_monkey_dfs(x, n, &actions)),
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
