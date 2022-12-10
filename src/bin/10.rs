use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut value = 1;
    let mut register_history = vec![1];
    for line in input.lines() {
        match &line[..4] {
            "noop" => register_history.push(value),
            "addx" => {
                register_history.extend(vec![value, value]);
                value += line.split(' ').nth(1)?.parse::<i32>().unwrap();
            }
            _ => unreachable!(),
        }
    }
    let interesting: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    Some(
        interesting
            .iter()
            .map(|x| *x as i32 * register_history[*x])
            .sum::<i32>() as u32,
    )
}
pub fn part_two(input: &str) -> Option<String> {
    let mut value = 1;
    let mut register_history = vec![];
    for line in input.lines() {
        match &line[..4] {
            "noop" => register_history.push(value),
            "addx" => {
                register_history.extend(vec![value, value]);
                value += line.split(' ').nth(1)?.parse::<i32>().unwrap();
            }
            _ => unreachable!(),
        }
    }

    Some(
        (0..6)
            .map(|x| {
                (x * 40..(x + 1) * 40)
                    .map(|y| {
                        if ((y % 40) as i32 - register_history[y]).abs() <= 1 {
                            '\u{2588}'
                        } else {
                            ' '
                        }
                    })
                    .join("")
            })
            .join("\n"),
    )
}
fn main() {
    let input = &aoc::read_file("inputs", 10);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 10);
        assert_eq!(part_one(&input), Some(13140));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 10);
        assert_eq!(
            part_two(&input),
            Some(
                "\
                    ██  ██  ██  ██  ██  ██  ██  ██  ██  ██  \n\
                    ███   ███   ███   ███   ███   ███   ███ \n\
                    ████    ████    ████    ████    ████    \n\
                    █████     █████     █████     █████     \n\
                    ██████      ██████      ██████      ████\n\
                    ███████       ███████       ███████     \
                    "
                .to_string()
            )
        );
    }
}
