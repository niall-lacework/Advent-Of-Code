use crate::Answer;

pub struct Day01;

impl Answer for Day01 {
    fn name(&self) -> &'static str {
        "Calorie Counting"
    }


    fn part_a(&self, input: &str) -> String {
        let result = input
            .split("\n\n") // each inventry can be identified by a double newline
            .map(|elf_inventory| { 
                elf_inventory
                    .lines()
                    // turbofish (::<TYPE>) to specify the type. Because parse returns a Result, we need to unwrap it.
                    // because we know the data is well formed we can unwrap it.
                    .map(|calories| calories.parse::<u32>().unwrap()) 
                    .sum::<u32>()   
            })
            .max()
            .unwrap();
        result.to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let mut result = input
            .split("\n\n") // each inventry can be identified by a double newline
            .map(|elf_inventory| { 
                elf_inventory
                    .lines()
                    // turbofish (::<TYPE>) to specify the type. Because parse returns a Result, we need to unwrap it.
                    // because we know the data is well formed we can unwrap it.
                    .map(|calories| calories.parse::<u32>().unwrap()) 
                    .sum::<u32>()   
            })
            // the Iterator Trait has no sort method, so we need to collect it into a Vec first.
            // checkout the itertools crate for more iterator methods.
            .collect::<Vec<_>>();
        // implement a reverse sort so that the largest number is first.
        // comparison is built into Rust, there are a numer of Traits that define how to order
        result.sort_by(|a,b| b.cmp(a));
        let sum = result.iter().take(3).sum::<u32>();
        sum.to_string()
    }
}

#[cfg(test)] // only compile when running tests
mod tests {
    use super::*;
    use crate::solution::load;

    #[test] // test macro
    fn test_part_a(){
        let input = load(2022, 1, "test.txt");
        assert_eq!("24000", Day01.part_a(&input))
    }

    #[test]
    fn test_part_b(){
        let input = load(2022, 1, "test.txt");
        assert_eq!("45000", Day01.part_b(&input))
    }
}