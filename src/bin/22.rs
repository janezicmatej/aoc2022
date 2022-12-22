use std::{
    collections::HashMap,
    iter::{from_fn, once},
};

use Direction::*;
use Rotation::*;

enum Rotation {
    Left,
    Right,
}

fn rotate(direction: (isize, isize), rotation: &Rotation) -> (isize, isize) {
    match rotation {
        Right => (-direction.1, direction.0),
        Left => (direction.1, -direction.0),
    }
}

fn evaluate_direction(direction: (isize, isize)) -> usize {
    match direction {
        (1, 0) => 0,
        (0, -1) => 1,
        (-1, 0) => 2,
        (0, 1) => 3,
        (_, _) => unreachable!(),
    }
}

enum Direction {
    Move(usize),
    Rotate(Rotation),
}

fn parse_direction(directions: &str) -> Vec<Direction> {
    let mut directions_vec = Vec::new();
    let mut chars = directions.chars();
    let mut buf = None;
    while let Some(c) = buf.take().or_else(|| chars.next()) {
        match c {
            '0'..='9' => {
                let literal = once(c)
                    .chain(from_fn(|| {
                        buf = chars.next();
                        buf.filter(|cc| cc.is_ascii_digit())
                    }))
                    .collect::<String>()
                    .parse()
                    .unwrap();
                directions_vec.push(Move(literal));
            }
            x => directions_vec.push(Rotate(match x {
                'L' => Left,
                'R' => Right,
                _ => unreachable!(),
            })),
        }
    }
    directions_vec
}

fn parse_board(board: &str) -> HashMap<(usize, usize), bool> {
    let mut map = HashMap::new();
    for (idl, line) in board.lines().enumerate() {
        for (idc, c) in line.chars().enumerate() {
            match c {
                ' ' => (),
                '.' => {
                    map.insert((idc, idl), true);
                }
                '#' => {
                    map.insert((idc, idl), false);
                }
                _ => unreachable!(),
            }
        }
    }

    map
}

fn wrap_part_one(
    d: &(isize, isize),
    at: &(usize, usize),
    board: &HashMap<(usize, usize), bool>,
) -> (usize, usize) {
    match d {
        (1, 0) => (
            board
                .iter()
                .filter(|(x, _)| x.1 == at.1)
                .map(|(x, _)| x.0)
                .min()
                .unwrap(),
            at.1,
        ),
        (0, -1) => (
            at.0,
            board
                .iter()
                .filter(|(x, _)| x.0 == at.0)
                .map(|(x, _)| x.1)
                .max()
                .unwrap(),
        ),
        (-1, 0) => (
            board
                .iter()
                .filter(|(x, _)| x.1 == at.1)
                .map(|(x, _)| x.0)
                .max()
                .unwrap(),
            at.1,
        ),
        (0, 1) => (
            at.0,
            board
                .iter()
                .filter(|(x, _)| x.0 == at.0)
                .map(|(x, _)| x.1)
                .min()
                .unwrap(),
        ),
        (_, _) => unreachable!(),
    }
}

