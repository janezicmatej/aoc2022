use hashbrown::HashSet;

fn marker_length(input: &str, length: usize) -> Option<u32> {
    for i in 0..input.len() {
        if HashSet::<char>::from_iter(input.chars().skip(i).take(length)).len() == length {
            return Some(i as u32 + length as u32);
        }
    }
    unreachable!()
}

pub fn part_one(input: &str) -> Option<u32> {
    marker_length(input, 4)
}
pub fn part_two(input: &str) -> Option<u32> {
    marker_length(input, 14)
}
fn main() {
    let input = &aoc::read_file("inputs", 6);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 6);
        assert_eq!(part_one(&input.trim()), Some(10));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 6);
        assert_eq!(part_two(&input.trim()), Some(29));
    }
}
