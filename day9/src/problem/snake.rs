use std::fmt::Display;

use super::vector::{Vector, ORIGIN};

pub(crate) struct Snake {
    pub(crate) head: Vector,
    pub(crate) tail: Vector,
}

impl Snake {
    pub(crate) fn new() -> Snake {
        Snake {
            head: ORIGIN,
            tail: ORIGIN,
        }
    }

    pub(crate) fn mov(&self, direction: Vector) -> Snake {
        let new_head = self.head + direction;
        let new_tail = Snake::constrain_tail_to_head(self.tail, new_head, self.head);
        Snake {
            head: new_head,
            tail: new_tail,
        }
    }

    pub(crate) fn constrain_tail_to_head(
        tail: Vector,
        new_head: Vector,
        old_head: Vector,
    ) -> Vector {
        let difference = new_head - tail;
        if difference.x.abs() > 1 || difference.y.abs() > 1 {
            old_head
        } else {
            tail
        }
    }
}

impl Display for Snake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..6).rev() {
            for x in 0..6 {
                let p: Vector = (x, y).into();
                let c = if p == self.head {
                    'H'
                } else if p == self.tail {
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
