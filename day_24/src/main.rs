use std::{
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

type Input = Vec<Vec<i64>>;

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> i64 {
    let mut output: i64 = 0;

    for i in input {}

    return output;
}

fn part_2(input: &Input) -> i64 {
    let mut output: i64 = 0;

    for i in input {}

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
        let numbers = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().expect("parse error"))
            .collect::<Vec<i64>>();

        output.push(numbers);
    }

    return output;
}
