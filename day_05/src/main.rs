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
        if is_correct(page, rules) {
            correct.push(page.clone());
        }
    }

    for page in correct {
        let middle_index = page.len() / 2;
        output += page[middle_index];
    }

    return output;
}

fn part_2(rules: &Rules, pages: &Pages) -> i64 {
    let mut output: i64 = 0;
    let mut incorrect: Pages = Vec::new();

    for page in pages {
        if !is_correct(page, rules) {
            incorrect.push(page.clone());
        }
    }

    let mut updated: Pages = Vec::new();
    for page in incorrect {
        updated.push(apply_rules(&page, rules));
    }

    let mut updated_1: Pages = Vec::new();
    for page in updated {
        println!("{:?}", page);
        updated_1.push(apply_rules(&page, rules));
    }

    for page in updated_1 {
        println!("{:?}", page);
        let middle_index = page.len() / 2;
        output += page[middle_index];
    }

    return output;
}

fn apply_rules(page: &Vec<i64>, rules: &Rules) -> Vec<i64> {
    let mut output = page.clone();

    for rule in rules {
        let first = rule.0;
        let second = rule.1;

        if output.contains(&first) && output.contains(&second) {
            let first_pos = output.iter().position(|x| *x == first).expect("") as usize;
            let second_pos = output.iter().position(|x| *x == second).expect("") as usize;

            if first_pos > second_pos {
                output.remove(first_pos);
                output.insert(second_pos, first);
            }
        }
    }

    return output;
}

fn is_correct(page: &Vec<i64>, rules: &Rules) -> bool {
    let mut output = true;
    for rule in rules {
        let first = rule.0;
        let second = rule.1;

        if page.contains(&first) && page.contains(&second) {
            let first_pos = page.iter().position(|x| x == &first);
            let second_pos = page.iter().position(|x| x == &second);

            if first_pos > second_pos {
                output = false;
                break;
            }
        }
    }
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
