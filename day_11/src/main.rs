use std::{
    collections::HashMap,
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

use rayon::iter::*;

type Input = Vec<usize>;
type MemResults = HashMap<(usize, i32), usize>;

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&mut input.clone()));
    println!("Part 2: {}", part_2(&mut input.clone()));
}

fn part_1(input: &mut Input) -> usize {
    stones_cnt(input, 25)
}

fn part_2(input: &mut Input) -> usize {
    stones_cnt(input, 75)
}

fn val_len(val: usize) -> u32 {
    val.ilog10() + 1
}

fn split_into_halfs(val: usize) -> (usize, usize) {
    let val_len = val_len(val);
    let half_len = val_len / 2;
    let pow_10 = 10_usize.pow(half_len);
    let val_pref = val / 10_usize.pow(half_len);
    let val_suff = val - val_pref * pow_10;

    return (val_suff, val_pref);
}

fn stones_cnt(input: &mut Vec<usize>, start_gen_cnt: i32) -> usize {
    input
        .par_iter()
        .map(|x| do_step(*x, start_gen_cnt, &mut HashMap::new()))
        .sum()
}

fn do_step(curr: usize, gen_cnt: i32, results: &mut MemResults) -> usize {
    let result;
    if gen_cnt == 0 {
        return 1;
    }

    match results.get(&(curr, gen_cnt)) {
        Some(prev_result) => return *prev_result,
        None => {
            let new_cnt = gen_cnt - 1;
            if curr == 0 {
                result = do_step(1, new_cnt, results);
            } else if val_len(curr) % 2 == 0 {
                let (val_1, val_2) = split_into_halfs(curr);
                result = do_step(val_1, new_cnt, results) + do_step(val_2, new_cnt, results);
            } else {
                result = do_step(curr * 2024, new_cnt, results)
            }
            results.insert((curr, gen_cnt), result);
            return result;
        }
    }
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
            .map(|x| x.parse::<usize>().expect("parse error"))
            .collect::<Vec<usize>>();

        output.push(numbers);
    }

    return output[0].clone();
}
