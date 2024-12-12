use std::{
    collections::{HashMap, HashSet},
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

type Input = Vec<Vec<char>>;

type Position = (i64, i64);
type RegionID = i64;
type RegionMap = HashMap<Position, RegionID>;

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> usize {
    let mut output = 0;

    let nbs = neighbours_map(input);
    let regions = regions_map(input, &nbs);
    let mut values = HashSet::new();
    for x in regions.values() {
        values.insert(x);
    }
    for key in values {
        output += perimeter(*key, &regions, input) * area(*key, &regions);
    }
    return output;
}

fn part_2(input: &Input) -> usize {
    let mut output = 0;

    let nbs = neighbours_map(input);
    let regions = regions_map(input, &nbs);

    let values: HashSet<i64> = regions.values().map(|x| *x).collect();

    for key in values {
        // let sides_cnt_ = sides_cnt(key, &regions, input);
        let better_sides_cnt_ = sides_cnt(key, &regions, input);
        let area_ = area(key, &regions);
        let (pos, _xx) = regions
            .iter()
            .filter(|(_pos, keyy)| **keyy == key)
            .max()
            .expect("");

        let sides_cnt_ = better_sides_cnt_;
        // let character: char = input[pos.1 as usize][pos.0 as usize];
        // println!("{character}({key}): area {area_} * {better_sides_cnt_} sides\n");
        output += area_ * sides_cnt_;

        match is_enclave(key, &regions) {
            Some(outer_key) => {
                let outer_area_ = area(outer_key, &regions);
                // println!("It's enclave of {outer_key} with outer area of {outer_area_}");
                output += outer_area_ * sides_cnt_;
            }
            None => {}
        }
    }

    return output;
}

fn is_enclave(key: i64, regions: &RegionMap) -> Option<i64> {
    let mut neighbour: Option<i64> = None;
    let positions = get_positions_by_key(regions, key);

    for pos in positions {
        let nb_regions: Vec<Option<&i64>> =
            all_neighbours(pos).iter().map(|x| regions.get(x)).collect();

        for x in nb_regions {
            match x {
                Some(nb_key) => {
                    if *nb_key == key {
                        continue;
                    } else {
                        match neighbour {
                            Some(old_nb_key) => {
                                if old_nb_key != *nb_key {
                                    return None;
                                }
                            }
                            None => neighbour = Some(*nb_key),
                        }
                    }
                }
                None => {
                    return None;
                }
            }
        }
    }

    neighbour
}

fn area(key: i64, regions: &RegionMap) -> usize {
    get_positions_by_key(regions, key).len()
}

fn perimeter(key: i64, regions: &RegionMap, input: &Input) -> usize {
    outer_sides(regions, key, input).len()
}

fn sides_cnt(key: i64, regions: &RegionMap, input: &Input) -> usize {
    let mut positions = get_positions_by_key(regions, key);
    positions.sort();

    let mut turns = 0;

    let start_pos = positions[0];
    let mut curr_pos = start_pos.clone();
    let mut curr_dir = Dir::Up;

    loop {
        // println!("[{turns}] {:?} {:?}", curr_pos, curr_dir);

        let next_pos_ = next_pos(curr_pos, &curr_dir);
        let to_right = next_dir(&curr_dir);
        let to_back = opossite_dir(&curr_dir);
        let to_left = next_dir(&to_back);

        let to_right_pos = &next_pos(curr_pos, &to_right);
        let to_left_pos = &next_pos(curr_pos, &to_left);

        if positions.contains(to_left_pos) {
            curr_dir = to_left;
            curr_pos = *to_left_pos;
            turns += 1;
        } else if positions.contains(&next_pos_) {
            curr_pos = next_pos_;
        } else {
            if positions.contains(to_right_pos) {
                curr_dir = to_right;
                curr_pos = *to_right_pos;
                turns += 1;
            } else if positions.contains(to_left_pos) {
                curr_dir = to_left;
                curr_pos = *to_left_pos;
                turns += 1;
            } else {
                curr_dir = to_back;
                turns += 2;
            }
        }
        if curr_pos == start_pos && (curr_dir == Dir::Up || curr_dir == Dir::Right) {
            // println!("ended with {:?} and [{turns}]", curr_dir);
            return turns - (turns % 2);
        }
    }
}

// fn sides_cnt(key: i64, regions: &RegionMap, input: &Input) -> usize {
//     let mut positions = get_positions_by_key(regions, key);
//     positions.sort();

//     let start_pos = positions[0];
//     println!(
//         "CHARACTER: {:?}",
//         input[start_pos.1 as usize][start_pos.0 as usize]
//     );
//     let mut cur_pos = start_pos.clone();
//     cur_pos = (cur_pos.0 - 1, cur_pos.1);
//     return get_sides_maps(
//         cur_pos,
//         Dir::Up,
//         cur_pos,
//         positions,
//         0, // &mut (&mut x_sides, &mut y_sides),
//     );
// }

// fn get_sides_maps(
//     pos: Position,
//     dir: Dir,
//     start_pos: Position,
//     positions: Vec<Position>,
//     output: usize, // sides: &mut (&mut SidesMap, &mut SidesMap),
// ) -> usize {
//     println!("[{output}]{:?} going {:?}", pos, dir);
//     let mut next_d = dir.clone();
//     let mut next_p = next_pos(pos, &dir);
//     let inside_on_right = next_pos(pos, &next_dir(&dir));
//     let mut next_output = output;

//     if !positions.contains(&inside_on_right) {
//         println!("TURN, THEN AHEAD");
//         next_d = next_dir(&dir);
//         next_p = next_pos(pos, &next_d);
//         next_output += 1;
//     } else if positions.contains(&next_p) {
//         println!("TURN");
//         let d_next = next_dir(&dir);
//         let d_escape = next_dir(&d_next);
//         let d_next_opposite = next_dir(&d_escape);

//         if !positions.contains(&next_pos(pos, &d_next)) {
//             next_d = d_next;
//             next_output += 1;
//         } else if !positions.contains(&next_pos(pos, &d_next_opposite)) {
//             next_d = d_next_opposite;
//             next_output += 1;
//         } else {
//             next_d = d_escape;
//             next_output += 2;
//             //escape
//         }
//         next_p = pos;
//     }

//     if next_p == start_pos {
//         println!("THE END");
//         return next_output;
//     }
//     return get_sides_maps(next_p, next_d, start_pos, positions, next_output);
// }

fn next_dir(dir: &Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Right,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Up,
        Dir::Right => Dir::Down,
    }
}

