use std::{
    collections::HashSet,
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

fn part_1(input: &Input) -> usize {
    let mut output = 0;

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 0 {
                let mut set: HashSet<(usize, usize)> = HashSet::new();
                for val in calc_value(&input, (x, y)) {
                    set.insert(val);
                }
                output += set.len();
            }
        }
    }

    return output;
}

fn part_2(input: &Input) -> usize {
    let mut output = 0;

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 0 {
                output += calc_value(&input, (x, y)).len();
            }
        }
    }

    return output;
}

fn calc_value(input: &Input, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut stack = vec![(0, pos)];

    let mut last_vals: Vec<(usize, usize)> = Vec::new();

    while !stack.is_empty() {
        let (curr_height, (x, y)) = stack.pop().expect("");

        if curr_height == 9 {
            last_vals.push((x, y));
        } else {
            let x = x as i64;
            let y = y as i64;

            let possible = vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)];

            for (new_x, new_y) in possible {
                if in_bounds(new_x, new_y, input.len(), input[0].len()) {
                    let (new_x, new_y) = (new_x as usize, new_y as usize);
                    if input[new_y][new_x] == curr_height + 1 {
                        stack.push((curr_height + 1, (new_x, new_y)));
                    }
                }
            }
        }
    }
    return last_vals;
}

fn in_bounds(new_x: i64, new_y: i64, y_size: usize, x_size: usize) -> bool {
    new_x >= 0 && new_y >= 0 && new_y < y_size as i64 && new_x < x_size as i64
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
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|x| {
                if *x == '.' {
                    -1
                } else {
                    format!("{}", x).parse::<i64>().expect("parse error")
                }
            })
            .collect::<Vec<i64>>();

        output.push(numbers);
    }

    return output;
}
