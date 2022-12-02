use crate::Answer;

pub struct Day24;

impl Answer for Day24 {
    fn name(&self) -> &'static str {
        "?"
    }

    fn part_a(&self, input: &str) -> String {
        "A".to_string()
    }

    fn part_b(&self, input: &str) -> String {
        "B".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::load;

    #[test]
    fn test_part_a() {
        let input = load(2022, 24, "test.txt");
        assert_eq!("Not A", Day24.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = load(2022, 24, "test.txt");
        assert_eq!("Not B", Day24.part_b(&input))
    }
}