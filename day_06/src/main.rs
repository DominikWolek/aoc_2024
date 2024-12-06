use std::{
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

type Input = Vec<Vec<char>>;

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn part_1(input: &Input) -> i64 {
    return walk_map(input).expect("");
}

fn walk_map(input: &Input) -> Option<i64> {
    let mut output: i64 = 1;
    let mut cnt: i64 = 1;

    let y_len = input.len();
    let x_len = input[0].len();

    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut dir = Direction::Up;

    for y in 0..y_len {
        for x in 0..x_len {
            if input[y][x] == '^' {
                x_pos = x;
                y_pos = y;
            }
        }
    }

    let mut escape = false;
    let mut new_input = input.clone();
    new_input[y_pos][x_pos] = 'X';

    while !escape {
        if cnt as usize > x_len * y_len * 4 {
            return None;
        }
        cnt += 1;
        match dir {
            Direction::Up => {
                if new_input[y_pos - 1][x_pos] == '#' {
                    dir = Direction::Right;
                } else {
                    y_pos -= 1;
                    if new_input[y_pos][x_pos] == '.' {
                        output += 1;
                        new_input[y_pos][x_pos] = 'X';
                    }
                    if y_pos == 0 {
                        escape = true;
                    }
                }
            }
            Direction::Down => {
                if new_input[y_pos + 1][x_pos] == '#' {
                    dir = Direction::Left;
                } else {
                    y_pos += 1;
                    if new_input[y_pos][x_pos] == '.' {
                        output += 1;
                        new_input[y_pos][x_pos] = 'X';
                    }
                    if y_pos == y_len - 1 {
                        escape = true;
                    }
                }
            }
            Direction::Right => {
                if new_input[y_pos][x_pos + 1] == '#' {
                    dir = Direction::Down;
                } else {
                    x_pos += 1;
                    if new_input[y_pos][x_pos] == '.' {
                        output += 1;
                        new_input[y_pos][x_pos] = 'X';
                    }
                    if x_pos == x_len - 1 {
                        escape = true;
                    }
                }
            }
            Direction::Left => {
                if new_input[y_pos][x_pos - 1] == '#' {
                    dir = Direction::Up;
                } else {
                    x_pos -= 1;
                    if new_input[y_pos][x_pos] == '.' {
                        output += 1;
                        new_input[y_pos][x_pos] = 'X';
                    }
                    if x_pos == 0 {
                        escape = true;
                    }
                }
            }
        }
        // for line in new_input.clone() {
        //     for c in line {
        //         print!("{}", c);
        //     }
        //     println!("");
        // }
        // println!("")
    }

    return Some(output);
}

fn part_2(input: &Input) -> i64 {
    let mut output = 0;

    let y_len = input.len();
    let x_len = input[0].len();

    for x in 0..x_len {
        for y in 0..y_len {
            println!("{}, {}", x, y);
            if input[y][x] == '.' {
                let mut with_obstacle = input.clone();
                with_obstacle[y][x] = '#';

                if walk_map(&with_obstacle).is_none() {
                    output += 1;
                }
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

    let mut output = Vec::new();

    for line_res in lines {
        let line = line_res.expect("");
        output.push(line.chars().collect());
    }

    return output;
}
