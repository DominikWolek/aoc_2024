use std::{
    collections::{HashMap, HashSet},
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

type Input = Vec<Vec<char>>;

type Position = (usize, usize);
type RegionID = i64;
type RegionMap = HashMap<Position, RegionID>;

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    // println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> usize {
    let mut output = 0;

    let nbs = neighbours_map(input);
    let regions = get_regions(input, &nbs);

    let mut values = HashSet::new();
    for x in regions.values() {
        values.insert(x);
    }
    for key in values {
        output += perimeter(*key, &regions, &nbs, input) * area(*key, &regions, &nbs);
    }
    return output as usize;
}

fn area(key: i64, regions: &RegionMap, nbs: &NeighbourMap) -> i32 {
    let mut out = 0;
    for (_, val) in regions {
        if *val == key {
            out += 1;
        }
    }
    return out;
}

fn perimeter(key: i64, reg_map: &RegionMap, nbs: &NeighbourMap, input: &Input) -> i32 {
    let mut set: Vec<((i64, i64))> = Vec::new();
    let filtered_positions = get_positions_by_key(reg_map, key);

    for (pos, _val) in filtered_positions {
        for (n_x, n_y) in all_neighbours(pos) {
            if n_x < 0 || n_y < 0 || n_y >= input.len() as i64 || n_x >= input[0].len() as i64 {
                set.push((n_x, n_y));
            } else {
                if *reg_map.get(&(n_x as usize, n_y as usize)).unwrap() != key {
                    set.push((n_x, n_y));
                }
            }
        }
    }

    return set.len() as i32;
}

fn get_positions_by_key(regions: &RegionMap, key: i64) -> Vec<((usize, usize), i64)> {
    regions
        .iter()
        .filter(|(_, keyy)| **keyy == key)
        .map(|(x, y)| (*x, *y))
        .collect()
}

fn get_regions(input: &Input, nbs_map: &NeighbourMap) -> RegionMap {
    do_get_regions(input, nbs_map, 0)
}

fn do_get_regions(input: &Input, nbs_map: &NeighbourMap, start_id: i64) -> RegionMap {
    let mut out: RegionMap = HashMap::new();
    let mut id = start_id;
    for (pos, _nbs) in nbs_map {
        if !out.contains_key(pos) {
            let mut stack: Vec<(Position, RegionID)> = vec![(*pos, id)];
            let mut visited: HashSet<Position> = HashSet::new();
            while !stack.is_empty() {
                let (curr_pos, curr_id) = stack.pop().unwrap();
                visited.insert(curr_pos);
                out.insert(curr_pos, curr_id);
                let curr_plant = input[curr_pos.1][curr_pos.0];
                for nb in good_neighbours(curr_pos, curr_plant, input) {
                    if !visited.contains(&nb) {
                        stack.push((nb, curr_id));
                    }
                }
            }
        }
        id += 1;
    }
    return out;
}

type NeighbourMap = HashMap<Position, Vec<Position>>;

fn neighbours_map(input: &Input) -> NeighbourMap {
    let mapa: &mut NeighbourMap = &mut HashMap::new();
    for curr_y in 0..input.len() {
        for curr_x in 0..input[0].len() {
            let pos = (curr_x, curr_y);
            let plant = input[curr_y][curr_x];

            mapa.insert(pos, good_neighbours(pos, plant, input));
        }
    }
    return mapa.clone();
}

fn good_neighbours(pos: Position, plant: char, input: &Vec<Vec<char>>) -> Vec<Position> {
    return neighbours(pos, input.len(), input[0].len())
        .iter()
        .filter(|(x, y)| input[*y][*x] == plant)
        .map(|(x, y)| (*x, *y))
        .collect();
}

fn all_neighbours((x, y): Position) -> Vec<(i64, i64)> {
    vec![
        (x as i64 + 1, y as i64),
        (x as i64 - 1, y as i64),
        (x as i64, y as i64 + 1),
        (x as i64, y as i64 - 1),
    ]
}

fn neighbours((curr_x, curr_y): Position, max_y: usize, max_x: usize) -> Vec<Position> {
    let x = curr_x as i64;
    let y = curr_y as i64;

    vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
        .iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < max_x as i64 && *y < max_y as i64)
        .map(|(x, y)| (*x as usize, *y as usize))
        .collect()
}

fn get_input() -> Input {
    let args: Vec<String> = env::args().collect();
    let input_path = format!("src/{}", &args[1]);
    let file = File::open(input_path).expect("Failed to open file");
    let lines = BufReader::new(file).lines();

    let mut output = Vec::new();

    for line_res in lines {
        let line = line_res.expect("");
        let x = line.chars().collect::<Vec<char>>();

        output.push(x);
    }

    return output;
}
