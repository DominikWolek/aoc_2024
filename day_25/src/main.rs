use std::{
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

type Height = i8;
type Key = Vec<Height>;
type Lock = Vec<Height>;

type Input = (Vec<Lock>, Vec<Key>);

const GROUP_HEIGHT: usize = 7;

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    println!("Merry Christmas!");
}

fn part_1((locks, keys): &Input) -> i64 {
    let mut output: i64 = 0;

    for lock in locks {
        for key in keys {
            if (0..5).all(|i| lock[i] + key[i] <= 5) {
                output += 1;
            }
        }
    }

    return output;
}

fn get_input() -> Input {
    let args: Vec<String> = env::args().collect();
    let input_path = format!("src/{}", &args[1]);
    let file = File::open(input_path).expect("Failed to open file");
    let lines: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut groupped: Vec<Vec<Vec<char>>> = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let group = lines[i..i + GROUP_HEIGHT]
            .iter()
            .map(|x| x.clone())
            .collect::<Vec<Vec<char>>>();

        groupped.push(group);
        i += GROUP_HEIGHT + 1;
    }

    let mut keys: Vec<Key> = Vec::new();
    let mut locks: Vec<Lock> = Vec::new();

    for group in groupped {
        let mut key = false;
        let mut vals: Vec<i8> = vec![-1, -1, -1, -1, -1];

        if group[0] == vec!['.', '.', '.', '.', '.'] {
            key = true;

            for y in (0..GROUP_HEIGHT).rev() {
                count_height(&group, &mut vals, y);
            }
        } else {
            for y in 0..GROUP_HEIGHT {
                count_height(&group, &mut vals, y);
            }
        }

        if key {
            keys.push(vals);
        } else {
            locks.push(vals);
        }
    }

    (locks, keys)
}

fn count_height(group: &Vec<Vec<char>>, vals: &mut Vec<i8>, y: usize) {
    for x in 0..5 {
        if group[y][x] == '#' {
            vals[x] += 1;
        }
    }
}
