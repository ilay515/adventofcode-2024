use std::fs;

use nom::{
    character::complete::{digit1, space1}, combinator::map_res, multi::separated_list1, sequence::separated_pair, IResult
};


fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_line(line: &str) -> Vec<u32>{
    let (_, result) = separated_list1(
        space1, parse_number)(line)
        .unwrap();
    result
}

fn are_pair_unsafe(a: i32, b: i32) -> bool {
    b - a < 1 || b - a > 3
}

fn is_report_safe(report: &Vec<u32>) -> bool {
    if report.len() < 2 {
        return false;
    }

    let mut report_copy = report.to_vec();
    if report[0] > report[1] {
        report_copy.reverse();
    }


    let mut is_safe = true;

    report_copy.windows(2).for_each(|window| {
        if let [a, b] = window {
            if are_pair_unsafe(*a as i32, *b as i32) {
                is_safe = false;
            }
        }
    });

    is_safe
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("failed to read input");

    let result = input
        .lines()
        .map(parse_line)
        .filter(is_report_safe)
        .count();

    println!("{:?}", result);
}