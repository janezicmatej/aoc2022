use std::ops::Not;

use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug)]
struct File {
    size: u32,
}

#[derive(Debug)]
struct Dir {
    dirs: HashMap<String, Dir>,
    files: Vec<File>,
}

impl Dir {
    fn update_from(&mut self, other: Dir) {
        self.files = other.files;
    }

    fn insert_rec(&mut self, mut stack: Vec<String>, current: &String, dir: Dir) {
        if !stack.is_empty() {
            let top = stack.pop().unwrap();
            if self.dirs.contains_key(&top).not() {
                self.dirs.insert(
                    top.to_string(),
                    Dir {
                        dirs: HashMap::new(),
                        files: Vec::new(),
                    },
                );
            }

            self.dirs
                .get_mut(&top)
                .unwrap()
                .insert_rec(stack, current, dir);
        } else {
            if self.dirs.contains_key(&current.to_string()).not() {
                self.dirs.insert(
                    current.to_string(),
                    Dir {
                        dirs: HashMap::new(),
                        files: Vec::new(),
                    },
                );
            }

            self.dirs.get_mut(current).unwrap().update_from(dir);
        }
    }
    fn rec_sum_full(&self) -> u32 {
        let files_size: u32 = self.files.iter().map(|x| x.size).sum();
        let dir_size: u32 = self.dirs.values().map(Dir::rec_sum_full).sum();
        dir_size + files_size
    }

    fn rec_sum(&self) -> u32 {
        let rc = self.dirs.values().map(Dir::rec_sum).sum();
        if self.rec_sum_full() < 100000 {
            self.rec_sum_full() + rc
        } else {
            rc
        }
    }

    fn find_smallest_gt(&self, size: u32) -> u32 {
        let rcf = self.rec_sum_full();
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
        files: Vec::new(),
    };

    let mut lines = input.lines().skip(1).peekable();

    while let Some(x) = lines.next() {
        if let '$' = x.chars().next().unwrap() {
            let split = x[2..].split(' ');
            if let "cd" = split.clone().next().unwrap() {
                match split.last().unwrap() {
                    ".." => current_dir = dir_stack.pop().unwrap(),
                    "/" => {
                        current_dir = "/".to_string();
                        dir_stack.clear();
                    }
                    x => {
                        dir_stack.push(current_dir);
                        current_dir = x.to_string();
                    }
                }
            } else {
                let mut dirs = HashMap::new();
                let mut files = Vec::new();
                while let Some(y) = lines.peek() {
                    if let '$' = y.chars().next().unwrap() {
                        break;
                    } else {
                        match lines.next().unwrap().split(' ').next_tuple().unwrap() {
                            ("dir", name) => {
                                dirs.insert(
                                    name.to_string(),
                                    Dir {
                                        dirs: HashMap::new(),
                                        files: Vec::new(),
                                    },
                                );
                            }
                            (size, _name) => {
                                let parsed_size = size.parse().unwrap();
                                files.push(File { size: parsed_size })
                            }
                        }
                    }
                }
                let mut rev_stack = dir_stack.to_vec();
                rev_stack.reverse();
                root.insert_rec(rev_stack, &current_dir, Dir { dirs, files });
            }
        }
    }

    root
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse_root(input).rec_sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let root = parse_root(input);
    Some(root.find_smallest_gt(root.rec_sum_full() - 40000000))
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
