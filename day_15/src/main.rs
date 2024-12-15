use core::hash;
use std::{
    collections::HashMap,
    env::{self},
    fs::File,
    io::{BufRead, BufReader, Read},
    iter::{zip, Map},
    usize,
};

#[derive(Debug, Clone)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

type Position = (usize, usize);
type Mapa = HashMap<Position, char>;
type Movements = Vec<Dir>;

type Input = (Mapa, Movements);

fn main() {
    let (mapa, movements) = get_input();
    println!("Part 1: {}", part_1(&mapa, &movements));
    // println!("Part 2: {}", part_2(&input));
}

fn part_1(mapa: &Mapa, movements: &Movements) -> usize {
    let mut output: i64 = 0;

    let mut curr_map = mapa.clone();
    let start_pos = find_start_pos(&curr_map);

    walk(movements, start_pos, &mut curr_map);

    return calc_value(curr_map);
}

fn walk(movements: &Movements, start_pos: Position, curr_map: &mut Mapa) {
    // println!("STARTING");
    // print_map(curr_map);
    // println!("{:?}", start_pos);
    // println!();

    let mut curr_pos = start_pos;
    for mov in movements.clone() {
        // println!("{:?}: {:?}", mov, curr_pos);

        match get_next_empty(curr_pos, &mov, curr_map) {
            Some(empty) => {
                // dbg!(empty);
                let mut cpy = *curr_map.get(&curr_pos).expect("");
                let mut to = next_pos(curr_pos, &mov);
                let mut mem = *curr_map.get(&to).expect("");
                loop {
                    curr_map.insert(to, cpy);
                    if to == empty {
                        break;
                    }

                    to = next_pos(to, &mov);
                    cpy = mem;
                    mem = *curr_map.get(&to).expect("");
                }
                curr_map.insert(curr_pos, '.');
                curr_pos = next_pos(curr_pos, &mov);
            }
            _ => (),
        }
        // print_map(curr_map);
        // println!();
    }
}

fn print_map(curr_map: &mut Mapa) {
    let max_x = curr_map.iter().map(|((x, _), _val)| *x).max().expect("");
    let max_y = curr_map.iter().map(|((_, y), _val)| *y).max().expect("");

    // println!("{:?}", curr_map);
    // println!("{max_x}");
    // println!("{max_y}");
    for y in 0..=max_y {
        for x in 0..=max_x {
            let val = *curr_map.get(&(x, y)).expect("no value");
            print!("{val}");
        }
        println!();
    }
}

fn get_next_empty(mut check_pos: Position, mov: &Dir, curr_map: &mut Mapa) -> Option<Position> {
    loop {
        check_pos = next_pos(check_pos, mov);

        let val = curr_map.get(&check_pos).expect("");
        if *val == '#' {
            return None;
        } else if *val == '.' {
            return Some(check_pos);
        }
    }
}

fn find_start_pos(curr_map: &Mapa) -> Position {
    for (pos, val) in curr_map.clone() {
        if val == '@' {
            return pos;
        }
    }

    panic!()
}

fn next_pos((x, y): Position, dir: &Dir) -> Position {
    match dir {
        Dir::Up => (x, y - 1),
        Dir::Down => (x, y + 1),
        Dir::Right => (x + 1, y),
        Dir::Left => (x - 1, y),
    }
}

fn calc_value(curr_map: Mapa) -> usize {
    return curr_map
        .into_iter()
        .filter(|(_pos, val)| *val == 'O')
        .map(|((x, y), val)| 100 * y + x)
        .sum::<usize>();
}

fn get_input() -> Input {
    let args: Vec<String> = env::args().collect();
    let input_path = format!("src/{}", &args[1]);
    let mut file = File::open(input_path).expect("Failed to open file");
    let mut buf = String::new();
    file.read_to_string(&mut buf);

    let splitted: Vec<&str> = buf.split("\n\n").collect();
    let mapa = parse_map(splitted[0]);
    let movements = parse_movements(splitted[1]);

    // println!("{:?}", mapa);

    return (mapa, movements);
}

fn parse_movements(splitted: &str) -> Vec<Dir> {
    splitted
        .chars()
        .filter(|x| *x != '\n')
        .map(|x| match x {
            '^' => Dir::Up,
            'v' => Dir::Down,
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => panic!(),
        })
        .collect()
}

fn parse_map(map_str: &str) -> Mapa {
    let splitted = map_str.split("\n");

    let mut out = HashMap::new();
    for (line, y) in zip(splitted, 0..100) {
        for (val, x) in zip(line.chars(), 0..100) {
            // println!("{val}");
            out.insert((x, y), val);
        }
    }
    return out;
}
