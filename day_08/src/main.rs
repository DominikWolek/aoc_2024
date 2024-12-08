use std::{
    collections::HashMap,
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

use std::collections::HashSet;

type Input = Vec<Vec<char>>;
type Location = (i64, i64);
type Antena = (char, Location);
type Pair = (Location, Location);
type Line = (char, Pair);
type Frequencies = HashMap<char, Vec<Location>>;

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> usize {
    let lines = lines(input);
    let anti = antinodes(&lines, input);
    return anti.len();
}

fn part_2(input: &Input) -> usize {
    let lines = lines(input);
    let anti = all_antinodes(&lines, input);
    return anti.len();
}

fn antinodes(lines: &Vec<Line>, input: &Input) -> HashSet<Location> {
    let mut out = HashSet::new();

    for line in lines {
        let symbol = line.0;
        let locs = antinodes_impl(line.1);

        for loc in locs {
            if check(loc, symbol, input) {
                out.insert(loc);
            }
        }
    }
    return out;
}

fn all_antinodes(lines: &Vec<Line>, input: &Input) -> HashSet<Location> {
    let mut out = HashSet::new();
    for line in lines {
        let locs = all_antinodes_impl(line.1, input);
        for loc in locs {
            out.insert(loc);
        }
    }
    return out;
}

fn all_antinodes_impl((loc_a, loc_b): Pair, input: &Input) -> HashSet<Location> {
    let diff_x = loc_a.0 - loc_b.0;
    let diff_y = loc_a.1 - loc_b.1;

    let mut out = HashSet::new();
    let mut cur: (i64, i64) = loc_a;
    while check_bounds(cur, input) {
        out.insert(cur);
        cur = (cur.0 + diff_x, cur.1 + diff_y);
    }

    cur = loc_a;
    while check_bounds(cur, input) {
        out.insert(cur);
        cur = (cur.0 - diff_x, cur.1 - diff_y);
    }

    return out;
}

fn check_bounds(location: Location, input: &Input) -> bool {
    let y = location.1;
    if y < 0 || y >= input.len() as i64 {
        return false;
    }

    let x = location.0;
    if x < 0 || x >= input[0].len() as i64 {
        return false;
    }
    return true;
}

fn antinodes_impl((loc_a, loc_b): Pair) -> Vec<Location> {
    let diff_x = loc_a.0 - loc_b.0;
    let diff_y = loc_a.1 - loc_b.1;

    let loc_c = (loc_a.0 + diff_x, loc_a.1 + diff_y);
    let loc_d = (loc_b.0 - diff_x, loc_b.1 - diff_y);

    vec![loc_c, loc_d]
}

fn check(location: Location, symbol: char, input: &Input) -> bool {
    let y = location.1 as usize;
    let x = location.0 as usize;
    return check_bounds(location, input) && !(input[y][x] == symbol);
}

fn lines(input: &Input) -> Vec<Line> {
    let antenas = antenas(input);
    let freqs = frequencies(&antenas);

    freqs
        .iter()
        .map(|(symbol, locations)| (*symbol, into_pairs(&locations)))
        .map(|(symbol, vec_of_pairs)| {
            vec_of_pairs
                .iter()
                .map(|pairs| (symbol, *pairs))
                .collect::<Vec<Line>>()
        })
        .flatten()
        .collect()
}

fn into_pairs(locations: &Vec<Location>) -> Vec<Pair> {
    let mut out = Vec::new();
    for i in 0..locations.len() {
        for j in i + 1..locations.len() {
            out.push((locations[i], locations[j]));
        }
    }
    return out;
}

fn antenas(input: &Input) -> Vec<Antena> {
    let mut out = Vec::new();

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] != '.' {
                out.push((input[y][x], (x as i64, y as i64)));
            }
        }
    }

    return out;
}

fn frequencies(antenas: &Vec<Antena>) -> Frequencies {
    let mut out: Frequencies = HashMap::new();
    for (symbol, location) in antenas {
        let current = out.get(&symbol);
        match current {
            Some(current) => {
                let mut current = current.clone();
                current.push(*location);
                out.insert(*symbol, current);
            }
            None => {
                out.insert(*symbol, vec![*location]);
            }
        }
    }
    return out;
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
