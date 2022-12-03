use crate::choice::Choice;
use crate::rps::Rps;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Match {
    pub player: Choice,
    pub opponent: Rps,
}

impl Match {
    pub fn part1_total_score(&self) -> i32 {
        let player_rps: Rps = self.player.into();
        let round_score = if player_rps.beats() == self.opponent {
            6
        } else if self.opponent.beats() == player_rps {
            0
        } else {
            3
        };
        player_rps.shape_score() + round_score
    }

    pub fn part2_total_score(&self) -> i32 {
        match self.player {
            Choice::X => self.opponent.beats().shape_score(),
            Choice::Y => 3 + self.opponent.shape_score(),
            Choice::Z => 6 + self.opponent.beaten_by().shape_score(),
        }
    }
}
