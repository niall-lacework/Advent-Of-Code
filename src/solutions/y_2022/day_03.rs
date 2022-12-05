use crate::Answer;

pub struct Day03;

impl Answer for Day03 {
    fn name(&self) -> &'static str {
        "Rucksack Reorganization"
    }

    fn part_a(&self, input: &str) -> String {
        let groups = input.split("\n").collect::<Vec<&str>>();
        groups
            .iter()
            .map(|s| s.split_at(s.len() / 2))
            .map(|(a, b)| get_sum(a, &[b]))
            .sum::<u32>()
            .to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let groups = input.split("\n").collect::<Vec<&str>>();
        groups
            .chunks(3)
            .map(|lines| get_sum(lines[0], &(lines[1..=2])))
            .sum::<u32>()
            .to_string()
    }
}

fn get_sum(a: &str, rest: &[&str]) -> u32 {
    let mut a_chars = a.chars().collect::<Vec<char>>();
    a_chars.sort();
    a_chars.dedup();
    a_chars
        .iter()
        .filter(|a_char| rest.iter().all(|r| r.contains(**a_char)))
        .map(|c| get_value(*c))
        .sum::<u32>()
}

fn get_value(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => panic!("Invalid character"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::load;

    #[test]
    fn test_part_a() {
        let input = load(2022, 03, "test.txt");
        assert_eq!("157", Day03.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = load(2022, 03, "test_b.txt");
        assert_eq!("70", Day03.part_b(&input))
    }
}
