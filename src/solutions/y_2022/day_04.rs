use crate::Answer;

pub struct Day04;

impl Answer for Day04 {
    fn name(&self) -> &'static str {
        "Camp Cleanup"
    }

    fn part_a(&self, input: &str) -> String {
        let assignments = input.split("\n").collect::<Vec<&str>>();
        let mut overlaps = 0;
        for assignment in assignments {
            let pairs = assignment.split(",").collect::<Vec<&str>>();
            let (lower_bound, upper_bound) = get_bounds(pairs[0]);
            let (lower_bound_2, upper_bound_2) = get_bounds(pairs[1]);
            if is_fully_contained(lower_bound, upper_bound, lower_bound_2, upper_bound_2) {
                // println!(
                //     "Contained: {}-{}, {}-{}",
                //     lower_bound, upper_bound, lower_bound_2, upper_bound_2
                // );
                overlaps += 1;
            }
        }
        overlaps.to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let assignments = input.split("\n").collect::<Vec<&str>>();
        let mut overlaps = 0;
        for assignment in assignments {
            let pairs = assignment.split(",").collect::<Vec<&str>>();
            let (lower_bound, upper_bound) = get_bounds(pairs[0]);
            let (lower_bound_2, upper_bound_2) = get_bounds(pairs[1]);
            if is_overlapping(lower_bound, upper_bound, lower_bound_2, upper_bound_2) {
                // println!(
                //     "Overlap: {}-{}, {}-{}",
                //     lower_bound, upper_bound, lower_bound_2, upper_bound_2
                // );
                overlaps += 1;
            }
        }
        overlaps.to_string()
    }
}

fn get_bounds(input: &str) -> (i32, i32) {
    let bounds = input.split("-").collect::<Vec<&str>>();
    let lower_bound = bounds[0].parse::<i32>().unwrap();
    let upper_bound = bounds[1].parse::<i32>().unwrap();
    (lower_bound, upper_bound)
}

fn is_fully_contained(
    lower_bound: i32,
    upper_bound: i32,
    lower_bound_2: i32,
    upper_bound_2: i32,
) -> bool {
    lower_bound <= lower_bound_2 && upper_bound >= upper_bound_2
        || lower_bound_2 <= lower_bound && upper_bound_2 >= upper_bound
}

fn is_overlapping(
    lower_bound: i32,
    upper_bound: i32,
    lower_bound_2: i32,
    upper_bound_2: i32,
) -> bool {
    lower_bound <= lower_bound_2 && upper_bound >= lower_bound_2
        || lower_bound_2 <= lower_bound && upper_bound_2 >= lower_bound
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::load;

    #[test]
    fn test_part_a() {
        let input = load(2022, 04, "test.txt");
        assert_eq!("2", Day04.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = load(2022, 04, "test.txt");
        assert_eq!("4", Day04.part_b(&input))
    }
}
