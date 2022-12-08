use itertools::{Either, Itertools};

fn can_see(range: &[u32], first: &u32) -> bool {
    range.iter().all(|x| x < first)
}

fn score(range: Vec<u32>, reverse: bool) -> u32 {
    let mut itr = match reverse {
        true => Either::Left(range.iter()),
        false => Either::Right(range.iter().rev()),
    };
    let first = itr.next().unwrap();
    itr.enumerate()
        .filter(|(_, &x)| x >= *first)
        .map(|(x, _)| x + 1)
        .min()
        .unwrap_or(range.len() - 1) as u32
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .split('\n')
        .map(|x| {
            x.chars()
                .map(|y| y.to_string().parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let trees = parse_input(input);

    let mut count = 0;
    for (idx, i) in trees.iter().enumerate() {
        for (idy, j) in i.iter().enumerate() {
            let up = trees[..idx].iter().map(|x| x[idy]).collect_vec();
            let down = trees[idx + 1..].iter().map(|x| x[idy]).collect_vec();
            let left = trees[idx][..idy].to_vec();
            let right = trees[idx][idy + 1..].to_vec();
            if vec![up, down, left, right].iter().any(|x| can_see(x, j)) {
                count += 1
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let trees = parse_input(input);

    let mut max_score = 0;
    for (idx, i) in trees.iter().enumerate() {
        for (idy, _) in i.iter().enumerate() {
            let up = trees[..idx + 1].iter().map(|x| x[idy]).collect_vec();
            let down = trees[idx..].iter().map(|x| x[idy]).collect_vec();
            let left = trees[idx][..idy + 1].to_vec();
            let right = trees[idx][idy..].to_vec();
            let score =
                score(up, false) * score(down, true) * score(left, false) * score(right, true);
            if score > max_score {
                max_score = score;
            }
        }
    }

    Some(max_score)
}
fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 8);
        assert_eq!(part_one(&input), Some(21));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
