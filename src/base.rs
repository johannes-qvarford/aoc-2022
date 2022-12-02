#[cfg(test)]
pub fn read_input(day: i32) -> String {
    std::fs::read_to_string(format!("src/res/{day}.input")).unwrap()
}
