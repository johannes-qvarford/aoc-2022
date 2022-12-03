use std::collections::BTreeSet;

use color_eyre::eyre::{self, Context};

use crate::{compartment::Compartment, item_type::ItemType};

#[derive(Clone)]
pub struct Rucksack {
    compartments: [Compartment; 2],
}

impl TryFrom<&str> for Rucksack {
    type Error = eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (a, b) = value.split_at(value.len() / 2);
        let rucksack = Rucksack {
            compartments: [
                a.try_into().wrap_err("parsing left compartment")?,
                b.try_into().wrap_err("parsing right compartment")?,
            ],
        };
        Ok(rucksack)
    }
}

impl Rucksack {
    pub fn items(&self) -> BTreeSet<ItemType> {
        self.compartments[0]
            .iter()
            .chain(self.compartments[1].iter())
            .collect()
    }

    pub fn misplaced_item(&self) -> ItemType {
        self.compartments[0].shared_item_type(&self.compartments[1])
    }
}
