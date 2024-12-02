use std::{
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(reports: &Vec<Vec<i32>>) -> i32 {
    let mut safe: i32 = 0;

    for report in reports {
        if is_safe(report) {
            safe += 1;
        }
    }

    return safe;
}

fn part_2(reports: &Vec<Vec<i32>>) -> i32 {
    let mut safe: i32 = 0;

    for report in reports {
        if is_safe(report) {
            safe += 1;
        } else {
            for i in 0..=report.len() - 1  {
                let mut without_one = report.clone();
                without_one.remove(i);

                if is_safe(&without_one) {
                    safe += 1;
                    break;
                }
            }
        }
    }

    return safe;
}

fn is_safe(report: &Vec<i32>) -> bool {
    let increase = report[0] < report[1];

    for i in 0..=report.len() - 2 {
        let current = report[i];
        let next = report[i + 1];
        let diff = (current - next).abs();

        if (current < next) != increase
            || diff < 1 
            || diff > 3 {
            return false;
        }
    }

    return true;
}

fn get_input() -> Vec<Vec<i32>> {
    let args: Vec<String> = env::args().collect();
    let input_path = format!("src/{}", &args[1]);
    let file = File::open(input_path).expect("Failed to open file");
    let lines = BufReader::new(file).lines();

    let mut output = Vec::new();

    for line_res in lines {
        let line = line_res.expect("");
        let numbers = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("parse error"))
            .collect::<Vec<i32>>();

        output.push(numbers);
    }

    return output;
}
