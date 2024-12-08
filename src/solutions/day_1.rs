use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{collections::HashMap, fs::read_to_string};

pub struct Day1Solutions {
    pub part_1: u32,
    pub part_2: u32,
}

pub(crate) fn day_1(file_path: &str) -> Day1Solutions {
    // Collect and parse input
    let (mut left, mut right) = collect_input(file_path);

    // Sort vectors
    left.sort();
    right.sort();

    // Part 1:
    // Calculate the differences
    let diff_sum: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(left_num, right_num)| left_num.abs_diff(*right_num))
        .sum();

    // Part 2: Calculate the similarity scores
    // Get the frequencies of the numbers in the right list
    let mut frequency_right: HashMap<u32, u32> = HashMap::new();
    for right_num in right {
        *frequency_right.entry(right_num).or_insert(0) += 1;
    }
    // Multiply each number in the left list by its frequency in the right list and
    // add that to the total sim_score
    let sim_score: u32 = left
        .iter()
        .map(|left_num| frequency_right.get(&left_num).unwrap_or(&0) * left_num)
        .sum();

    Day1Solutions {
        part_1: diff_sum,
        part_2: sim_score,
    }
}

fn collect_input(file_path: &str) -> (Vec<u32>, Vec<u32>) {
    let input = read_to_string(file_path).expect("Issue with reading the file");
    let (_, lines) = get_lines(&input).expect("Issue with parsing the file");

    // Split up Vec<(u32, u32)> into two separate Vec's
    let mut left_numbers = Vec::with_capacity(lines.len());
    let mut right_numbers = Vec::with_capacity(lines.len());
    lines.iter().for_each(|(left, right)| {
        left_numbers.push(*left);
        right_numbers.push(*right);
    });
    (left_numbers, right_numbers)
}

fn get_numbers(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, (num_1, num_2)) = separated_pair(complete::u32, tag("   "), complete::u32)(input)?;
    Ok((input, (num_1, num_2)))
}

fn get_lines(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (input, numbers) = separated_list1(newline, get_numbers)(input)?;
    Ok((input, numbers))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example_input_path = "src/inputs/day_1_part_1_example.txt";
        let example_solution = day_1(&example_input_path);
        assert_eq!(example_solution.part_1, 11);
        assert_eq!(example_solution.part_2, 31);
    }
}
