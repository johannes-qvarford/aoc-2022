use super::vector::Vector;

#[derive(Clone, Copy)]
pub(crate) struct Movement {
    pub(crate) direction: Vector,
    pub(crate) length: i32,
}

pub(crate) type Input = Vec<Movement>;
pub(crate) type Output = usize;
