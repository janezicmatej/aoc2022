use elves::{many_max, many_min, parsers::as_vec_vec};
use hashbrown::HashSet;
use itertools::any;

pub fn part_one(input: &str) -> Option<isize> {
    let cubes = HashSet::<[isize; 3]>::from_iter(
        as_vec_vec::<isize>(input.trim(), '\n', ',')
            .iter()
            .map(|x| [x[0], x[1], x[2]]),
    );

    let mut count = 0;
    for cube in cubes.iter() {
        for direction in [-1, 1] {
            for component in 0..3 {
                let mut neighbour = *cube;
                neighbour[component] += direction;
                if !cubes.contains(&neighbour) {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}
pub fn part_two(input: &str) -> Option<isize> {
    let cubes = HashSet::<[isize; 3]>::from_iter(
        as_vec_vec::<isize>(input.trim(), '\n', ',')
            .iter()
            .map(|x| [x[0], x[1], x[2]]),
    );

    let mut dfs_min: isize = isize::max_value();
    let mut dfs_max: isize = isize::min_value();

    for [x, y, z] in cubes.iter() {
        dfs_min = *many_min!(&dfs_min, x, y, z);
        dfs_max = *many_max!(&dfs_max, x, y, z);
    }

    dfs_min -= 1;
    dfs_max += 1;

    let mut queue = vec![[dfs_min, dfs_min, dfs_min]];
    let mut visited = HashSet::new();

    while !queue.is_empty() {
        let [x, y, z] = queue.pop().unwrap();
        for (dx, dy, dz) in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let (nx, ny, nz) = (x + dx, y + dy, z + dz);
            if any(vec![nx, ny, nz], |a| a > dfs_max || a < dfs_min)
                || cubes.contains(&[nx, ny, nz])
                || visited.contains(&[nx, ny, nz])
            {
                continue;
            }

            visited.insert([nx, ny, nz]);
            queue.push([nx, ny, nz]);
        }
    }

    let mut count = 0;
    for cube in cubes.iter() {
        for direction in [-1, 1] {
            for component in 0..3 {
                let mut neighbour = *cube;
                neighbour[component] += direction;
                if visited.contains(&neighbour) {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}
fn main() {
    let input = &aoc::read_file("inputs", 18);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 18);
        assert_eq!(part_one(&input), Some(64));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
