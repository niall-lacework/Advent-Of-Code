use std::fs;

pub trait Answer {
    fn name(&self) -> &'static str;
    fn part_a(&self, input: &str) -> String;
    fn part_b(&self, input: &str) -> String;
}

pub fn load(year: u32, day: u32, filename:&str ) -> String {
    let file = format!("inputs/{year}/day_{:02}/{filename}", day);
    fs::read_to_string(&file).unwrap_or_else(|_| panic!("Error reading file {}", file))
}