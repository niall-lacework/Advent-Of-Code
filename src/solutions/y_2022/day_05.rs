use std::borrow::BorrowMut;

use crate::Answer;

pub struct Day05;

impl Answer for Day05 {
    fn name(&self) -> &'static str {
        "Supply Stacks"
    }

    fn part_a(&self, input: &str) -> String {
        let mut stacks = Vec::from([
            Vec::from(['Q', 'H', 'C', 'T', 'N', 'S', 'V', 'B']),
            Vec::from(['G', 'B', 'D', 'W']),
            Vec::from(['B', 'Q', 'S', 'T', 'R', 'W', 'F']),
            Vec::from(['N', 'D', 'J', 'Z', 'S', 'W', 'G', 'L']),
            Vec::from(['F', 'V', 'D', 'P', 'M']),
            Vec::from(['J', 'W', 'F']),
            Vec::from(['V', 'J', 'B', 'Q', 'N', 'L']),
            Vec::from(['N', 'S', 'Q', 'J', 'C', 'R', 'T', 'G']),
            Vec::from(['M', 'D', 'W', 'C', 'Q', 'S', 'J']),
        ]);
        let instructions = input.split("\n").collect::<Vec<&str>>();
        for instruction in instructions {
            if instruction.starts_with("move") {
                // println!("Instruction: {}", instruction);
                let parts = instruction.split(" ").collect::<Vec<&str>>();
                let amount = parts[1].parse::<usize>().unwrap();
                let from = parts[3].parse::<usize>().unwrap() - 1;
                let to = parts[5].parse::<usize>().unwrap() - 1;
                for _ in 0..amount {
                    // println!("Moving {} from {:?} to {:?}", stacks[from][0], stacks[from], stacks[to]);
                    let val = stacks[from][0];
                    stacks[from].remove(0);
                    stacks[to].insert(0, val);
                }
                // println!("Stacks: {:?}", stacks);
            }
        }

        let code = stacks.iter().map(|s| s[0]).collect::<String>();
        code
    }

    fn part_b(&self, input: &str) -> String {
        // let mut stacks = Vec::from([
        //     Vec::from(['N', 'Z']),
        //     Vec::from(['D', 'C', 'M']),
        //     Vec::from(['P']),
        // ]);
        let mut stacks = Vec::from([
            Vec::from(['Q', 'H', 'C', 'T', 'N', 'S', 'V', 'B']),
            Vec::from(['G', 'B', 'D', 'W']),
            Vec::from(['B', 'Q', 'S', 'T', 'R', 'W', 'F']),
            Vec::from(['N', 'D', 'J', 'Z', 'S', 'W', 'G', 'L']),
            Vec::from(['F', 'V', 'D', 'P', 'M']),
            Vec::from(['J', 'W', 'F']),
            Vec::from(['V', 'J', 'B', 'Q', 'N', 'L']),
            Vec::from(['N', 'S', 'Q', 'J', 'C', 'R', 'T', 'G']),
            Vec::from(['M', 'D', 'W', 'C', 'Q', 'S', 'J']),
        ]);
        let instructions = input.split("\n").collect::<Vec<&str>>();
        for instruction in instructions {
            if instruction.starts_with("move") {
                // println!("Instruction: {}", instruction);
                let parts = instruction.split(" ").collect::<Vec<&str>>();
                let amount = parts[1].parse::<usize>().unwrap();
                let from = parts[3].parse::<usize>().unwrap() - 1;
                let to = parts[5].parse::<usize>().unwrap() - 1;

                let vals = stacks[from][0..amount].to_vec();
                for val in vals.iter().rev() {
                    stacks[to].insert(0, *val);
                    stacks[from].remove(0);
                }
                // println!("Stacks: {:?}", stacks);
            }
        }
        let code = stacks.iter().map(|s| s[0]).collect::<String>();
        code
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::load;

    #[test]
    fn test_part_a() {
        let input = load(2022, 05, "test.txt");
        assert_eq!("CMZ", Day05.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = load(2022, 05, "test.txt");
        assert_eq!("MCD", Day05.part_b(&input))
    }
}
