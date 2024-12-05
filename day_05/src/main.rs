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
    let correct = pages
        .iter()
        .cloned()
        .filter(|page| is_correct(&page, rules))
        .collect();

    return sum_middle(correct);
}

fn part_2(rules: &Rules, pages: &Pages) -> i64 {
    let corrected = pages
        .iter()
        .cloned()
        .filter(|page| !is_correct(&page, rules))
        .map(|page| apply_rules(&page, rules))
        .map(|page| apply_rules(&page, rules))
        .collect();

    return sum_middle(corrected);
}

fn apply_rules(page: &Vec<i64>, rules: &Rules) -> Vec<i64> {
    let mut output = page.clone();

    for rule in rules {
        output = apply_rule(*rule, &output);
    }

    return output;
}

fn apply_rule(rule: (i64, i64), page: &Vec<i64>) -> Vec<i64> {
    let first = rule.0;
    let second = rule.1;

    let mut output = page.clone();

    if output.contains(&first) && output.contains(&second) {
        let first_pos = output.iter().position(|x| *x == first).expect("") as usize;
        let second_pos = output.iter().position(|x| *x == second).expect("") as usize;

        if first_pos > second_pos {
            output.remove(first_pos);
            output.insert(second_pos, first);
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

fn sum_middle(pages: Pages) -> i64 {
    return pages.iter().map(|page| page[page.len() / 2]).sum();
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
            let nums = split_parse(line, "|".to_string());
            rules.push((nums[0], nums[1]));
        } else {
            pages.push(split_parse(line, ",".to_string()));
        }
    }

    return (rules, pages);
}

fn split_parse(line: String, what: String) -> Vec<i64> {
    return line
        .split(&what)
        .map(|x| x.parse::<i64>().expect(""))
        .collect::<Vec<i64>>();
}
