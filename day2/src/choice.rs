use crate::rps::Rps;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Choice {
    X,
    Y,
    Z,
}

impl Choice {
    pub fn parse(c: char) -> Choice {
        match c {
            'X' => Choice::X,
            'Y' => Choice::Y,
            'Z' => Choice::Z,
            _ => {
                println!("Character is {c}");
                unimplemented!()
            }
        }
    }
}

impl From<Choice> for Rps {
    fn from(c: Choice) -> Self {
        match c {
            Choice::X => Rps::Rock,
            Choice::Y => Rps::Paper,
            Choice::Z => Rps::Scissors,
        }
    }
}
