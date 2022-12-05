use std::collections::VecDeque;

use std::collections::BTreeMap;
use std::str::from_utf8;
use std::str::FromStr;

use color_eyre::{Report, Result};
use itertools::Itertools;

use super::simple::Crate;
use super::simple::StackId;

#[derive(Clone)]
pub(crate) struct Stacks(BTreeMap<StackId, VecDeque<Crate>>);

impl Stacks {
    pub(crate) fn by_id_mut(&mut self, id: StackId) -> Option<&mut VecDeque<Crate>> {
        self.0.get_mut(&id)
    }

    pub(crate) fn top_crates(&mut self) -> String {
        let vec = self
            .0
            .iter_mut()
            .map(|(_, vd)| {
                vd.pop_front()
                    .expect("There to be at least one crate left.")
            })
            .collect_vec();
        from_utf8(&vec[..])
            .expect("Bytes are in ascii range.")
            .to_owned()
    }
}

impl FromStr for Stacks {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows_and_index_row = s.lines().collect_vec();
        let rows_len = rows_and_index_row.len() - 1;
        let rows = rows_and_index_row.into_iter().take(rows_len);

        let insertions = rows.flat_map(|row| Stacks::maybe_crates(row.as_bytes().iter().copied()));

        let mut map = BTreeMap::<StackId, VecDeque<Crate>>::new();

        for (id, crat) in insertions {
            let vd = map.entry(id).or_insert_with(VecDeque::new);
            vd.push_back(crat);
        }

        Ok(Stacks(map))
    }
}

impl Stacks {
    fn maybe_crates<I>(it: I) -> Vec<(StackId, Crate)>
    where
        I: Iterator<Item = u8>,
    {
        it.chunks(4)
            .into_iter()
            .map(|chunk| chunk.collect_vec())
            .enumerate()
            .map(|(i, v)| (StackId(i + 1), Stacks::maybe_crate(&v)))
            .flat_map(|(id, option)| option.map(|stack| (id, stack)))
            .collect()
    }

    fn maybe_crate(it: &[u8]) -> Option<Crate> {
        match it[..3] {
            [_, crat, _] if crat != b' ' => Some(crat),
            _ => None,
        }
    }
}
