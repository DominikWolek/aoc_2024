use std::{
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

use pathfinding::{
    grid::{self, Grid},
    prelude::dijkstra,
};

type Position = (usize, usize);
type Input = Vec<Vec<char>>;

fn main() {
    let (grid, start, end) = racetrack();

    println!("Part 1: {}", part_1(&grid, &start, &end));
    // println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Grid, start: &Position, end: &Position) -> i64 {
    let mut output: i64 = 0;

    let base_time = best_path(start, end, input).unwrap().1;

    let mut inverted_grid = input.clone();
    inverted_grid.invert();

    let mut grid_with_cheats = input.clone();

    for cheat_start in &inverted_grid {
        grid_with_cheats.add_vertex(cheat_start);

        if let Some(path_to_cheat) = best_path(start, &cheat_start, &grid_with_cheats) {
            let neighbs = grid_with_cheats.neighbours(cheat_start);
            let good_neighbours = neighbs
                .iter()
                .filter(|neighbour| !path_to_cheat.0.contains(neighbour))
                .collect::<Vec<_>>();
            for cheat_end in good_neighbours {
                let time_to_end = best_path(&cheat_end, end, &grid_with_cheats).unwrap().1;

                let cheat_time = base_time as i64 - (path_to_cheat.1 + time_to_end + 1) as i64;
                if cheat_time >= 100 {
                    // println!("{:?} {:?} {:?}", cheat_start, cheat_end, cheat_time)
                    output += 1;
                }
            }
        };
        grid_with_cheats.remove_vertex(cheat_start);
    }

    return output;
}

fn best_path(start: &Position, end: &Position, grid: &Grid) -> Option<(Vec<Position>, usize)> {
    dijkstra(start, succs_fun(grid), |pos| *pos == *end)
}

fn succs_fun(grid: &Grid) -> impl Fn(&Position) -> Vec<(Position, usize)> + use<'_> {
    let successors = |pos: &Position| {
        grid.neighbours(*pos)
            .iter()
            .map(|neighbour| (*neighbour, 1))
            .collect::<Vec<(Position, usize)>>()
    };
    successors
}

fn find(input: &Input, arg: char) -> Option<Position> {
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == arg {
                return Some((x, y));
            }
        }
    }
    return None;
}

// fn part_2(input: &Input) -> i64 {
//     let mut output: i64 = 0;

//     for i in input {}

//     return output;
// }

fn racetrack() -> (Grid, Position, Position) {
    let mut input: Vec<Vec<char>> = get_input();

    let s_cord = find(&input, 'S').unwrap();
    let e_cord = find(&input, 'E').unwrap();

    input[s_cord.1][s_cord.0] = '.';
    input[e_cord.1][e_cord.0] = '.';
    let grid = grid_from_input(&input);

    return (grid, s_cord, e_cord);
}

fn grid_from_input(input: &Input) -> Grid {
    input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let enumerated = line
                .iter()
                .enumerate()
                .map(|(x, c)| (x, y, *c))
                .collect::<Vec<(usize, usize, char)>>();

            enumerated
        })
        .flatten()
        .filter_map(|(x, y, c)| (c == '.').then(|| (x, y)))
        .collect::<Grid>()
}

fn get_input() -> Input {
    let args: Vec<String> = env::args().collect();
    let input_path = format!("src/{}", &args[1]);
    let file = File::open(input_path).expect("Failed to open file");
    let lines = BufReader::new(file).lines();

    let mut output = Vec::new();

    for line_res in lines {
        let line = line_res.expect("");
        let characters = line.chars().collect::<Vec<_>>();

        output.push(characters);
    }

    return output;
}
