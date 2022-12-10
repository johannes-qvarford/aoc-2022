use std::fmt::Display;

pub(crate) type Input = Box<[Instruction]>;
pub(crate) type Output = i32;

#[derive(Clone, Copy)]
pub(crate) struct Addx(pub(crate) i32);

#[derive(Clone, Copy)]
pub(crate) enum Instruction {
    Noop,
    Addx(Addx),
}

impl Instruction {
    fn state_increment(&self) -> Option<i32> {
        match self {
            Instruction::Noop => None,
            Instruction::Addx(Addx(amount)) => Some(*amount),
        }
    }
}

pub(crate) trait InstructionIteratorExt: Iterator<Item = Instruction> {
    fn execute_instructions(&mut self) -> std::vec::IntoIter<State> {
        let mut current = State::beginning();
        let mut history = vec![];
        for instruction in self {
            for state in current.perform_instruction(&instruction) {
                history.push(state)
            }
            current = *history.last().expect("History has at least one item");
        }
        history.into_iter()
    }
}

// alternatively use a blanket implementation to implement `InstructionIteratorExt` for all types that also implement `Iterator<Item=Instruction>`: `T`, `<T: Iterator<Item=Instruction>>`
impl<T> InstructionIteratorExt for T where T: Iterator<Item = Instruction> {}

#[derive(Clone, Copy)]
pub(crate) struct Register {
    during_cycle: i32,
    after_cycle: i32,
}

impl Register {
    fn new(amount: i32) -> Register {
        Register {
            during_cycle: amount,
            after_cycle: amount,
        }
    }

    fn increment_at_end_of_cycle(&self, amount: i32) -> Register {
        Register {
            during_cycle: self.after_cycle,
            after_cycle: self.after_cycle + amount,
        }
    }

    fn perform_noop(&self) -> Register {
        Register {
            during_cycle: self.after_cycle,
            after_cycle: self.after_cycle,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct State {
    register: Register,
}

impl State {
    pub(crate) fn beginning() -> State {
        State {
            register: Register::new(1),
        }
    }

    pub(crate) fn perform_instruction(&self, instruction: &Instruction) -> Vec<State> {
        if let Some(delayed_addition) = instruction.state_increment() {
            let register1 = self.register.perform_noop();
            let register2 = register1.increment_at_end_of_cycle(delayed_addition);
            vec![
                State {
                    register: register1,
                },
                State {
                    register: register2,
                },
            ]
        } else {
            vec![State {
                register: self.register.perform_noop(),
            }]
        }
    }

    pub(crate) fn value_during_cycle(&self) -> i32 {
        self.register.during_cycle
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "|{}=>{}|",
            self.register.during_cycle, self.register.after_cycle
        )
    }
}

pub struct Crt(Vec<State>);

impl Crt {
    pub(crate) fn draw_screen(states: Vec<State>) -> Crt {
        Crt(states)
    }
}

const SCREEN_WIDTH: i32 = 40;

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (cycle, state) in self.0.iter().enumerate() {
            let column = cycle as i32 % SCREEN_WIDTH;
            let val = state.value_during_cycle();
            let draw_sprite = (val - 1..=val + 1).contains(&(column as i32));
            if draw_sprite {
                '#'.fmt(f)?
            } else {
                '.'.fmt(f)?
            }

            if column == SCREEN_WIDTH - 1 {
                '\n'.fmt(f)?
            }
        }

        Ok(())
    }
}
