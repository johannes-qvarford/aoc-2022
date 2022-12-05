use std::str::FromStr;

use color_eyre::{eyre::Context, Report};

pub(crate) type Crate = u8;

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub(crate) struct StackId(pub(crate) usize);

impl FromStr for StackId {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let us = usize::from_str(s).wrap_err("parsing stack id")?;
        Ok(StackId(us))
    }
}
