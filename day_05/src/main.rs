use std::{
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

type Rules = Vec<(i64, i64)>;
type Pages = Vec<Vec<i64>>;
type Input = (Rules, Pages);

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input.0, &input.1));
    println!("Part 2: {}", part_2(&input.0, &input.1));
}

fn part_1(rules: &Rules, pages: &Pages) -> i64 {
    let mut output: i64 = 0;
    let mut correct: Pages = Vec::new();

    for page in pages {
        let mut is_correct = true;
        for rule in rules {
            let first = rule.0;
            let second = rule.1;

            if page.contains(&first) && page.contains(&second) {
                let first_pos = page.iter().position(|x| x == &first);
                let second_pos = page.iter().position(|x| x == &second);

                // println!(
                //     "{:?}, {:?}, ({:?}, {:?}), {:?}",
                //     page,
                //     rule,
                //     first_pos,
                //     second_pos,
                //     (first_pos > second_pos)
                // );
                if first_pos > second_pos {
                    is_correct = false;
                    break;
                }
            }
        }
        if is_correct {
            correct.push(page.clone());
        }
    }
    for page in correct {
        let middle_index = page.len() / 2;
        output += page[middle_index];
        println!("{:?}", page);
    }

    return output;
}

fn part_2(rules: &Rules, pages: &Pages) -> i64 {
    let mut output: i64 = 0;

    // for i in input {}

    return output;
}

fn get_input() -> Input {
    let args: Vec<String> = env::args().collect();
    let input_path = format!("src/{}", &args[1]);
    let file = File::open(input_path).expect("Failed to open file");
    let lines = BufReader::new(file).lines();

    let mut rules = Vec::new();
    let mut pages = Vec::new();

    let mut still_rules = true;

    for line_res in lines {
        let line = line_res.expect("");

        if line == "" {
            still_rules = false;
        } else if still_rules {
            let nums = line
                .split("|")
                .map(|x| x.parse::<i64>().expect(""))
                .collect::<Vec<i64>>();

            rules.push((nums[0], nums[1]));
        } else {
            let nums = line
                .split(",")
                .map(|x| x.parse::<i64>().expect(""))
                .collect::<Vec<i64>>();

            pages.push(nums);
        }
        // let numbers = line
        //     .split_whitespace()
        //     .map(|x| x.parse::<i64>().expect("parse error"))
        //     .collect::<Vec<i64>>();

        // output.push(numbers);
    }

    return (rules, pages);
}
