use std::{
    collections::{HashMap, HashSet},
    env::{self},
    fs::File,
    io::Read,
    iter::zip,
};

#[derive(Debug, Clone, PartialEq)]
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
    println!("Part 2: {}", part_2(&mapa, &movements));
}

fn part_1(mapa: &Mapa, movements: &Movements) -> usize {
    let mut curr_map = mapa.clone();
    let start_pos = find_start_pos(&curr_map);

    walk_1(movements, start_pos, &mut curr_map);

    return calc_value(&curr_map);
}

fn part_2(mapa: &Mapa, movements: &Movements) -> usize {
    let mut curr_map = wide(mapa);
    print_map(&mut curr_map);

    let start_pos = find_start_pos(&curr_map);
    walk_2(movements, start_pos, &mut curr_map);
    return calc_value(&curr_map);
}

fn wide(mapa: &Mapa) -> Mapa {
    let mut output = HashMap::new();

    for ((x, y), val) in mapa {
        let first = (2 * x, *y);
        let second = (2 * x + 1, *y);
        let (first_val, second_val) = match val {
            '#' => ('#', '#'),
            'O' => ('[', ']'),
            '.' => ('.', '.'),
            '@' => ('@', '.'),
            _ => panic!(),
        };
        output.insert(first, first_val);
        output.insert(second, second_val);
    }

    return output;
}

fn walk_2(movements: &Movements, start_pos: Position, curr_map: &mut Mapa) {
    let mut curr_pos = start_pos;
    for mov in movements.clone() {
        if mov == Dir::Left || mov == Dir::Right {
            curr_pos = step_1(curr_pos, &mov, curr_map);
        } else {
            curr_pos = step_2(curr_pos, &mov, curr_map);
        }
    }
}

fn step_2(pos: Position, dir: &Dir, mapa: &mut Mapa) -> Position {
    let starting_positions = starting_positions(pos, dir, mapa);
    let output = if starting_positions.is_empty() {
        pos
    } else {
        next_pos(pos, dir)
    };

    let mut sorted = starting_positions.iter().collect::<Vec<&Position>>();
    sorted.sort_by(|(_, y_left), (_, y_right)| {
        if *dir == Dir::Down {
            return (y_right).cmp(y_left);
        } else {
            return y_left.cmp(y_right);
        }
    });

    // println!("{:?}", sorted);

    for start_pos in sorted {
        let val = mapa.get(start_pos).unwrap().clone();
        let next_pos_ = next_pos(*start_pos, dir);
        // println!("{:?}, val: {val}", start_pos);
        match val {
            '[' => {
                mapa.insert(next_pos_, '[');
                mapa.insert(*start_pos, '.');

                let side_pos = other_box_pos(*start_pos, '[');
                let next_side_pos = next_pos(side_pos, dir);

                mapa.insert(next_side_pos, ']');
                mapa.insert(side_pos, '.');
            }
            '@' => {
                mapa.insert(next_pos_, '@');
                mapa.insert(*start_pos, '.');
            }
            _ => panic!(),
        }
        // print_map(&mapa);
    }

    return output;
}

fn starting_positions(pos: Position, dir: &Dir, mapa: &Mapa) -> HashSet<Position> {
    let mut output = HashSet::new();

    let mut stack = vec![pos];

    while !stack.is_empty() {
        let curr_pos = stack.pop().unwrap();
        let val = mapa.get(&curr_pos).unwrap();

        match val {
            '.' => continue,
            '@' => {
                output.insert(curr_pos);
                stack.push(next_pos(curr_pos, dir));
            }
            '[' => {
                let other_pos = other_box_pos(curr_pos, '[');
                // check_and_push(&mut output, &pos);
                output.insert(curr_pos);
                stack.push(next_pos(curr_pos, dir));
                stack.push(next_pos(other_pos, dir));
            }
            ']' => {
                let other_pos = other_box_pos(curr_pos, ']');
                output.insert(other_pos);
                stack.push(next_pos(curr_pos, dir));
                stack.push(next_pos(other_pos, dir));
            }
            '#' => {
                return HashSet::new();
            }
            _ => panic!(),
        }
    }

    return output;
}

fn other_box_pos((x, y): Position, val: char) -> Position {
    return match val {
        '[' => (x + 1, y),
        ']' => (x - 1, y),
        _ => panic!(),
    };
}

fn walk_1(movements: &Movements, start_pos: Position, curr_map: &mut Mapa) {
    let mut curr_pos = start_pos;
    for mov in movements.clone() {
        curr_pos = step_1(curr_pos, &mov, curr_map);
    }
}

fn step_1(curr_pos: Position, dir: &Dir, curr_map: &mut HashMap<(usize, usize), char>) -> Position {
    if let Some(empty) = get_next_empty(curr_pos, &dir, curr_map) {
        let mut cpy = *curr_map.get(&curr_pos).expect("");
        let mut to = next_pos(curr_pos, &dir);
        let mut mem = *curr_map.get(&to).expect("");
        loop {
            curr_map.insert(to, cpy);
            if to == empty {
                break;
            }

            to = next_pos(to, &dir);
            cpy = mem;
            mem = *curr_map.get(&to).expect("");
        }
        curr_map.insert(curr_pos, '.');
        return next_pos(curr_pos, &dir);
    }
    return curr_pos;
}

fn print_map(curr_map: &Mapa) {
    let max_x = curr_map.iter().map(|((x, _), _val)| *x).max().expect("");
    let max_y = curr_map.iter().map(|((_, y), _val)| *y).max().expect("");

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

fn calc_value(curr_map: &Mapa) -> usize {
    return curr_map
        .into_iter()
        .filter(|(_pos, val)| **val == 'O' || **val == '[')
        .map(|((x, y), _val)| 100 * y + x)
        .sum::<usize>();
}

fn get_input() -> Input {
    let args: Vec<String> = env::args().collect();
    let input_path = format!("src/{}", &args[1]);
    let mut file = File::open(input_path).expect("Failed to open file");
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    let splitted: Vec<&str> = buf.split("\n\n").collect();
    let mapa = parse_map(splitted[0]);
    let movements = parse_movements(splitted[1]);

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
            out.insert((x, y), val);
        }
    }
    return out;
}
