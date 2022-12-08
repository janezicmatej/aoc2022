use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug, Default)]
struct Dir {
    dirs: HashMap<String, Dir>,
    file_size: u32,
    size: Option<u32>,
}

impl Dir {
    pub fn is_empty(&self) -> bool {
        self.dirs.is_empty() && self.file_size == 0 && self.size.is_none()
    }

    fn insert_rec(&mut self, mut stack: Vec<String>, dir: Dir) {
        let next_name = stack.pop().unwrap();
        if !stack.is_empty() {
            self.dirs
                .entry(next_name)
                .or_default()
                .insert_rec(stack, dir);
        } else {
            self.dirs.entry(next_name).or_default().file_size = dir.file_size;
        }
    }

    fn calc_size(&mut self) -> Option<u32> {
        if self.size.is_none() {
            let dir_size: u32 = self
                .dirs
                .values_mut()
                .map(Dir::calc_size)
                .map(Option::unwrap)
                .sum();
            let total_size = dir_size + self.file_size;
            self.size = Some(total_size);
        }
        self.size
    }

    fn sum_lt(&self, constraint: u32) -> u32 {
        let recur = self.dirs.values().map(|x| x.sum_lt(constraint));
        let rc = recur.sum();

        if self.size.unwrap() < constraint {
            self.size.unwrap() + rc
        } else {
            rc
        }
    }

    fn find_smallest_gt(&self, size: u32) -> u32 {
        let rcf = self.size.unwrap();
        if rcf < size {
            return rcf;
        }

        self.dirs
            .values()
            .map(|x| x.find_smallest_gt(size))
            .filter(|x| x > &size)
            .min()
            .unwrap_or(rcf)
    }
}

fn parse_root(input: &str) -> Dir {
    let mut dir_stack = Vec::new();
    let mut current_dir = "/".to_string();
    let mut root = Dir {
        dirs: HashMap::new(),
        file_size: 0,
        size: None,
    };

    let mut ls_stack = Dir {
        dirs: HashMap::new(),
        file_size: 0,
        size: None,
    };

    for split in input.lines().map(|x| x.split_whitespace().collect_vec()) {
        match split.first().unwrap() {
            &"$" => {
                if !ls_stack.is_empty() {
                    let mut rev_stack = dir_stack.to_vec();
                    rev_stack.push(current_dir.to_string());
                    rev_stack.reverse();
                    root.insert_rec(rev_stack, ls_stack);
                    ls_stack = Dir::default();
                }
                match *split.last().unwrap() {
                    "/" => {
                        dir_stack.clear();
                        current_dir = "/".to_string();
                    }
                    ".." => current_dir = dir_stack.pop().unwrap(),
                    "ls" => (),
                    x => {
                        dir_stack.push(current_dir);
                        current_dir = x.to_string();
                    }
                }
            }
            _ => match split.iter().tuple_windows().next().unwrap() {
                (&"dir", x) => drop(ls_stack.dirs.insert(x.to_string(), Dir::default())),
                (size, _) => ls_stack.file_size += size.parse::<u32>().unwrap(),
            },
        };
    }
    // drain ls_stack
    let mut rev_stack = dir_stack.to_vec();
    rev_stack.push(current_dir);
    rev_stack.reverse();
    root.insert_rec(rev_stack, ls_stack);

    root
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut root = parse_root(input);
    root.calc_size();
    Some(root.sum_lt(100000))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut root = parse_root(input);
    root.calc_size();
    let size = root.size.unwrap() - 40000000;
    Some(root.find_smallest_gt(size))
}
fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 7);
        assert_eq!(part_one(&input), Some(95437));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
