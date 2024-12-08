use std::fs::read_to_string;
use std::i32::{MIN, MAX};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

pub struct Day2Solutions {
    pub part_1: usize,
    pub part_2: usize,
}

#[derive(Default)]
enum IncreasingCheck {
    #[default]
    NotChecked,
    Increasing,
    Decreasing,
}

pub(crate) fn day_2(file_path: &str) -> Day2Solutions {
    let input = collect_input(file_path);

    // Part 1
    let safe_reports_part_1 = input.iter().filter(|report| is_report_safe(report)).count();

    // Part 2:
    let mut safe_reports_part_2 = 0;
    for report in input {
        // Iterate over every combination of the vec itself, and the vec with 1 element removed.
        let mut report_combinations = vec![report.clone()];
        for i in 0..report.len() {
            let mut new_combination = report.clone();
            new_combination.remove(i);
            report_combinations.push(new_combination);
        }

        if report_combinations
            .iter()
            .filter(|report| is_report_safe(report))
            .count()
            > 0
        {
            safe_reports_part_2 += 1;
        }
    }

    Day2Solutions {
        part_1: safe_reports_part_1,
        part_2: safe_reports_part_2,
    }
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut is_increasing = IncreasingCheck::default();
    // Iterate over 2 elements at a time to compare them
    for report_entries in report.windows(2) {
        let difference = report_entries[0] - report_entries[1];

        // First check the increasing check
        match (&is_increasing, difference) {
            (IncreasingCheck::NotChecked, MIN..0) => {
                is_increasing = IncreasingCheck::Increasing;
            } 
            (IncreasingCheck::NotChecked, 1..=MAX) => {
                is_increasing = IncreasingCheck::Decreasing;
            }
            (IncreasingCheck::Decreasing, MIN..0)
            | (IncreasingCheck::Increasing, 1..=MAX)
            | (_, 0) => {
                return false;
            }
            (IncreasingCheck::Decreasing, 1..=MAX)
            | (IncreasingCheck::Increasing, MIN..0) => {}
        }

        // Now check the maximum allowed difference.
        if difference.abs() > 3 {
            return false;
        }
    }

    true
}

fn collect_input(file_path: &str) -> Vec<Vec<i32>> {
    let input = read_to_string(file_path).expect("Issue with reading the file");
    let (_, lines) = get_lines(&input).expect("Issue with parsing the file");
    lines
}

fn get_numbers(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, nums) = separated_list1(tag(" "), complete::i32)(input)?;
    Ok((input, nums))
}

fn get_lines(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, numbers) = separated_list1(newline, get_numbers)(input)?;
    Ok((input, numbers))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example_input_path = "src/inputs/day_2_part_1_example.txt";
        let example_solution = day_2(&example_input_path);
        assert_eq!(example_solution.part_1, 2);
    }
}
