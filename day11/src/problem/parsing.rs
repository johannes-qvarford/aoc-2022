use std::collections::VecDeque;
use yaml_rust::YamlLoader;

use crate::problem::domain::{BinaryExpression, Monkey};

use super::Input;

fn parse_monkey(monkey_yaml: &yaml_rust::Yaml) -> Monkey {
    let starting_items_yaml = &monkey_yaml["Starting items"];
    //println!("Starting items is:\n{:?}", starting_items_yaml);

    let items: VecDeque<usize> = starting_items_yaml
        .as_str()
        .map(|s| {
            s.split(", ")
                //.inspect(|s| println!("Gonna parse an item: {}", s))
                .map(|s| s.parse().expect("Item to be an integer"))
                .collect()
        })
        .unwrap_or_else(|| {
            vec![starting_items_yaml
                .as_i64()
                .expect("Starting items to be a single integer if it's not a string")
                as usize]
            .into()
        });
    //println!("Items are:\n{:?}", items);

    let operation_yaml = &monkey_yaml["Operation"];
    let expression: BinaryExpression = operation_yaml
        .as_str()
        .expect("Opeation to be a string")
        .replace("new = ", "")
        .parse()
        .expect("Operation is a valid BinaryExpression");
    //println!("Operation is:\n{:?}", expression);

    let test_yaml = &monkey_yaml["Test"];
    let denominator: usize = test_yaml
        .as_str()
        .expect("Test to be a string")
        .replace("divisible by ", "")
        .parse()
        .expect("Denominator to be an integer");
    //println!("Denominator is:\n{}", denominator);

    // If true: throw to monkey 1
    let parse_if = |label: &str| {
        let if_yaml = &monkey_yaml[&format!("If {}", label)[..]];
        let target: usize = if_yaml
            .as_str()
            .expect("If expression to be a string")
            .replace("throw to monkey ", "")
            .parse()
            .expect("target monkey to be an integer");
        target
    };
    let if_false = parse_if("false");
    let if_true = parse_if("true");
    //println!("If true: {}\nIf false: {}", if_true, if_false);

    Monkey {
        items,
        operation: expression,
        denominator,
        target_if_false: if_false,
        target_if_true: if_true,
    }
}

pub(crate) fn parse(_s: &str) -> Input {
    let yaml_string = _s.replace("  If", "If");
    //println!("YAML STRING:\n{}", yaml_string);
    let docs = YamlLoader::load_from_str(&yaml_string)
        .expect("Input to be valid YAML, once the 'If' lines are deindented.");
    let doc = docs[0].clone();
    //println!("\nParsed YAML:\n{:?}", doc);

    let mut monkeys: Vec<Monkey> = vec![];
    for (key, value) in doc.into_hash().expect("YAML document to be a Hash") {
        let _id: usize = key
            .as_str()
            .expect("YAML key to be 'Monkey X'")
            .replace("Monkey ", "")
            .parse()
            .expect("All monkey labels to end with an integer");
        //println!("Monkey {} YAML is:\n{:?}", id, value);
        let monkey = parse_monkey(&value);
        //println!("Monkey {} is:\n{:?}", id, monkey);
        monkeys.push(monkey);
    }

    monkeys
}
