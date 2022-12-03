use color_eyre::{eyre, Result};

use thiserror::Error;

#[derive(Error, Debug)]
enum ParsingError {
    #[error("Invalid character: {0}. Expected one in the range [a-z] or [A-Z])")]
    InvalidCharacter(char),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ItemType(i32);

impl TryFrom<char> for ItemType {
    type Error = eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let mut ascii_bytes = [0; 4];
        value.encode_utf8(&mut ascii_bytes);

        if ascii_bytes > [b'z', 0, 0, 0] {
            return Err(ParsingError::InvalidCharacter(value).into());
        }

        let b = ascii_bytes[0];

        if (b'A'..=b'Z').contains(&b) {
            Ok(Self(((b - b'A') + 27).into()))
        } else if (b'a'..=b'z').contains(&b) {
            Ok(Self(((b - b'a') + 1).into()))
        } else {
            Err(ParsingError::InvalidCharacter(value).into())
        }
    }
}

impl ItemType {
    pub fn priority(&self) -> i32 {
        self.0
    }
}
