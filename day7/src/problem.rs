use itertools::Itertools;
use nom::combinator::all_consuming;
use std::collections::HashMap;

use self::{
    domain::{DirectoryContent, DirectoryName, Interaction, Node, space::Space, PARENT_DIRECTORY},
    parsing::parse_interactions,
    parsing::MyResult,
};

mod domain;
mod parsing;

pub(crate) type Input = Vec<Interaction>;

pub(crate) type Output = i32;

pub(crate) fn parse(s: &str) -> MyResult<Input> {
    all_consuming(parse_interactions)(s)
    //tag("$ cd /")(s)?;
    //Ok(("", vec![]))
}

fn nodes_to_directory_content(
    parent_directory: DirectoryName,
    nodes: &Vec<Node>,
) -> DirectoryContent {
    let mut computed_space = Space::bytes(0);
    let mut uncomputed_directories: Vec<DirectoryName> = vec![];

    for node in nodes {
        match node {
            Node::Directory(directory_name) => uncomputed_directories.push(DirectoryName(format!(
                "{parent_directory}/{directory_name}"
            ))),
            Node::File(space) => computed_space += *space,
        }
    }
    DirectoryContent {
        computed_space,
        uncomputed_directories,
    }
}

fn path(v: &[DirectoryName]) -> String {
    v.iter().join("/")
}

fn build_filesystem(input: &Input) -> HashMap<DirectoryName, DirectoryContent> {
    let mut filesystem: HashMap<DirectoryName, DirectoryContent> = HashMap::new();

    let mut cwd: Vec<DirectoryName> = Vec::new();

    for interaction in input {
        match interaction {
            Interaction::Cd(directory_name) => {
                if directory_name.0 == PARENT_DIRECTORY {
                    cwd.pop();
                } else {
                    cwd.push(directory_name.clone());
                }
            }
            Interaction::Ls(nodes) => {
                let directory_name = DirectoryName(path(&cwd));

                filesystem.insert(
                    directory_name.clone(),
                    nodes_to_directory_content(directory_name, nodes),
                );
            }
        }
    }
    filesystem
}

fn space_for_directory(
    directory_name: &DirectoryName,
    filesystem: &mut HashMap<DirectoryName, DirectoryContent>,
) -> Space {
    let content = filesystem
        .get(directory_name)
        .expect("All directories can be referenced from the filesystem.")
        .clone();
    let new_space = content
        .uncomputed_directories
        .iter()
        .map(|sub_directory_name| space_for_directory(sub_directory_name, filesystem))
        .sum();
    let total_space = content.computed_space + new_space;
    filesystem.insert(
        directory_name.clone(),
        DirectoryContent {
            computed_space: total_space,
            uncomputed_directories: vec![],
        },
    );

    total_space
}

pub(crate) fn part1(input: &Input) -> Output {
    let mut filesystem = build_filesystem(input);

    let directory_names: Vec<_> = filesystem.keys().cloned().collect();

    let mut small_directories_total_space = Space::bytes(0);

    for directory_name in directory_names {
        let space = space_for_directory(&directory_name, &mut filesystem);

        if space <= Space::bytes(100_000) {
            small_directories_total_space += space;
        }
    }

    small_directories_total_space.0
}

fn best_fit_directory(
    directory_name: &DirectoryName,
    space_to_free: Space,
    filesystem: &mut HashMap<DirectoryName, DirectoryContent>,
) -> Option<Space> {
    let content = filesystem
        .get(directory_name)
        .expect("All directories can be referenced from the filesystem.")
        .clone();
    let content_space = content
        .uncomputed_directories
        .iter()
        .map(|sub_directory_name| space_for_directory(sub_directory_name, filesystem))
        .sum();
    let total = content.computed_space + content_space;

    filesystem.insert(
        directory_name.clone(),
        DirectoryContent {
            computed_space: total,
            uncomputed_directories: vec![],
        },
    );

    if total >= space_to_free {
        Some(total)
    } else {
        None
    }
}

pub(crate) fn part2(input: &Input) -> Output {
    let mut filesystem = build_filesystem(input);

    let total_space = Space::bytes(70_000_000);
    let space_needed = Space::bytes(30_000_000);
    let min_space_to_leave = total_space - space_needed;
    let space_used = space_for_directory(&DirectoryName("/".to_owned()), &mut filesystem);
    let space_to_free = space_used - min_space_to_leave;

    let directory_names: Vec<_> = filesystem.keys().cloned().collect();

    let best_fit = directory_names
        .iter()
        .filter_map(|dn| best_fit_directory(dn, space_to_free, &mut filesystem))
        .min();

    best_fit
        .expect("We have to have found something small enough")
        .0
}

pub(crate) const INPUT_STR: &str = include_str!("_input");

#[cfg(test)]
const EXAMPLE_STR: &str = include_str!("_example");

#[cfg(test)]
const SMALL_STR: &str = include_str!("_small");

#[cfg(test)]
const SMALL2_STR: &str = include_str!("_small2");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test_small() {
        assert_eq!(part1(&parse(SMALL_STR).unwrap().1), 0)
    }

    #[test]
    fn part1_test_small2() {
        assert_eq!(
            DirectoryName("direct".to_owned()),
            parsing::parse_directory_name("direct")
                .expect("to be able to parse directory name")
                .1
        );
        assert_eq!(part1(&parse(SMALL2_STR).unwrap().1), 600)
    }

    #[test]
    fn part1_test_example() {
        assert_eq!(part1(&parse(EXAMPLE_STR).unwrap().1), 95437)
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1(&parse(INPUT_STR).unwrap().1), 1141028)
    }

    #[test]
    fn part2_test_example() {
        assert_eq!(part2(&parse(EXAMPLE_STR).unwrap().1), 24933642)
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(part2(&parse(INPUT_STR).unwrap().1), 8278005)
    }
}
