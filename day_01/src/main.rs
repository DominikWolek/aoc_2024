use std::{
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};
fn main() {
    let mut input = get_input();
    println!("Part 1: {}", part_1(&mut input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(locations: &mut (Vec<i32>, Vec<i32>)) -> i32 {
    let mut output = 0;

    locations.0.sort_unstable();
    locations.1.sort_unstable();

    for i in 0..=locations.0.len() - 1 {
        output += (locations.0[i] - locations.1[i]).abs();
    }

    return output;
}

fn part_2(locations: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut output = 0;

    for i in &locations.0 {
        let mut count = 0;
        for j in &locations.1 {
            if i == j {
                count += 1;
            }
        }
        output += i * count;
    }

    return output;
}

fn get_input() -> (Vec<i32>, Vec<i32>) {
    let args: Vec<String> = env::args().collect();
    let input_path = format!("src/{}", &args[1]);
    let file = File::open(input_path).expect("Failed to open file");
    let lines = BufReader::new(file).lines();

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line_res in lines {
        let line = line_res.expect("");
        let numbers = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("parse error"))
            .collect::<Vec<i32>>();
        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    return (left, right);
}
