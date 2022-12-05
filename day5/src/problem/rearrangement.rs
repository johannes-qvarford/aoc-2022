use std::str::FromStr;

use color_eyre::{eyre::Context, Report, Result};
use thiserror::Error;

use super::{iterator_ext::IteratorNextExt, simple::StackId};

#[derive(Error, Debug)]
enum InputParsingError {
    #[error("Could not parse rearangment 'move <amount> from <from> to <to>' from ({0}) ")]
    NotADrawingAndInstructions(String),
}

#[derive(Clone)]
pub(crate) struct Rearrangement {
    pub(crate) amount: i32,
    pub(crate) from: StackId,
    pub(crate) to: StackId,
}

impl FromStr for Rearrangement {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        if let (Some(_), Some(amount), Some(_), Some(from), Some(_), Some(to), None) = split.next7()
        {
            let r = Rearrangement {
                amount: amount
                    .parse()
                    .wrap_err("parsing 'amount' for rearrangement")?,
                from: from.parse().wrap_err("parsing 'from' for rearrangement")?,
                to: to.parse().wrap_err("parsing 'to' for rearrangement")?,
            };
            Ok(r)
        } else {
            Err(InputParsingError::NotADrawingAndInstructions(s.to_owned()).into())
        }
    }
}
