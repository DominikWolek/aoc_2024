use std::{
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

#[derive(Clone, Debug)]
enum Op {
    Add,
    Mult,
    Conc,
}

type Input = Vec<(i64, Vec<i64>)>;

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> i64 {
    return sum_correct(input, &Vec::from([Op::Add, Op::Mult]));
}

fn part_2(input: &Input) -> i64 {
    return sum_correct(input, &Vec::from([Op::Add, Op::Mult, Op::Conc]));
}

fn sum_correct(input: &Input, ops: &Vec<Op>) -> i64 {
    input
        .iter()
        .filter(|x| can_be_valid(x.0, &x.1, ops))
        .map(|x| x.0)
        .sum()
}

fn can_be_valid(result: i64, args: &Vec<i64>, allowed_ops: &Vec<Op>) -> bool {
    let mask_len = (args.len() - 1) as u32;

    (0..mask_len)
        .map(|_| allowed_ops.clone())
        .multi_cartesian_product()
        .filter(|mask| apply_operators(args, mask) == result)
        .count()
        > 0
}

fn apply_operators(args: &[i64], mask: &Vec<Op>) -> i64 {
    let mut cur = args[0];
    for i in 1..args.len() {
        match mask[i - 1] {
            Op::Add => {
                cur = cur + args[i];
            }
            Op::Mult => {
                cur = cur * args[i];
            }
            Op::Conc => {
                cur = concatenate(cur, args[i]);
            }
        }
    }
    return cur;
}

fn concatenate(cur: i64, i: i64) -> i64 {
    return format!("{}{}", cur.to_string(), i.to_string())
        .parse::<i64>()
        .expect("");
}

fn get_input() -> Input {
    let args: Vec<String> = env::args().collect();
    let input_path = format!("src/{}", &args[1]);
    let file = File::open(input_path).expect("Failed to open file");
    let lines = BufReader::new(file).lines();

    let mut output = Vec::new();

    for line_res in lines {
        let line = line_res.expect("");

        let splitted = line.split(": ").collect::<Vec<&str>>();
        let test_val = splitted[0].parse::<i64>().expect("");

        let numbers = splitted[1]
            .split_whitespace()
            .map(|x| x.parse::<i64>().expect("parse error"))
            .collect::<Vec<i64>>();

        output.push((test_val, numbers));
    }

    return output;
}
