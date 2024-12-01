use std::fs;

use nom::{
    character::complete::{digit1, space1},
    sequence::separated_pair,
    combinator::map_res,
    IResult,
};


fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_line(line: &str, left: &mut Vec<u32>, right: &mut Vec<u32>) {
    let (__, (first, second)) = separated_pair(
        parse_number, 
        space1, 
        parse_number)(line)
        .unwrap();

    left.push(first);
    right.push(second);
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("failed to read input");

    let (mut left, mut right) = (Vec::new(), Vec::new());

    input.lines().for_each(|line| parse_line(line, &mut left, &mut right));

    left.sort();
    right.sort();

    let result: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();


    println!("{:?}", result);
}
