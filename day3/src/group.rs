use color_eyre::eyre::Result;
use std::collections::BTreeSet;
use thiserror::Error;

use crate::{item_type::ItemType, rucksack::Rucksack};

#[derive(Error, Debug)]
enum BadgeError {
    #[error("Badge wasn't found.")]
    NotFound,
    #[error("Multiple badge candidates were found ({0})")]
    MultipleCandidates(usize),
}

pub struct Group {
    pub rucksacks: [Rucksack; 3],
}

impl Group {
    pub fn badge(&self) -> Result<ItemType> {
        let first: BTreeSet<ItemType> = self.rucksacks[0]
            .items()
            .intersection(&self.rucksacks[1].items())
            .copied()
            .collect();
        let second: Vec<ItemType> = first
            .intersection(&self.rucksacks[2].items())
            .copied()
            .collect();
        if second.is_empty() {
            Err(BadgeError::NotFound.into())
        } else if second.len() > 1 {
            Err(BadgeError::MultipleCandidates(second.len()).into())
        } else {
            Ok(second[0])
        }
    }
}
