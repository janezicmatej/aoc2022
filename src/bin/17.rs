use itertools::Itertools;
use Shape::*;

enum Shape {
    Dash,
    Cross,
    InvertedL,
    Vertical,
    Square,
}

impl Shape {
    fn origin(&self, height: usize) -> (usize, usize) {
        (3, height + 3)
    }

    fn points(&self) -> Vec<(usize, usize)> {
        match self {
            Dash => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Cross => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            InvertedL => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Vertical => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Square => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }

    fn points_relative(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        self.points()
            .iter()
            .map(|x| (x.0 + position.0, x.1 + position.1))
            .collect_vec()
    }

    fn order() -> Vec<Self> {
        vec![Dash, Cross, InvertedL, Vertical, Square]
    }
}

fn fall_rocks(input: &str, t: usize) -> Option<usize> {
    let mut cavern = vec![vec![true]; 9];
    let mut jet = input.trim().chars().cycle();

    for shape in Shape::order().iter().cycle().take(t) {
        let height = cavern[0].len();
        let mut origin = shape.origin(height);

        loop {
            let new_origin = match jet.next()? {
                '<' => (origin.0 - 1, origin.1),
                '>' => (origin.0 + 1, origin.1),
                _ => unreachable!(),
            };

            // jet push
            if !shape
                .points_relative(new_origin)
                .iter()
                .map(|(x, y)| if *x == 0 || *x == 8 { (0, 0) } else { (*x, *y) })
                .filter(|(_, y)| y < &height)
                .any(|(x, y)| cavern[x][y])
            {
                origin = new_origin;
            }

            let new_origin = (origin.0, origin.1 - 1);

            // downwards movement
            if !shape
                .points_relative(new_origin)
                .iter()
                .map(|(x, y)| if *x == 0 || *x == 8 { (0, 0) } else { (*x, *y) })
                .filter(|(_, y)| y < &height)
                .any(|(x, y)| cavern[x][y])
            {
                origin = new_origin;
            } else {
                // can't move down, time for next shape
                let increased_height = shape
                    .points_relative(origin)
                    .iter()
                    .map(|x| x.1)
                    .max()
                    .unwrap();

                while cavern[0].len() <= increased_height {
                    for column in cavern.iter_mut() {
                        column.push(false);
                    }
                    *cavern.first_mut()?.last_mut()? = true;
                    *cavern.last_mut()?.last_mut()? = true;
                }

                for (px, py) in shape.points_relative(origin) {
                    cavern[px][py] = true;
                }
                break;
            }
        }
    }

    Some(cavern[0].len() - 1)
}

pub fn part_one(input: &str) -> Option<usize> {
    fall_rocks(input, 2022)
}
pub fn part_two(input: &str) -> Option<usize> {
    const THROWS: usize = 1_000_000_000_000;
    // 2550 is when my thingy cycles and after that on every 1725th shape
    let start = fall_rocks(input, 2550).unwrap();
    let repeat = fall_rocks(input, 2550 + 1725).unwrap() - start;
    let rest = fall_rocks(input, 2550 + 775).unwrap() - start;
    Some(start + rest + ((THROWS - 2550) / 1725) * repeat)
}
fn main() {
    let input = &aoc::read_file("inputs", 17);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 17);
        assert_eq!(part_one(&input), Some(3068));
    }
}
