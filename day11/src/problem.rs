mod domain;
mod parsing;

use itertools::Itertools;

pub(crate) use self::domain::{Input, Output};
pub(crate) use self::parsing::parse;

pub(crate) fn part1(_input: &Input) -> Output {
    let mut inspections = vec![0; _input.len()];
    let mut monkeys = _input.clone();
    for _round in 0..20 {
        //println!("<<ROUND({})>>", round + 1);

        for i in 0..monkeys.len() {
            //println!("Monkey {}\n", i);

            let monkey = monkeys[i].clone();
            for item in monkey.items.iter() {
                //println!("  Monkey inspects an item with a worry level of {}.", item);
                let new_worry_level = monkey.operation.worry_level(*item);
                let lowered_worry_level = new_worry_level / 3;
                //println!("    Monkey gets bored with item. Worry level is divided by {} to {}.", 3, lowered_worry_level);

                let divisible = lowered_worry_level % monkey.denominator == 0;
                //println!("    Current worry level is{} divisible by {}.", if divisible { "" } else { " not "}, monkey.denominator);
                let target = if divisible {
                    monkey.target_if_true
                } else {
                    monkey.target_if_false
                };

                monkeys[target].items.push_back(lowered_worry_level);
                monkeys[i].items.pop_front();
                inspections[i] += 1;
                //println!("    Item with worry level {} is thrown to monkey {}.", lowered_worry_level, target);
            }
        }

        //println!("After round {}, the monkeys are holding items with these worry levels:", round + 1);
        for (_i, monkey) in monkeys.iter().enumerate() {
            let _items = monkey.items.iter().join(", ");
            //println!("Monkey {}: {}", i, items);
        }
        //println!("");
    }

    for (_i, _count) in inspections.iter().enumerate() {
        //println!("Monkey {} inspected items {} times.", i, count);
    }

    inspections.sort_by(|a, b| b.cmp(a));
    inspections.into_iter().take(2).product()
}

pub(crate) fn part2(_input: &Input) -> Output {
    42
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
        assert_eq!(part2(&parse(EXAMPLE_STR)), 42)
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(part2(&parse(INPUT_STR)), 42)
    }
}
