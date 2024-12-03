use std::{
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
    process::Output,
    str::Utf8Chunk,
};

use regex::Regex;

type Input = Vec<(i64, i64)>;

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    // println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> i64 {
    let mut output: i64 = 0;

    for mult in input {
        println!("({}, {})", mult.0, mult.1);
        output += mult.0 * mult.1;
    }
    // for i in input {}

    return output;
}

// fn part_2(input: &Input) -> i64 {
//     let mut output: i32 = 0;

//     for i in input {}

//     return output;
// }

fn get_input() -> Input {
    let args: Vec<String> = env::args().collect();
    let input_path = format!("src/{}", &args[1]);
    let file = File::open(input_path).expect("Failed to open file");
    let lines = BufReader::new(file).lines();

    let re: Regex = Regex::new(r"mul\(\b[0-9]+\b\,\b[0-9]+\b\)").unwrap();

    let mut output = Vec::new();
    for line_res in lines {
        let line = line_res.expect("");
        let mut mults = re
            .find_iter(line.as_str())
            .filter_map(|mult| Some(cut_and_parse(mult.as_str())))
            .collect::<Vec<(i64, i64)>>();
        output.append(&mut mults);
    }

    return output;
}

fn cut_and_parse(multiplication: &str) -> (i64, i64) {
    let length = multiplication.len();
    let vec = &multiplication[4..length - 1]
        .split(",")
        .map(|x| x.parse::<i64>().expect("failed to parse"))
        .collect::<Vec<i64>>();

    return (vec[0], vec[1]);
}
