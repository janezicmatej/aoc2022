use hashbrown::HashSet;
use itertools::Itertools;
use Direction::*;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    steps: u32,
}

fn parse_input(input: &str) -> Vec<Move> {
    let mut v = Vec::new();
    for (d, s) in input.lines().map(|x| x.split(' ').next_tuple().unwrap()) {
        let steps = s.parse().unwrap();
        let direction = match d {
            "L" => Left,
            "R" => Right,
            "U" => Up,
            "D" => Down,
            _ => unreachable!(),
        };
        v.push(Move { direction, steps })
    }
    v
}

fn move_tail(tail: &mut [i32; 2], prev: &[i32; 2]) {
    let normalized = tail
        .iter()
        .zip(prev.iter())
        .map(|(t, p)| match t - p {
            0 => 0,
            _ => (p - t) / (t - p).abs(),
        })
        .collect_vec();

    if tail.iter().zip(prev.iter()).any(|(t, p)| (t - p).abs() > 1) {
        for (t, z) in tail.iter_mut().zip(normalized) {
            *t += z
        }
    }
}

fn move_head(head: &mut [i32; 2], direction: &Direction) {
    match direction {
        Left => head[0] -= 1,
        Right => head[0] += 1,
        Up => head[1] += 1,
        Down => head[1] -= 1,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let moves = parse_input(input);

    let mut head = [0; 2];
    let mut tail = [0; 2];
    let mut tail_trail = HashSet::new();
    tail_trail.insert(tail);

    for m in moves.iter() {
        for _ in 0..m.steps {
            move_head(&mut head, &m.direction);
            move_tail(&mut tail, &head);
            tail_trail.insert(tail);
        }
    }

    Some(tail_trail.len() as u32)
}
pub fn part_two(input: &str) -> Option<u32> {
    let moves = parse_input(input);

    let mut head = [0; 2];
    let mut tail = [[0; 2]; 9];
    let mut tail_trail = HashSet::new();
    tail_trail.insert(tail[8]);

    for m in moves.iter() {
        for _ in 0..m.steps {
            move_head(&mut head, &m.direction);
            move_tail(&mut tail[0], &head);
            for tail_index in 1..=8 {
                let prev = tail[tail_index - 1];
                move_tail(&mut tail[tail_index], &prev);
            }
            tail_trail.insert(tail[8]);
        }
    }

    Some(tail_trail.len() as u32)
}
fn main() {
    let input = &aoc::read_file("inputs", 9);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 9);
        assert_eq!(part_one(&input), Some(88));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
