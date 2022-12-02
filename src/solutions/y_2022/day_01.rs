use crate::Answer;

pub struct Day01;

impl Answer for Day01 {
    fn name(&self) -> &'static str {
        "Calorie Counting"
    }

    fn part_a(&self, input: &str) -> String {
        let elfs = get_elfs(&input);

        elfs.last().unwrap().to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let elfs = get_elfs(&input);

        elfs.iter().rev().take(3).sum::<u32>().to_string()
    }
}

fn get_elfs(data: &str) -> Vec<u32> {
    let mut out = data
        .replace('\r', "")
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .collect::<Vec<_>>();
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::load;

    #[test]
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