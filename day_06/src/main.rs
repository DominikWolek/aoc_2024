use std::{
    collections::HashSet,
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

type Input = Vec<Vec<char>>;
type Position = (usize, usize);

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
pub enum Next {
    Empty,
    Obstruction,
    Escape,
}

fn part_1(input: &Input) -> usize {
    return walk_map(input).expect("");
}

fn part_2(input: &Input) -> i64 {
    let mut output = 0;

    for x in 0..input[0].len() {
        for y in 0..input.len() {
            if will_loop(&(x, y), input) {
                output += 1;
            }
        }
    }

    return output;
}

fn walk_map(input: &Input) -> Option<usize> {
    let mut position: Position = starting_position(input);
    let mut direction: Direction = starting_direction(position, input);

    let mut positions_visited = HashSet::new();
    let mut states_visited = HashSet::new();

    loop {
        if states_visited.contains(&(position, direction.clone())) {
            return None;
        }

        positions_visited.insert(position);
        states_visited.insert((position, direction.clone()));

        match check_next(&position, &direction, input) {
            Next::Empty => {
                position = next_position(&position, &direction);
            }
            Next::Obstruction => {
                direction = next_direction(&direction);
            }
            Next::Escape => {
                break;
            }
        }
    }

    return Some(positions_visited.len());
}

fn starting_direction(position: Position, input: &Input) -> Direction {
    match input[position.1][position.0] {
        '^' => Direction::Up,
        '>' => Direction::Right,
        '<' => Direction::Left,
        'v' => Direction::Down,
        _ => Direction::Up,
    }
}

fn will_loop(position: &Position, input: &Vec<Vec<char>>) -> bool {
    let x = position.0;
    let y = position.1;

    if input[y][x] == '.' {
        let mut with_obstacle = input.clone();
        with_obstacle[y][x] = '#';

        if walk_map(&with_obstacle).is_none() {
            return true;
        }
    }
    return false;
}

fn starting_position(input: &Input) -> Position {
    let mut position = (0, 0);
    let starting_chars = Vec::from(['v', '<', '>', '^']);

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if starting_chars.contains(&input[y][x]) {
                position = (x, y);
            }
        }
    }

    return position;
}

fn next_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Right => Direction::Down,
        Direction::Left => Direction::Up,
    }
}

fn next_position(position: &Position, direction: &Direction) -> Position {
    match direction {
        Direction::Up => (position.0, position.1 - 1),
        Direction::Down => (position.0, position.1 + 1),
        Direction::Right => (position.0 + 1, position.1),
        Direction::Left => (position.0 - 1, position.1),
    }
}

fn check_next(position: &Position, direction: &Direction, current_map: &Vec<Vec<char>>) -> Next {
    let next_position = next_position(position, direction);
    let y_len = current_map.len();
    let x_len = current_map[0].len();

    if {
        match direction {
            Direction::Up => position.1 == 0,
            Direction::Down => position.1 == x_len - 1,
            Direction::Right => position.0 == y_len - 1,
            Direction::Left => position.0 == 0,
        }
    } {
        return Next::Escape;
    } else {
        match current_map[next_position.1][next_position.0] {
            '#' => Next::Obstruction,
            _ => Next::Empty,
        }
    }
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
