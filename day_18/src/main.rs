use std::{
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

use pathfinding::{grid::Grid, prelude::dijkstra};

type Position = (usize, usize);
type Input = Vec<Position>;

const SIZE: usize = 70 + 1;
const FALLEN: usize = 1024;

const START: Position = (0, 0);
const END: Position = (SIZE - 1, SIZE - 1);

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {:?}", part_2(&input).unwrap());
}

fn part_1(input: &Input) -> usize {
    let mut grid: Grid = input.iter().take(FALLEN).map(|x| *x).collect::<Grid>();

    grid.invert();
    grid.resize(SIZE, SIZE);

    match dijkstra(&START, succs_fun(grid), is_end_fun()) {
        Some((_path, path_len)) => return path_len,
        None => return 0,
    }
}

fn part_2(input: &Input) -> Option<Position> {
    for fallen_cnt in 1..=input.len() {
        let grid = grid_from_input(input, fallen_cnt);
        if dijkstra(&START, succs_fun(grid), is_end_fun()).is_none() {
            return Some(input[fallen_cnt - 1]);
        }
    }

    return None;
}

fn grid_from_input(input: &Vec<Position>, fallen: usize) -> Grid {
    let mut grid: Grid = input.iter().take(fallen).map(|x| *x).collect::<Grid>();
    grid.resize(SIZE, SIZE);
    grid.invert();

    return grid;
}

fn is_end_fun() -> impl FnMut(&(usize, usize)) -> bool {
    |pos| *pos == END
}

fn succs_fun(grid: Grid) -> impl Fn(&Position) -> Vec<(Position, usize)> {
    let successors = move |pos: &Position| {
        grid.neighbours(*pos)
            .iter()
            .map(|neighbour| (*neighbour, 1))
            .collect::<Vec<(Position, usize)>>()
    };
    successors
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
