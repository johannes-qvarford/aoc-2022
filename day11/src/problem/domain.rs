use std::{collections::VecDeque, str::FromStr};

use color_eyre::Report;

pub(crate) type Input = Vec<Monkey>;
pub(crate) type Output = usize;

#[derive(Debug, Clone)]
enum LeafExpression {
    Old,
    Constant(usize),
}
impl LeafExpression {
    fn worry_level(&self, item: usize) -> usize {
        match self {
            LeafExpression::Old => item,
            LeafExpression::Constant(constant) => *constant,
        }
    }
}

impl FromStr for LeafExpression {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            Ok(LeafExpression::Old)
        } else {
            Ok(LeafExpression::Constant(
                s.parse().expect("Non-old leaf expression to be an integer"),
            ))
        }
    }
}

#[derive(Debug, Clone)]
enum Operator {
    Plus,
    Times,
}

impl FromStr for Operator {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some(op) => Ok(if op == '+' {
                Operator::Plus
            } else {
                Operator::Times
            }),
            None => panic!("Operator needs to consist of a single char."),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BinaryExpression {
    left: LeafExpression,
    right: LeafExpression,
    operator: Operator,
}
impl BinaryExpression {
    pub(crate) fn worry_level(&self, item: usize) -> usize {
        let left = self.left.worry_level(item);
        let right = self.right.worry_level(item);
        match self.operator {
            Operator::Plus => left + right,
            Operator::Times => left * right,
        }
    }
}

impl FromStr for BinaryExpression {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        match (parts.next(), parts.next(), parts.next()) {
            (Some(left), Some(operator), Some(right)) => Ok(BinaryExpression {
                left: left.parse().expect("Left to be a leaf expression"),
                right: right.parse().expect("Right to be a leaf expression"),
                operator: operator.parse().expect("Operator to be a valid operator"),
            }),
            _ => panic!("Binary expression is not made up from 3 parts separated by spaces"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Monkey {
    pub(crate) items: VecDeque<usize>,
    pub(crate) operation: BinaryExpression,
    pub(crate) denominator: usize,
    pub(crate) target_if_true: usize,
    pub(crate) target_if_false: usize,
}
