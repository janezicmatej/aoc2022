use std::collections::VecDeque;

use hashbrown::HashSet;
use itertools::Itertools;

struct Input {
    grid: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_input(input: &str) -> Input {
    let mut grid = input
        .trim()
        .split('\n')
        .map(|x| x.chars().collect_vec())
        .collect_vec();

    let (sx, sy) = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(x, y)| grid[x][y] == 'S')
        .unwrap();
    let (ex, ey) = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(x, y)| grid[x][y] == 'E')
        .unwrap();

    grid[sx][sy] = 'a';
    grid[ex][ey] = 'z';

    Input {
        grid,
        start: (sx, sy),
        end: (ex, ey),
    }
}
fn bfs(grid: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> Option<u32> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 0));
    visited.insert(start);
    while !queue.is_empty() {
        let ((x, y), c) = queue.pop_front().unwrap();
        if (x, y) == end {
            return Some(c);
        }

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            let Some(&next) = grid.get(nx).and_then(|row| row.get(ny)) else {continue;};

            if grid[x][y] as u8 + 1 >= next as u8 && !visited.contains(&(nx, ny)) {
                visited.insert((nx, ny));
                queue.push_back(((nx, ny), c + 1));
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let Input { grid, start, end } = parse_input(input);
    bfs(&grid, start, end)
}

pub fn part_two(input: &str) -> Option<u32> {
    let Input { grid, end, .. } = parse_input(input);
    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .filter(|&(x, y)| grid[x][y] == 'a')
        .filter_map(|start| bfs(&grid, start, end))
        .min()
}
fn main() {
    let input = &aoc::read_file("inputs", 12);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 12);
        assert_eq!(part_one(&input), Some(31));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
