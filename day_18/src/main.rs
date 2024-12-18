use std::{
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
    path,
};

use pathfinding::{grid::Grid, prelude::dijkstra};

type Position = (usize, usize);
type Input = Vec<Position>;

const SIZE: usize = 70 + 1;
const FALLEN: usize = 1024;

// const SIZE: usize = 6 + 1;
// const FALLEN: usize = 12;

const START: Position = (0, 0);
const END: Position = (SIZE - 1, SIZE - 1);

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    // println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> i64 {
    let mut grid = input.iter().take(FALLEN).map(|x| *x).collect::<Grid>();

    grid.invert();

    grid.resize(SIZE, SIZE);
    let successors = |pos: &Position| {
        grid.neighbours(*pos)
            .iter()
            .map(|neighbour| (*neighbour, 1))
            .collect::<Vec<_>>()
    };
    match dijkstra(&START, successors, |pos| *pos == END) {
        Some((_path, path_len)) => return path_len,
        None => return 0,
    }
}

fn part_2(input: &Input) -> i64 {
    let mut output: i64 = 0;

    // for i in input {}

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
        let (x, y) = line.split_once(",").unwrap();

        output.push((x.parse().expect(""), y.parse().expect("")));
    }

    return output;
}
