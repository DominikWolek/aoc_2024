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
type Line = (char, Vec<Location>);
type Frequencies = HashMap<char, Vec<Location>>;

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    // println!("Part 2: {}", part_2(&input));
}

fn part_2(input: &Input) -> i64 {
    let mut output: i64 = 0;

    return output;
}

fn part_1(input: &Input) -> usize {
    let antenas = get_antenas(input);
    // println!("antenas: {:?}", antenas);

    let freqs = get_frequencies(&antenas);
    // println!("freqs: {:?}", freqs);
    let lines = get_lines(&freqs);
    // println!("lines: {:?}", lines);

    let anti = get_antinodes(&lines, input);
    // println!("anti: {:?}", anti);

    // for y in 0..input.len() {
    //     for x in 0..input.len() {
    //         if anti.contains(&(x as i64, y as i64)) {
    //             print!("#")
    //         } else {
    //             print!("{}", input[y][x])
    //         }
    //     }
    //     println!();
    // }
    return anti.len();
}

fn get_antinodes(lines: &Vec<Line>, input: &Input) -> HashSet<Location> {
    let mut out = HashSet::new();
    for line in lines {
        let symbol = line.0;
        let loc_a = line.1[0];
        let loc_b = line.1[1];

        let diff_x = loc_a.0 - loc_b.0;
        let diff_y = loc_a.1 - loc_b.1;

        // println!(
        //     "loc:{:?} diff_x: {:?}, diff y: {:?}",
        //     line.1, diff_x, diff_y
        // );

        let loc_c = (loc_a.0 + diff_x, loc_a.1 + diff_y);
        let loc_d = (loc_b.0 - diff_x, loc_b.1 - diff_y);

        // println!(
        //     "loc:{:?} new_loc:{:?}, new_loc_2:{:?}",
        //     line.1, loc_c, loc_d
        // );

        if check(loc_c, symbol, input) {
            out.insert(loc_c);
        }

        if check(loc_d, symbol, input) {
            out.insert(loc_d);
        }
    }
    return out;
}

fn check(location: Location, symbol: char, input: &Input) -> bool {
    let y = location.1 as usize;
    if y < 0 || y >= input.len() {
        return false;
    }

    let x = location.0 as usize;
    if x < 0 || x >= input[0].len() {
        return false;
    }
    if input[y][x] == symbol {
        return false;
    }

    return true;
}

fn get_lines(freqs: &Frequencies) -> Vec<Line> {
    let naive_pairs: Vec<(&char, Vec<Vec<Location>>)> = freqs
        .iter()
        .map(|(symbol, locations)| (symbol, into_pair(&locations)))
        .collect();

    naive_pairs
        .iter()
        .map(|(symbol, vec_of_pairs)| {
            let out: Vec<(&char, &Vec<Location>)> =
                vec_of_pairs.iter().map(|pairs| (*symbol, pairs)).collect();
            return out;
        })
        .flatten()
        .map(|(symbol, rest)| (*symbol, rest.clone()))
        .collect()
}

fn into_pair(locations: &Vec<Location>) -> Vec<Vec<Location>> {
    let mut out = Vec::new();
    for i in 0..locations.len() {
        for j in i + 1..locations.len() {
            out.push(Vec::from([locations[i], locations[j]]));
        }
    }
    return out;
}

fn get_antenas(input: &Input) -> Vec<Antena> {
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

fn get_frequencies(antenas: &Vec<Antena>) -> Frequencies {
    let mut out: Frequencies = HashMap::new();
    for antena in antenas {
        let v = out.get(&antena.0);
        match v {
            Some(v) => {
                let mut c = v.clone();
                c.push(antena.1);
                out.insert(antena.0, c);
            }
            None => {
                let mut v = Vec::new();
                v.push(antena.1);
                out.insert(antena.0, v);
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
