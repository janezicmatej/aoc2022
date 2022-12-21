use itertools::Itertools;
pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split("\n\n")
            .map(|x| to_vec(x, '\n').iter().sum())
            .max()
            .unwrap(),
    )
}
pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split("\n\n")
            .map(|x| to_vec(x, '\n').iter().sum::<u32>())
            .sorted_by(|a, b| b.cmp(a))
            .take(3)
            .sum(),
    )
}
fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 1);
        assert_eq!(part_one(&input), Some(24000));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
