use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<String> {
    let mut decimal: i64 = input
        .lines()
        .map(|x| {
            x.chars()
                .rev()
                .enumerate()
                .map(|(idy, y)| match y {
                    '-' => -(5_i64.pow(idy as u32)),
                    '=' => -2 * 5_i64.pow(idy as u32),
                    z => 5_i64.pow(idy as u32) * z.to_digit(10).unwrap() as i64,
                })
                .sum::<i64>()
        })
        .sum();

    let mut snafu = VecDeque::new();

    while decimal > 0 {
        decimal += 2;
        snafu.push_front(decimal % 5);
        decimal /= 5;
    }

    Some(
        snafu
            .iter()
            .map(|x| match x {
                0 => '=',
                1 => '-',
                x => char::from_digit(*x as u32 - 2, 10).unwrap(),
            })
            .collect(),
    )
}
fn main() {
    let input = &aoc::read_file("inputs", 25);
    aoc::solve!(1, part_one, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".to_string()));
    }
}
