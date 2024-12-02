use core::num;
use std::{
    env::{self},
    fs::File,
    io::{BufRead, BufReader}, process::Output,
};
fn main() {
    let mut input = get_input();
    println!("Part 1: {}", part_1(&mut input));
    // println!("Part 2: {}", part_2(&input));
}

fn part_1(locations: &mut Vec<Vec<i32>>) -> i32 {
    let mut output = 0;

    for report in locations {
        let increase = report[0] < report[1];
        let mut is_safe = true;

        for i in 0..=report.len() - 2   {
            let current = report[i];
            let next = report[i + 1];
            let diff = (current - next).abs();

            println!("{:?}, curr: {}, next: {}", report, current, next);

            if ((current < next) != increase) 
                || diff < 1
                || diff > 3  {
                is_safe = false;
                println!("breaking");
                break;
            }
        }

        if is_safe {
            output += 1;
        }
    }

    return output;
}

// fn part_2(locations: &mut Vec<Vec<i32>>) -> i32 {
//     let mut output = 0;

//     return output;
// }

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
