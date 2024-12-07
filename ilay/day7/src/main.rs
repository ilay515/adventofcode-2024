use std::{fs, iter::Sum};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

struct Equation {
    parameters: Vec<u64>,
    result: u64,
}

impl Equation {
    fn new(line: &str) -> Equation {
        let (_, (result, parameters)) = separated_pair(
            Equation::parse_number,
            tag(": "),
            Equation::parse_parameters,
        )(line)
        .unwrap();
        Equation { parameters, result }
    }

    fn parse_number(input: &str) -> IResult<&str, u64> {
        map_res(digit1, |s: &str| s.parse::<u64>())(input)
    }

    fn parse_parameters(input: &str) -> IResult<&str, Vec<u64>> {
        separated_list0(space1, Equation::parse_number)(input)
    }

    fn is_valid(&self) -> bool {
        let parameters_left = self.parameters[1..].to_vec();
        let current_result = self.parameters[0];
        self.run_calculation(parameters_left, current_result)
    }

    fn run_calculation(&self, mut parameters_left: Vec<u64>, current_result: u64) -> bool {
        if current_result > self.result {
            return false;
        }
        if parameters_left.is_empty() {
            return current_result == self.result;
        }

        let next_parameter = parameters_left.remove(0);

        self.run_calculation(parameters_left.clone(), current_result * next_parameter)
            || self.run_calculation(parameters_left.clone(), current_result + next_parameter)
            || self.run_calculation(parameters_left, Equation::concat(current_result, next_parameter))
    }

    fn concat(a: u64, b: u64) -> u64 {
        a as u64 * 10u64.pow(b.ilog10() + 1) + b as u64
    }
}

fn solve(input: &str) {
    let result = input
        .lines()
        .map(|line| Equation::new(line))
        .filter(|equation| {
            equation.is_valid()
        })
        .map(|eq| eq.result)
        .sum::<u64>();
    println!("{:?}", result);
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("failed to read input");
    solve(&input);
}
