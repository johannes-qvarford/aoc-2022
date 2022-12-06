use std::collections::VecDeque;

use color_eyre::eyre::Result;
use itertools::Itertools;

pub(crate) type Input = String;

pub(crate) type Output = usize;

pub(crate) fn parse(s: &str) -> Result<Input> {
    Ok(s.to_owned())
}

pub(crate) fn part1(input: &Input) -> Output {
    let entry = input
        .chars()
        .tuple_windows::<(char, char, char, char)>()
        .enumerate()
        .find(|&(_, (a, b, c, d))| [a, b, c, d].into_iter().all_unique())
        .expect("There should be at least one unique sequence of 4 chars");
    entry.0 + 4
}

pub(crate) fn part2(input: &Input, marker_len: usize) -> Output {
    fn shrink(a: u8) -> usize {
        (a - b'a') as usize
    }

    let buffer_size = marker_len;
    let mut buffer = VecDeque::new();
    let bytes = input.as_bytes();

    let mut byte_counts = [0usize; 26];
    let mut duplicates = 0;

    for byte in bytes.iter().take(buffer_size) {
        let shrunk = shrink(*byte);
        buffer.push_back(shrunk);
        byte_counts[shrunk] += 1;

        if byte_counts[shrunk] > 1 {
            duplicates += 1;
        }
    }

    for (i, shrunk) in bytes[buffer_size..].iter().map(|b| shrink(*b)).enumerate() {
        byte_counts[shrunk] += 1;
        buffer.push_back(shrunk);
        if byte_counts[shrunk] != 1 {
            duplicates += 1;
        }

        let previous = buffer
            .pop_front()
            .expect("We always push at least as much as we pop");
        byte_counts[previous] -= 1;

        if byte_counts[previous] != 0 {
            duplicates -= 1;

            if duplicates == 0 {
                return i + 1 + buffer_size;
            }
        }
    }

    unimplemented!()
}

pub(crate) const INPUT_STR: &str = include_str!("_input");

#[cfg(test)]
const EXAMPLE_STR: &str = include_str!("_example");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_STR).unwrap()), 7)
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(&parse(INPUT_STR).unwrap()), 1892)
    }

    #[test]
    fn part2_example_small() {
        assert_eq!(part2(&parse("aaabcaa").unwrap(), 3), 5)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_STR).unwrap(), 14), 19)
    }

    #[test]
    fn part2_example2() {
        assert_eq!(
            part2(&parse("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 14),
            23
        )
    }

    #[test]
    fn part2_example3() {
        assert_eq!(
            part2(&parse("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 14),
            23
        )
    }

    #[test]
    fn part2_example4() {
        assert_eq!(
            part2(&parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 14),
            29
        )
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&parse(INPUT_STR).unwrap(), 14), 2313)
    }
}