fn next_pos((x, y): Position, dir: &Dir) -> Position {
    match dir {
        Dir::Up => (x, y - 1),
        Dir::Down => (x, y + 1),
        Dir::Left => (x - 1, y),
        Dir::Right => (x + 1, y),
    }
}

fn opossite_dir(dir: &Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Down,
        Dir::Down => Dir::Up,
        Dir::Left => Dir::Right,
        Dir::Right => Dir::Left,
    }
}
fn outer_sides(regions: &RegionMap, key: i64, input: &Input) -> Vec<(i64, i64)> {
    let mut outer_sides = Vec::new();
    let positions = get_positions_by_key(regions, key);

    for pos in positions {
        for (n_x, n_y) in all_neighbours(pos) {
            if n_x < 0 || n_y < 0 || n_y >= input.len() as i64 || n_x >= input[0].len() as i64 {
                outer_sides.push((n_x, n_y));
            } else {
                if *regions.get(&(n_x, n_y)).unwrap() != key {
                    outer_sides.push((n_x, n_y));
                }
            }
        }
    }

    return outer_sides;
}

fn get_positions_by_key(regions: &RegionMap, key: i64) -> Vec<Position> {
    regions
        .iter()
        .filter(|(_, keyy)| **keyy == key)
        .map(|(x, _y)| *x)
        .collect()
}

fn regions_map(input: &Input, nbs_map: &NeighbourMap) -> RegionMap {
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
                let curr_plant = input[curr_pos.1 as usize][curr_pos.0 as usize];
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
            let pos = (curr_x as i64, curr_y as i64);
            let plant = input[curr_y][curr_x];

            mapa.insert(pos, good_neighbours(pos, plant, input));
        }
    }
    return mapa.clone();
}

#[derive(Clone, PartialEq, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn good_neighbours(pos: Position, plant: char, input: &Vec<Vec<char>>) -> Vec<Position> {
    return neighbours(pos, input.len(), input[0].len())
        .iter()
        .filter(|(x, y)| input[*y as usize][*x as usize] == plant)
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
    all_neighbours((curr_x, curr_y))
        .iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < max_x as i64 && *y < max_y as i64)
        .map(|(x, y)| (*x, *y))
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
