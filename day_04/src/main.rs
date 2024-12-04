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

fn part_1(input: &Input) -> i64 {
    let mut output: i64 = 0;

    let y_len = input.len();
    let x_len = input[0].len();

    for y in 0..y_len - 3 {
        for x in 0..x_len {
            output += check_xmas(
                input[y][x],
                input[y + 1][x],
                input[y + 2][x],
                input[y + 3][x],
            );
        }
    }

    for y in 0..y_len {
        for x in 0..x_len - 3 {
            output += check_xmas(
                input[y][x],
                input[y][x + 1],
                input[y][x + 2],
                input[y][x + 3],
            );
        }
    }

    for y in 0..y_len - 3 {
        for x in 0..x_len - 3 {
            output += check_xmas(
                input[y][x],
                input[y + 1][x + 1],
                input[y + 2][x + 2],
                input[y + 3][x + 3],
            );
            output += check_xmas(
                input[y + 3][x],
                input[y + 2][x + 1],
                input[y + 1][x + 2],
                input[y][x + 3],
            );
        }
    }

    return output;
}

fn part_2(input: &Input) -> i64 {
    let mut output: i64 = 0;

    let y_len = input.len();
    let x_len = input[0].len();

    for y in 0..y_len - 2 {
        for x in 0..x_len - 2 {
            output += check_x_mas(
                input[y][x],
                input[y + 1][x + 1],
                input[y + 2][x + 2],
                input[y][x + 2],
                input[y + 2][x],
            );
        }
    }

    return output;
}

fn check_xmas(i_0: char, i_1: char, i_2: char, i_3: char) -> i64 {
    let s = &(format!("{}{}{}{}", i_0, i_1, i_2, i_3));
    if s == "XMAS" || s == "SAMX" {
        return 1;
    }
    return 0;
}

fn check_x_mas(i_0: char, i_1: char, i_2: char, i_3: char, i_4: char) -> i64 {
    let s_1 = &(format!("{}{}{}", i_0, i_1, i_2));
    let s_2 = &(format!("{}{}{}", i_3, i_1, i_4));

    if (s_1 == "MAS" || s_1 == "SAM") && (s_2 == "MAS" || s_2 == "SAM") {
        return 1;
    }
    return 0;
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
