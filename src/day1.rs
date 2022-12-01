use anyhow::{Context, Result};

fn backpack_string_to_calorie_list(s: &str) -> Result<Vec<i32>> {
    let items = s
        .split('\n')
        .map(|row| row.parse::<i32>().context("Failed parsing calorie row"));

    items.collect()
}

fn calorie_lists() -> Result<impl Iterator<Item = i32>> {
    let s = std::fs::read_to_string("src/res/1.input")?;
    let backpack_strings = s.split("\n\n");
    let backpacks_maybe: Result<Vec<Vec<i32>>> = backpack_strings
        .map(backpack_string_to_calorie_list)
        .collect();
    let backpacks = backpacks_maybe?;

    let calories = backpacks
        .into_iter()
        .map(|backpack| backpack.into_iter().sum());
    Ok(calories)
}

fn day1_1() -> Result<i32> {
    let lists = calorie_lists()?;
    Ok(lists.max().unwrap_or_default())
}

fn day1_2() -> Result<i32> {
    let lists = calorie_lists()?;
    let mut sorted = lists.collect::<Vec<_>>();
    sorted.sort_by(|a, b| b.cmp(a));
    let top_calories_sum = sorted.into_iter().take(3).sum();
    Ok(top_calories_sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day1_1_test() {
        assert_eq!(day1_1().unwrap(), 73211);
    }

    #[test]
    fn day1_2_test() {
        assert_eq!(day1_2().unwrap(), 213958);
    }
}
