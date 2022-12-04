use std::{ops::RangeInclusive, str::FromStr};

use color_eyre::eyre::Context;
use thiserror::Error;

#[derive(Error, Debug)]
enum ParsingError {
    #[error("The section assignment was not a range separated by a dash ({0})")]
    NotARange(String),
}

#[derive(Clone)]
pub struct SectionAssignment(RangeInclusive<i32>);

impl SectionAssignment {
    pub fn fully_contains(&self, other: &SectionAssignment) -> bool {
        self.0.contains(other.0.start()) && self.0.contains(other.0.end())
    }
}

impl FromStr for SectionAssignment {
    type Err = color_eyre::eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        if let (Some(lower), Some(upper), None) = (split.next(), split.next(), split.next()) {
            let assignment = SectionAssignment(
                lower.parse().wrap_err("parsing the lower bound")?
                    ..=upper.parse().wrap_err("parsing the upper bound")?,
            );
            Ok(assignment)
        } else {
            Err(ParsingError::NotARange(s.to_owned()).into())
        }
    }
}
