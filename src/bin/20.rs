use elves::parsers::vec_lines;
use itertools::Itertools;

#[derive(Debug)]
struct MixedPacket {
    mixed: isize,
    number: isize,
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut packets = vec_lines(input.trim())
        .iter()
        .enumerate()
        .map(|(idx, x)| MixedPacket {
            mixed: idx as isize,
            number: *x,
        })
        .collect_vec();

    for i in 0..packets.len() {
        let x = packets.iter().position(|x| x.mixed == i as isize).unwrap();
        let p = packets.remove(x);
        packets.insert(
            (x as isize + p.number).rem_euclid(packets.len() as isize) as usize,
            p,
        );
    }

    let zero = packets.iter().position(|x| x.number == 0).unwrap();

    Some(
        (1..=3)
            .map(|x| packets[(x * 1000 + zero) % packets.len()].number)
            .sum(),
    )
}
pub fn part_two(input: &str) -> Option<isize> {
    const DC_KEY: isize = 811589153;
    let mut packets = vec_lines::<isize>(input.trim())
        .iter()
        .enumerate()
        .map(|(idx, x)| MixedPacket {
            mixed: idx as isize,
            number: *x * DC_KEY,
        })
        .collect_vec();

    for _ in 0..10 {
        for i in 0..packets.len() {
            let x = packets.iter().position(|x| x.mixed == i as isize).unwrap();
            let p = packets.remove(x);
            packets.insert(
                (x as isize + p.number).rem_euclid(packets.len() as isize) as usize,
                p,
            );
        }
    }

    let zero = packets.iter().position(|x| x.number == 0).unwrap();

    Some(
        (1..=3)
            .map(|x| packets[(x * 1000 + zero) % packets.len()].number)
            .sum(),
    )
}
fn main() {
    let input = &aoc::read_file("inputs", 20);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 20);
        assert_eq!(part_one(&input), Some(3));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
