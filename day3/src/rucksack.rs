use color_eyre::eyre::{self, Context};

use crate::{compartment::Compartment, item_type::ItemType};

#[derive(Clone)]
pub struct Rucksack {
    compartment: [Compartment; 2],
}

impl TryFrom<&str> for Rucksack {
    type Error = eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (a, b) = value.split_at(value.len() / 2);
        let rucksack = Rucksack {
            compartment: [
                a.try_into().wrap_err("parsing left compartment")?,
                b.try_into().wrap_err("parsing right compartment")?,
            ],
        };
        Ok(rucksack)
    }
}

impl Rucksack {
    pub fn misplaced_item(&self) -> ItemType {
        self.compartment[0].shared_item_type(&self.compartment[1])
    }
}
