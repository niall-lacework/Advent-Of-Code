use std::{cmp::Ordering, str::FromStr, collections::HashMap};
use crate::Answer;

pub struct Day02AlternativeAnswer;

// Create an enum to represent the shapes with values that match
// the rules of the game.
#[derive(PartialEq)] // Needed for the `PartialOrd` implementation
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err("Not a known move".to_string()),
        }
    }
}

// Implement `PartialOrd` because the comparison for Rock, Paper, Scissors is circular.
// There is no maximum or minimum value, so we need to implement the edge cases.
impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &Shape::Scissors && other == &Shape::Rock { // we lost edge case
            Some(Ordering::Less)
        } else if self == &Shape::Rock && other == &Shape::Scissors { // we won edge case
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8))) // compare the numeric values of the Shape enum for all other cases.
        }
    }
}

impl Answer for Day02AlternativeAnswer {
    fn name(&self) -> &'static str {
        "Rock Paper Scissors"
    }

    fn part_a(&self, input: &str) -> String {
        let result: u32 = input
            .lines()
            .map(|line| {
                let shapes: Vec<Shape> = line
                    .split(" ") // split the line into two each player shape
                    .map(|s| s.parse::<Shape>().unwrap()) // parse the string into a Shape enum
                    .collect();
                // because we implemented `PartialOrd` we can use our `partial_cmp` method to compare the shapes.
                match shapes[0].partial_cmp(&shapes[1]) {
                    Some(Ordering::Equal) => 3 + shapes[0] as u32,
                    Some(Ordering::Greater) => 6 + shapes[0] as u32,
                    Some(Ordering::Less) => 0 + shapes[0] as u32,
                    None => panic!("moves should be comparable")
                }
        })
        .sum();
        result.to_string()
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
        assert_eq!("15", Day02AlternativeAnswer.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = load(2022, 2, "test.txt");
        assert_eq!("12", Day02AlternativeAnswer.part_b(&input))
    }
}
