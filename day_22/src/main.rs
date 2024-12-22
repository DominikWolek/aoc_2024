use std::{
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

type Input = Vec<usize>;

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

const GENERATIONS_CNT: usize = 2000;
const PRUNE_BASE: usize = 16_777_216;

fn part_1(input: &Input) -> usize {
    input
        .iter()
        .map(|initial| calc_secret(*initial, GENERATIONS_CNT))
        .sum()
}

fn part_2(input: &Input) -> i64 {
    let mut output: i64 = 0;

    for i in input {}

    return output;
}

fn calc_secret(secret: usize, generations_cnt: usize) -> usize {
    if generations_cnt == 0 {
        return secret;
    } else {
        let mut current = mix(secret << 6, secret);
        current = prune(current);

        current = mix(current >> 5, current);
        current = prune(current);

        current = mix((current * 2048), current);
        current = prune(current);

        return calc_secret(current, generations_cnt - 1);
    }
}

fn mix(left: usize, right: usize) -> usize {
    left ^ right
}

fn prune(current: usize) -> usize {
    current % PRUNE_BASE
}

fn get_input() -> Input {
    let args: Vec<String> = env::args().collect();
    let input_path = format!("src/{}", &args[1]);
    let file = File::open(input_path).expect("Failed to open file");

    BufReader::new(file)
        .lines()
        .map(|line| line.expect("").parse::<usize>().expect("parse error"))
        .collect()
}
