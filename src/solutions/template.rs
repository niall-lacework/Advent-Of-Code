use crate::Answer;

pub struct DayXX;

impl Answer for DayXX {
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
        let input = load(2022, XX, "test.txt");
        assert_eq!("Not A", DayXX.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = load(2022, XX, "test.txt");
        assert_eq!("Not B", DayXX.part_b(&input))
    }
}