use std::collections::HashMap;
use crate::Answer;

pub struct Day02;

impl Answer for Day02 {
    fn name(&self) -> &'static str {
        "Rock Paper Scissors"
    }

    fn part_a(&self, input: &str) -> String {
        let mut shape_score = HashMap::new();
        shape_score.insert('X', 1); // ROCK
        shape_score.insert('Y', 2); // PAPER
        shape_score.insert('Z', 3); // SCISSORS

        let mut outcomes = HashMap::new();
        outcomes.insert("win", 6);
        outcomes.insert("loss", 0);
        outcomes.insert("draw", 3);

        let mut scores = HashMap::new();
        scores.insert("A X", outcomes["draw"] + shape_score[&'X']);
        scores.insert("A Y", outcomes["win"] + shape_score[&'Y']);
        scores.insert("A Z", outcomes["loss"] + shape_score[&'Z']);
        scores.insert("B X", outcomes["loss"] + shape_score[&'X']);
        scores.insert("B Y", outcomes["draw"] + shape_score[&'Y']);
        scores.insert("B Z", outcomes["win"] + shape_score[&'Z']);
        scores.insert("C X", outcomes["win"] + shape_score[&'X']);
        scores.insert("C Y", outcomes["loss"] + shape_score[&'Y']);
        scores.insert("C Z", outcomes["draw"] + shape_score[&'Z']);

        let mut game_scores: Vec<i32> = vec![];
        for game in input.split("\n") {
            game_scores.push(scores[game])
        }

        let sum: i32 = game_scores.iter().sum();
        sum.to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let mut shape_score = HashMap::new();
        shape_score.insert('X', 1); // ROCK
        shape_score.insert('Y', 2); // PAPER
        shape_score.insert('Z', 3); // SCISSORS

        let mut outcomes = HashMap::new();
        outcomes.insert("win", 6);
        outcomes.insert("loss", 0);
        outcomes.insert("draw", 3);

        let mut scores = HashMap::new();
        scores.insert("A X", outcomes["loss"] + shape_score[&'Z']);
        scores.insert("A Y", outcomes["draw"] + shape_score[&'X']);
        scores.insert("A Z", outcomes["win"] + shape_score[&'Y']);
        scores.insert("B X", outcomes["loss"] + shape_score[&'X']);
        scores.insert("B Y", outcomes["draw"] + shape_score[&'Y']);
        scores.insert("B Z", outcomes["win"] + shape_score[&'Z']);
        scores.insert("C X", outcomes["loss"] + shape_score[&'Y']);
        scores.insert("C Y", outcomes["draw"] + shape_score[&'Z']);
        scores.insert("C Z", outcomes["win"] + shape_score[&'X']);

        let mut game_scores: Vec<i32> = vec![];
        for game in input.split("\n") {
            game_scores.push(scores[game])
        }

        let sum: i32 = game_scores.iter().sum();
        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::load;

    #[test]
    fn test_part_a() {
        let input = load(2022, 2, "test.txt");
        assert_eq!("15", Day02.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = load(2022, 2, "test.txt");
        assert_eq!("12", Day02.part_b(&input))
    }
}
