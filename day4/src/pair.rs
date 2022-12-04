use std::str::FromStr;

use color_eyre::eyre::{Context, Report, Result};
use thiserror::Error;

use crate::section_assignment::SectionAssignment;

#[derive(Error, Debug)]
enum ParsingError {
    #[error("The section assignments were not separated by a comma ({0})")]
    NotAPair(String),
}

#[derive(Clone)]
pub struct Pair(pub [SectionAssignment; 2]);

impl Pair {
    pub fn one_range_fully_contains_the_other(&self) -> bool {
        self.0[0].fully_contains(&self.0[1]) || self.0[1].fully_contains(&self.0[0])
    }
}

impl FromStr for Pair {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');

        if let (Some(left), Some(right), None) = (split.next(), split.next(), split.next()) {
            fn parse(ss: &str) -> Result<SectionAssignment> {
                ss.parse()
                    .wrap_err_with(|| format!("parsing section assignment ({0})", ss.to_owned()))
            }
            let pair = Pair([
                parse(left).wrap_err("parsing left assignment")?,
                parse(right).wrap_err("parsing right assignment")?,
            ]);
            Ok(pair)
        } else {
            Err(ParsingError::NotAPair(s.to_owned()).into())
        }
    }
}
