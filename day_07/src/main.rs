use std::{
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

// use cached::proc_macro::cached;
// use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    return input
        .iter()
        .filter(|x| {
            let mut reversed = x.1.clone();
            reversed.reverse();
            can_be_valid(x.0, &reversed, 0, ops)
        })
        .map(|x| x.0)
        .sum();
}

fn can_be_valid(result: i64, args: &Vec<i64>, i: usize, allowed_ops: &Vec<Op>) -> bool {
    if i == args.len() - 1 {
        return args[i] == result;
    } else {
        return allowed_ops
            .iter()
            .any(|op| match reverse_op(result, args[i], op) {
                Some(result) => can_be_valid(result, args, i + 1, allowed_ops),
                None => false,
            });
    }
}

fn reverse_op(x: i64, y: i64, op: &Op) -> Option<i64> {
    match op {
        Op::Add => Some(x - y),
        Op::Mult => (x % y == 0).then(|| x / y),
        Op::Conc => reverse_concat(y, x),
    }
}

fn reverse_concat(y: i64, x: i64) -> Option<i64> {
    let y_len = f64::log(y as f64 + 0.1, 10.0).ceil() as u32;
    let pow_10 = 10_i64.pow(y_len);
    let x_pref = x / 10_i64.pow(y_len);
    let x_suff = x - x_pref * pow_10;

    if x_suff == y {
        return Some(x_pref);
    } else {
        return None;
    };
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
