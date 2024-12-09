use std::{
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
    process::Output,
};

type Input = Vec<i16>;
type DiscMap = Vec<i16>;

const EMPTY: i16 = -1;

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    // println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> usize {
    let mut disc_map: DiscMap = get_map(input);

    let len = disc_map.len();
    let mut free = 0;

    let mut take = len - 1;

    loop {
        for i in 0..len {
            if disc_map[i] == EMPTY {
                free = i;
                break;
            }
        }

        let mut reversed: Vec<usize> = (0..len).collect();
        reversed.reverse();

        for i in reversed {
            if disc_map[i] != EMPTY {
                take = i;
                break;
            }
        }

        if free < take {
            disc_map[free] = disc_map[take];
            disc_map[take] = EMPTY;
        } else {
            break;
        }
        println!("free {} take {} len {}", free, take, len);
    }

    return calc_checksum(&disc_map);
}

fn calc_checksum(disc_map: &DiscMap) -> usize {
    for i in disc_map.clone() {
        print!("{}", i);
    }
    println!();

    let mut curr_ID = 0;
    let mut output = 0;

    for i in disc_map {
        if *i != EMPTY {
            let num: usize = format!("{}", i).parse().expect("");

            output += num * curr_ID;
            curr_ID += 1;

            println!("{}", output);
        } else {
            break;
        }
    }

    return output;
}

fn get_map(input: &[i16]) -> DiscMap {
    let mut output = DiscMap::new();
    let mut ids = true;
    let mut current_ID = 0;

    for i in input {
        if ids {
            for _ in 0..*i {
                output.push(current_ID);
            }
            current_ID += 1;
        } else {
            for _ in 0..*i {
                output.push(EMPTY);
            }
        }
        ids = !ids;
    }
    return output;
}

fn part_2(input: &Input) -> usize {
    let mut output: i64 = 0;

    // for i in input {}

    return output as usize;
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
            .map(|x| format!("{}", x).parse::<i16>().expect(""))
            .collect::<Vec<i16>>();

        output.push(numbers);
    }

    let output = output[0].clone();

    return output;
}
