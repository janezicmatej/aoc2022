use itertools::Itertools;

fn read_line(line: &str) -> ((u32, u32), (u32, u32)) {
    line.split(',')
        .map(|x| {
            x.split('-')
                .map(|y| y.parse().unwrap())
                .tuples()
                .next()
                .unwrap()
        })
        .tuples()
        .next()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score = 0;
    for line in input.trim().split('\n') {
        let ((a, b), (x, y)) = read_line(line);
        if (a >= x && b <= y) || (x >= a && y <= b) {
            score += 1;
        }
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score = 0;
    for line in input.trim().split('\n') {
        let ((a, b), (x, y)) = read_line(line);
        if (x <= a && a <= y) || (a <= x && x <= b) {
            score += 1;
        }
    }
    Some(score)
}
fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 4);
        assert_eq!(part_one(&input), Some(2));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
