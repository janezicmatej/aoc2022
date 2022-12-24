use hashbrown::{HashMap, HashSet};

fn parse_snowstorms(input: &str) -> HashMap<(isize, isize), Vec<(isize, isize)>> {
    let mut map: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
    for (idl, line) in input
        .lines()
        .skip(1)
        .enumerate()
        .take(input.lines().count() - 2)
    {
        for (idc, c) in line.chars().skip(1).enumerate().take(line.len() - 2) {
            match c {
                '.' => (),
                '>' => map
                    .entry((idl as isize, idc as isize))
                    .or_default()
                    .push((0, 1)),
                '<' => map
                    .entry((idl as isize, idc as isize))
                    .or_default()
                    .push((0, -1)),
                'v' => map
                    .entry((idl as isize, idc as isize))
                    .or_default()
                    .push((1, 0)),
                '^' => map
                    .entry((idl as isize, idc as isize))
                    .or_default()
                    .push((-1, 0)),
                _ => unreachable!(),
            };
        }
    }

    map
}

fn step_snowstorms(
    snowstorms: HashMap<(isize, isize), Vec<(isize, isize)>>,
    h: isize,
    w: isize,
) -> HashMap<(isize, isize), Vec<(isize, isize)>> {
    let mut new: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();

    for ((lx, ly), d) in snowstorms.iter() {
        for (dx, dy) in d.iter() {
            let nl = ((*lx + dx).rem_euclid(h), (*ly + dy).rem_euclid(w));
            new.entry(nl).or_default().push((*dx, *dy));
        }
    }

    new
}

fn populate_neighbours(
    visited: HashSet<(isize, isize)>,
    snowstorms: &HashMap<(isize, isize), Vec<(isize, isize)>>,
    h: isize,
    w: isize,
) -> HashSet<(isize, isize)> {
    let is_valid = |x: (isize, isize)| {
        !snowstorms.contains_key(&x)
            && (x == (-1, 0) || x == (h, w - 1) || (x.0 >= 0 && x.0 < h && x.1 >= 0 && x.1 < w))
    };

    let mut new = HashSet::new();

    for (vx, vy) in visited.iter() {
        for (dx, dy) in [(0, 0), (1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nv = ((*vx + dx), (*vy + dy));
            if is_valid(nv) {
                new.insert(nv);
            }
        }
    }

    new
}

pub fn part_one(input: &str) -> Option<u32> {
    let h = input.lines().count() as isize - 2;
    let w = input.lines().next()?.len() as isize - 2;

    let mut snowstorms = parse_snowstorms(input);
    let mut visited = HashSet::from([(-1, 0)]);

    let mut count = 0;

    while !visited.contains(&(h, w - 1)) {
        count += 1;
        snowstorms = step_snowstorms(snowstorms, h, w);
        visited = populate_neighbours(visited, &snowstorms, h, w);
    }

    Some(count)
}
pub fn part_two(input: &str) -> Option<u32> {
    let h = input.lines().count() as isize - 2;
    let w = input.lines().next()?.len() as isize - 2;

    let mut snowstorms = parse_snowstorms(input);
    let mut visited = HashSet::from([(-1, 0)]);

    let mut count = 0;

    while !visited.contains(&(h, w - 1)) {
        count += 1;
        snowstorms = step_snowstorms(snowstorms, h, w);
        visited = populate_neighbours(visited, &snowstorms, h, w);
    }
    visited = HashSet::from([(h, w - 1)]);
    while !visited.contains(&(-1, 0)) {
        count += 1;
        snowstorms = step_snowstorms(snowstorms, h, w);
        visited = populate_neighbours(visited, &snowstorms, h, w);
    }
    visited = HashSet::from([(-1, 0)]);
    while !visited.contains(&(h, w - 1)) {
        count += 1;
        snowstorms = step_snowstorms(snowstorms, h, w);
        visited = populate_neighbours(visited, &snowstorms, h, w);
    }

    Some(count)
}
fn main() {
    let input = &aoc::read_file("inputs", 24);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 24);
        assert_eq!(part_one(&input), Some(18));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 24);
        assert_eq!(part_two(&input), Some(54));
    }
}
