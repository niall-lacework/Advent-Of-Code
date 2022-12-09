use std::iter::Enumerate;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{self, alpha1, digit1, newline},
    combinator::verify,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    *,
};

use crate::Answer;

fn a_num(input: &str) -> IResult<&str, u32> {
    let (input, c) = verify(complete::anychar, |c| c.to_digit(10).is_some())(input)?;
    let number = c.to_digit(10).unwrap();
    Ok((input, number))
}
fn parse_trees(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, vecs) = separated_list1(newline, many1(a_num))(input)?;

    Ok((input, vecs))
}

pub struct Day08;

impl Answer for Day08 {
    fn name(&self) -> &'static str {
        "Treetop Tree House"
    }

    fn part_a(&self, input: &str) -> String {
        let (_, trees) = parse_trees(input).unwrap();
        let mut visible_trees: Vec<Vec<bool>> = trees
            .iter()
            .enumerate() // enumerate to get the row number
            .map(|(i, tree_line)| {
                let line_max_length = tree_line.len() - 1; // we might not have a square grid so calculate the max length
                tree_line.iter()
                    .enumerate() // enumerate to get the column number
                    .map(|(line_i, _)| {
                        // these are all the edges
                        if i == 0 || i == line_max_length || line_i == 0 || line_i == line_max_length {
                            true
                        } else {
                            false
                        }
                    })
                    .collect()
            })
            .collect();
        
        for row in 0..trees.len() {
            let mut current_tree_size = 0;
            for col in 0..trees[row].len() {
                if col == 0 {
                    current_tree_size = trees[row][col];
                } else if trees[row][col] > current_tree_size {
                    current_tree_size = trees[row][col];
                    visible_trees[row][col] = true;
                }
            }
        }
        for row in (0..trees.len()).rev() {
            let mut current_tree_size = 0;
            for col in (0..trees[row].len()).rev() {
                if col == trees.len() - 1 {
                    current_tree_size = trees[row][col];
                } else if trees[row][col] > current_tree_size {
                    current_tree_size = trees[row][col];
                    visible_trees[row][col] = true;
                }
            }
        }

        for col in 0..trees.len() {
            let mut current_tree_size = 0;
            for row in 0..trees[0].len() {
                if row == 0 {
                    current_tree_size = trees[row][col] as usize;
                } else if trees[row][col] > current_tree_size as u32
                {
                    current_tree_size = trees[row][col] as usize;
                    visible_trees[row][col] = true;
                }
            }
        }
        for col in (0..trees.len()).rev() {
            let mut current_tree_size = 0;
            for row in (0..trees[0].len()).rev() {
                if row == trees.len() - 1 {
                    current_tree_size = trees[row][col] as usize;
                } else if trees[row][col] > current_tree_size as u32
                {
                    current_tree_size = trees[row][col] as usize;
                    visible_trees[row][col] = true;
                }
            }
        }
        visible_trees
            .iter()
            .flatten()
            .filter(|&&x|x)
            .count()
            .to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let (_, trees) = parse_trees(input).unwrap();
        let mut max_scenic_score = 0;

        let y_max = trees.len();
        let x_max = trees[0].len();

        for (row, tree_line) in trees.iter().enumerate() {
            for (col, current_tree) in tree_line.iter().enumerate() {
                let mut scores = [0, 0, 0, 0];
                // left
                for x_pos in (0..col).rev() {
                    // println!("checking left {}", x_pos);
                    if trees[row][x_pos] < *current_tree {
                        scores[0] += 1;
                    } else {
                        scores[0] += 1;
                        break;
                    }
                }
                // right
                for x_pos in (col + 1)..x_max {
                    // println!("checking right {}", x_pos);
                    if trees[row][x_pos] < *current_tree {
                        scores[1] += 1;
                    } else {
                        scores[1] += 1;
                        break;
                    }
                }
                // up
                for y_pos in (0..row).rev() {
                    // println!("checking up {}", y_pos);
                    if trees[y_pos][col] < *current_tree {
                        scores[2] += 1;
                    } else {
                        scores[2] += 1;
                        break;
                    }
                }
                // down
                for y_pos in (row + 1)..y_max {
                    // println!("checking down {}", y_pos);
                    if trees[y_pos][col] < *current_tree {
                        scores[3] += 1;
                    } else {
                        scores[3] += 1;
                        break;
                    }
                }
                let current_score = scores.iter().product::<usize>();
                if current_score > max_scenic_score {
                    max_scenic_score = current_score;
                }
            }
        }

        max_scenic_score.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::load;

    #[test]
    fn test_part_a() {
        let input = load(2022, 08, "test.txt");
        assert_eq!("21", Day08.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = load(2022, 08, "test.txt");
        assert_eq!("8", Day08.part_b(&input))
    }
}
