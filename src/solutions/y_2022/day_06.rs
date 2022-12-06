
use std::collections::HashSet;

use crate::Answer;

pub struct Day06;

impl Answer for Day06 {
    fn name(&self) -> &'static str {
        "Tuning Trouble"
    }

    fn part_a(&self, input: &str) -> String {
        let mut marker: Vec<char> = Vec::new();
        for (position, character) in input.chars().enumerate() {
            // println!("Marker: {:?}", marker);
            marker.push(character); // Add the character to the marker
            if marker.len() > 4 {
                marker.remove(0);
            }
            let mut setv = HashSet::new(); // Create a new set
            marker.iter().for_each(|x| {
                setv.insert(x);
            }); // Add all the characters to the set
            let mut temp_vec = Vec::new(); // Create a new temp vector because I couldn't get the length of the set?
            for i in setv {
                temp_vec.push(i);
            }
            // println!("Set: {:?}", temp_vec);
            if temp_vec.len() == 4 {
                return (position + 1).to_string(); // If the set is 4, return the position
            }
        }
        "No solution found".to_string()

    }

    fn part_b(&self, input: &str) -> String {
        let mut marker: Vec<char> = Vec::new();
        for (position, character) in input.chars().enumerate() {
            // println!("Marker: {:?}", marker);
            marker.push(character); // Add the character to the marker
            if marker.len() > 14 {
                marker.remove(0);
            }
            let mut setv = HashSet::new(); // Create a new set
            marker.iter().for_each(|x| {
                setv.insert(x);
            }); // Add all the characters to the set
            let mut temp_vec = Vec::new(); // Create a new temp vector because I couldn't get the length of the set?
            for i in setv {
                temp_vec.push(i);
            }
            // println!("Set: {:?}", temp_vec);
            if temp_vec.len() == 14 {
                return (position + 1).to_string(); // If the set is 4, return the position
            }
        }
        "No solution found".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::load;

    #[test]
    fn test_part_a() {
        let input = load(2022, 06, "test.txt");
        assert_eq!("6", Day06.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = load(2022, 06, "test.txt");
        assert_eq!("19", Day06.part_b(&input))
    }
}