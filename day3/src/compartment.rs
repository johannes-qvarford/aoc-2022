use color_eyre::{
    eyre::{self, Context},
    Result,
};
use std::collections::BTreeSet;

use crate::item_type::ItemType;

#[derive(Clone)]
pub struct Compartment {
    items: BTreeSet<ItemType>,
}

impl TryFrom<&str> for Compartment {
    type Error = eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let vec: Result<Vec<ItemType>> = value
            .chars()
            .map(|c| c.try_into().wrap_err("parsing an item type"))
            .collect();
        let items: BTreeSet<_> = vec.wrap_err("parsing item types")?.into_iter().collect();
        Ok(Compartment { items })
    }
}

impl Compartment {
    pub fn iter(&self) -> impl Iterator<Item = ItemType> + '_ {
        return self.items.iter().copied();
    }

    pub fn shared_item_type(&self, other: &Compartment) -> ItemType {
        let mut combination = self.items.intersection(&other.items);
        let n = combination.next().unwrap();
        *n
    }
}
