use std::fmt::Display;

use super::vector::{Vector, ORIGIN};

#[derive(Clone)]
pub(crate) struct Snake {
    pub(crate) segments: Box<[Vector]>,
}

impl Snake {
    pub(crate) fn of_length(len: i32) -> Snake {
        Snake {
            segments: vec![ORIGIN; len as usize].into_boxed_slice(),
        }
    }

    pub(crate) fn mov(&self, direction: Vector) -> Snake {
        let mut copy = self.clone();

        copy.segments[0] = copy.segments[0] + direction;
        for i in 1..copy.segments.len() {
            let before = copy.segments[i - 1];
            let after = copy.segments[i];
            let new_after = Snake::constrain_after_to_before(before, after);
            copy.segments[i] = new_after;
        }
        copy
    }

    pub(crate) fn constrain_after_to_before(before: Vector, after: Vector) -> Vector {
        let difference = before - after;
        if difference.x.abs() > 1 || difference.y.abs() > 1 {
            after + (difference.x.signum(), difference.y.signum()).into()
        } else {
            after
        }
    }

    pub(crate) fn tail(&self) -> Vector {
        *self.segments.last().expect("Snake has at least 1 segment")
    }

    fn head(&self) -> Vector {
        *self.segments.first().expect("Snake has at least 1 segment")
    }
}

impl Display for Snake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..6).rev() {
            for x in 0..6 {
                let p: Vector = (x, y).into();
                let c = if p == self.head() {
                    'H'
                } else if p == self.tail() {
                    'T'
                } else {
                    '.'
                };
                c.fmt(f)?;
            }
            '\n'.fmt(f)?;
        }
        Ok(())
    }
}
