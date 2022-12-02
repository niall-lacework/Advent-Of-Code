use crate::Answer;

pub struct Day12;

impl Answer for Day12 {
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
        let input = load(2022, 12, "test.txt");
        assert_eq!("Not A", Day12.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = load(2022, 12, "test.txt");
        assert_eq!("Not B", Day12.part_b(&input))
    }
}