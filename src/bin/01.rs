use aoc::helpers::sum_lines;
pub fn part_one(input: &str) -> Option<u32> {
    let mut max = u32::min_value();
    for elf in input.trim().split("\n\n") {
        let current = sum_lines(elf);
        if current > max {
            max = current;
        }
    }
    Some(max)
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut max = Vec::<u32>::new();
    for elf in input.trim().split("\n\n") {
        let current = sum_lines(elf);
        max.push(current);
    }
    max.sort_by(|a, b| b.cmp(a));
    Some(max[..3].iter().sum())
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
