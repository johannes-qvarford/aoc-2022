mod domain;
mod parsing;

pub(crate) use self::domain::{Input, Output};
pub(crate) use self::parsing::parse;

fn monkey_business(
    _input: &Vec<domain::Monkey>,
    worry_level_denominator: usize,
    rounds: usize,
) -> usize {
    let mut inspections = vec![0; _input.len()];
    let mut monkeys = _input.clone();

    let modolu: usize = monkeys.iter().map(|monkey| monkey.denominator).product();

    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            for item in monkey.items.iter() {
                let new_worry_level = monkey.operation.worry_level(*item);
                let lowered_worry_level = new_worry_level / worry_level_denominator;
                let lowered_worry_level = lowered_worry_level % modolu;

                let divisible = lowered_worry_level % monkey.denominator == 0;
                let target = if divisible {
                    monkey.target_if_true
                } else {
                    monkey.target_if_false
                };

                monkeys[target].items.push_back(lowered_worry_level);
                monkeys[i].items.pop_front();
                inspections[i] += 1;
            }
        }
    }
    inspections.sort_by(|a, b| b.cmp(a));
    inspections.into_iter().take(2).product()
}

pub(crate) fn part1(_input: &Input) -> Output {
    monkey_business(_input, 3, 20)
}

pub(crate) fn part2(_input: &Input) -> Output {
    monkey_business(_input, 1, 10_000)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input::{EXAMPLE_STR, INPUT_STR};

    #[test]
    fn part1_test_example() {
        assert_eq!(part1(&parse(EXAMPLE_STR)), 10605)
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1(&parse(INPUT_STR)), 95472)
    }

    #[test]
    fn part2_test_example() {
        assert_eq!(part2(&parse(EXAMPLE_STR)), 2713310158)
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(part2(&parse(INPUT_STR)), 17926061332)
    }
}
