use std::{
    borrow::Cow,
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

type Input = Vec<String>;
type Mults = Vec<(i64, i64)>;

const MULT: &str = r"mul\(\b[0-9]+\b\,\b[0-9]+\b\)";
const DO: &str = r".*?do\(\)";

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> i64 {
    let mults = parse_mults(input);
    return calculate(mults);
}

fn part_2(input: &Input) -> i64 {
    let mults: Vec<(i64, i64)> = parse_part_2(input);
    return calculate(mults);
}

fn calculate(mults: Vec<(i64, i64)>) -> i64 {
    let mut output: i64 = 0;
    for mult in mults {
        output += mult.0 * mult.1;
    }
    return output;
}

fn get_input() -> Input {
    let args: Input = env::args().collect();
    let input_path = format!("src/{}", &args[1]);
    let file = File::open(input_path).expect("Failed to open file");
    let lines = BufReader::new(file).lines();

    let mut output = Vec::new();
    for line_res in lines {
        let line = line_res.expect("");
        output.push(line);
    }

    return output;
}

fn parse_mults(input: &Input) -> Mults {
    let re_mult = Regex::new(MULT).unwrap();
    let mut output = Vec::new();

    for line in input {
        let mut mults = re_mult
            .find_iter(&line)
            .filter_map(|mult| Some(cut_and_parse(mult.as_str())))
            .collect::<Mults>();
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

fn parse_part_2(input: &Input) -> Mults {
    let joined: String = input.join("");
    let mut splitted = joined.split("don't()").collect::<Vec<&str>>();
    let first = "do()".to_owned() + splitted[0];
    splitted[0] = &first;

    return parse_mults(&delete_do_prefixes(splitted));
}

fn delete_do_prefixes(splitted: Vec<&str>) -> Input {
    let re_do: Regex = Regex::new(DO).unwrap();
    let mut output = Vec::new();

    for line in splitted {
        let mut new_line = Cow::from("");
        if line.find("do()") != None {
            new_line = re_do.replace(line, "");
        }

        output.push(new_line.into_owned());
    }

    return output;
}