fn wrap_part_two(
    d: &(isize, isize),
    at: &(usize, usize),
    board: &HashMap<(usize, usize), bool>,
) -> ((usize, usize), (isize, isize)) {
    // hardcoded mappings for specific cube for part 2 (pretty sure cube shape is the same for
    // everyone)
    let (at, d) = match ((at.0 / 50, at.1 / 50), d) {
        ((1, 0), (-1, 0)) => ((99, 149 - at.1), (1, 0)),
        ((1, 0), (0, -1)) => ((49, at.0 + 100), (1, 0)),
        ((2, 0), (0, -1)) => ((at.0 - 100, 100), (0, -1)),
        ((2, 0), (1, 0)) => ((0, 149 - at.1), (-1, 0)),
        ((2, 0), (0, 1)) => ((50, at.0 - 50), (-1, 0)),
        ((1, 1), (-1, 0)) => ((at.1 - 50, 199), (0, 1)),
        ((1, 1), (1, 0)) => ((at.1 + 50, 0), (0, -1)),
        ((0, 2), (-1, 0)) => ((149, 149 - at.1), (1, 0)),
        ((0, 2), (0, -1)) => ((99, at.0 + 50), (1, 0)),
        ((1, 2), (1, 0)) => ((50, 149 - at.1), (-1, 0)),
        ((1, 2), (0, 1)) => ((0, at.0 + 100), (-1, 0)),
        ((0, 3), (-1, 0)) => ((at.1 - 100, 149), (0, 1)),
        ((0, 3), (0, 1)) => ((at.0 + 100, 49), (0, 1)),
        ((0, 3), (1, 0)) => ((at.1 - 100, 0), (0, -1)),
        _ => unreachable!(),
    };

    (wrap_part_one(&d, &at, board), d)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (board, directions) = input.split_once("\n\n").unwrap();

    let directions = parse_direction(directions.trim());
    let board = parse_board(board);

    let mut d = (1, 0);
    let mut at = (
        board
            .iter()
            .filter(|(x, y)| x.1 == 0 && **y)
            .map(|(x, _)| x.0)
            .min()
            .unwrap(),
        0,
    );

    for direction in directions.iter() {
        match direction {
            Rotate(x) => d = rotate(d, x),
            Move(x) => {
                for _ in 0..*x {
                    let (nx, ny) = (
                        (at.0 as isize + d.0) as usize,
                        (at.1 as isize + d.1) as usize,
                    );
                    match board.get(&(nx, ny)) {
                        Some(true) => at = (nx, ny),
                        Some(false) => break,
                        None => {
                            let nat = wrap_part_one(&d, &at, &board);
                            if *board.get(&nat).unwrap() {
                                at = nat;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    Some(1000 * (at.1 + 1) + 4 * (at.0 + 1) + evaluate_direction(d))
}
pub fn part_two(input: &str) -> Option<usize> {
    let (board, directions) = input.split_once("\n\n").unwrap();

    let directions = parse_direction(directions.trim());
    let board = parse_board(board);

    let mut d = (1, 0);
    let mut at = (
        board
            .iter()
            .filter(|(x, y)| x.1 == 0 && **y)
            .map(|(x, _)| x.0)
            .min()
            .unwrap(),
        0,
    );

    for direction in directions.iter() {
        match direction {
            Rotate(x) => d = rotate(d, x),
            Move(x) => {
                for _ in 0..*x {
                    let (nx, ny) = (
                        (at.0 as isize + d.0) as usize,
                        (at.1 as isize + d.1) as usize,
                    );
                    match board.get(&(nx, ny)) {
                        Some(true) => at = (nx, ny),
                        Some(false) => break,
                        None => {
                            let (nat, nd) = wrap_part_two(&d, &at, &board);
                            if *board.get(&nat).unwrap() {
                                at = nat;
                                d = nd;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    Some(1000 * (at.1 + 1) + 4 * (at.0 + 1) + evaluate_direction(d))
}
fn main() {
    let input = &aoc::read_file("inputs", 22);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 22);
        assert_eq!(part_one(&input), Some(6032));
    }
    #[test]
    fn test_rotation() {
        // left
        assert_eq!(rotate((1, 0), &Right), (0, 1));
        assert_eq!(rotate((0, 1), &Right), (-1, 0));
        assert_eq!(rotate((-1, 0), &Right), (0, -1));
        assert_eq!(rotate((0, -1), &Right), (1, 0));
        // right
        assert_eq!(rotate((1, 0), &Left), (0, -1));
        assert_eq!(rotate((0, -1), &Left), (-1, 0));
        assert_eq!(rotate((-1, 0), &Left), (0, 1));
        assert_eq!(rotate((0, 1), &Left), (1, 0));
    }
}
