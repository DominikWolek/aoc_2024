use std::{
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

type Input = Vec<(i64, Vec<i64>)>;

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> i64 {
    return input
        .iter()
        .filter(|x| can_be_valid(x.0, &x.1))
        .map(|x| x.0)
        .sum();
}

fn can_be_valid(result: i64, args: &Vec<i64>) -> bool {
    let mask_len = (args.len() - 1) as u32;
    for i in 0..(2_i32.pow(mask_len)) {
        let mask = get_mask(i, mask_len);

        if apply_operators(args, mask.clone()) == result {
            println!("{}: {:?} for {:?}", result, args, mask);
            return true;
        }
    }

    return false;
}

fn apply_operators(args: &[i64], mask: Vec<bool>) -> i64 {
    let mut cur = args[0];
    for i in 1..args.len() {
        if mask[i - 1] {
            cur = cur + args[i];
        } else {
            cur = cur * args[i];
        }
    }
    return cur;
}

fn get_mask(num: i32, mask_len: u32) -> Vec<bool> {
    let mut out = Vec::new();
    for i in 0..mask_len {
        out.push(((num >> i) & 1) == 0);
    }
    return out;
}

fn part_2(input: &Input) -> i64 {
    let mut output: i64 = 0;

    // for i in input {}

    return output;
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
