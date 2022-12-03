#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    pub fn shape_score(&self) -> i32 {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }

    pub fn beats(&self) -> Rps {
        match self {
            Rps::Rock => Rps::Scissors,
            Rps::Paper => Rps::Rock,
            Rps::Scissors => Rps::Paper,
        }
    }

    pub fn beaten_by(&self) -> Rps {
        match self {
            Rps::Rock => Rps::Paper,
            Rps::Paper => Rps::Scissors,
            Rps::Scissors => Rps::Rock,
        }
    }

    pub fn parse(c: char) -> Rps {
        match c {
            'A' => Rps::Rock,
            'B' => Rps::Paper,
            'C' => Rps::Scissors,
            _ => {
                println!("Character is {c}");
                unimplemented!()
            }
        }
    }
}
