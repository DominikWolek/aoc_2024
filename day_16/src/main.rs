use std::{
    collections::{HashMap, HashSet},
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

type Input = Vec<Vec<char>>;
type Position = (usize, usize);
const TURN_COST: i64 = 1000;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> i64 {
    let start_pos = get_pos(input, 'S');
    let end_pos = get_pos(input, 'E');

    let mut new_input = input.clone();
    new_input[start_pos.1][start_pos.0] = '.';

    let (dist, _prev) = dijkstra(start_pos, input);
    let (_pos, distance) = dist
        .iter()
        .filter(|((pos, _dir), _val)| *pos == end_pos)
        .min_by(|(_x, distance_x), (_y, distance_y)| (*distance_x).cmp(distance_y))
        .unwrap();
    return *distance;
}

fn part_2(input: &Input) -> usize {
    let start_pos = get_pos(input, 'S');
    let end_pos = get_pos(input, 'E');

    let mut new_input = input.clone();
    new_input[start_pos.1][start_pos.0] = '.';

    let (dist, prev) = dijkstra(start_pos, input);

    let (v, _distance) = dist
        .iter()
        .filter(|((pos, _dir), _val)| *pos == end_pos)
        .min_by(|(_x, distance_x), (_y, distance_y)| (*distance_x).cmp(distance_y))
        .unwrap();

    let path = get_path(prev, v.clone());

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if path.contains(&(x, y)) {
                print!("O");
            } else {
                print!("{}", input[y][x]);
            }
        }
        println!();
    }

    return path.len();
}

fn get_path(prev: PrevMap, end_v: (Position, Dir)) -> HashSet<Position> {
    let mut visited_positions: HashSet<Position> = HashSet::new();

    let mut stack = Vec::new();
    stack.push(end_v);
    // visited_positions.insert(end_v.clone().0);

    while !stack.is_empty() {
        let curr_v = stack.pop().unwrap();

        let prevs = prev.get(&curr_v).unwrap();

        visited_positions.insert(curr_v.0);
        for (prev_pos, prev_dir) in prevs {
            visited_positions.insert(*prev_pos);
            stack.push((*prev_pos, prev_dir.clone()));
        }
    }

    return visited_positions;
}

type DistMap = HashMap<(Position, Dir), i64>;

type PrevMap = HashMap<(Position, Dir), HashSet<(Position, Dir)>>;

const INFINITY: i64 = TURN_COST * TURN_COST * TURN_COST;

fn dijkstra(start_pos: Position, input: &Input) -> (DistMap, PrevMap) {
    let start_dir = Dir::Right;
    let mut dist: DistMap = HashMap::new();
    let mut prev: PrevMap = HashMap::new();

    let mut q: HashSet<(Position, Dir)> = HashSet::new();

    //setup
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let pos = (x, y);
            if val(pos, input) == '#' {
                continue;
            } else {
                for dir in vec![Dir::Up, Dir::Down, Dir::Right, Dir::Left] {
                    let key = (pos, dir.clone());

                    dist.insert(key.clone(), INFINITY);
                    prev.insert(key.clone(), HashSet::new());
                    q.insert(key);
                }
            }
        }
    }

    dist.insert((start_pos, start_dir), 0);

    while !HashSet::is_empty(&q) {
        //bad
        let hash_set = q.clone();
        let (v, val) = hash_set
            .iter()
            .map(|key| {
                let a = &dist.get(key).unwrap().clone();
                (key, *a)
            })
            .min_by(|(_x, distance_x), (_y, distance_y)| (*distance_x).cmp(distance_y))
            .unwrap();

        q.remove(&v);

        println!("Left: {:?}", q.len());

        for (n_pos, n_dir, n_val) in neighbours(v.0, &v.1, input) {
            let alt = n_val + val;
            let n_v = (n_pos, n_dir);
            let curr_val = dist.get(&n_v).unwrap().clone();

            if alt < curr_val {
                HashMap::insert(&mut dist, n_v.clone(), alt);
                let new_set = HashSet::from([v.clone()]);
                prev.insert(n_v, new_set);
            } else if alt == curr_val {
                let mut curr_set = prev.get(&n_v).unwrap().clone();
                curr_set.insert(v.clone());
                prev.insert(n_v, curr_set);
            }
        }
    }

    return (dist, prev);
}

fn get_pos(input: &Input, val: char) -> Position {
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == val {
                return (x, y);
            }
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

fn val((x, y): Position, input: &Input) -> char {
    return input[y][x];
}

fn next_dir_clock(dir: &Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Right,
        Dir::Down => Dir::Left,
        Dir::Right => Dir::Down,
        Dir::Left => Dir::Up,
    }
}

fn next_dir_counter(dir: &Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Left,
        Dir::Down => Dir::Right,
        Dir::Right => Dir::Up,
        Dir::Left => Dir::Down,
    }
}

fn neighbours(pos: Position, dir: &Dir, input: &Input) -> Vec<(Position, Dir, i64)> {
    let mut output = vec![
        (pos, next_dir_clock(dir), TURN_COST),
        (pos, next_dir_counter(dir), TURN_COST),
    ];
    let next_pos = next_pos(pos, dir);
    if val(next_pos, input) != '#' {
        output.insert(0, (next_pos, dir.clone(), 1));
    }

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
        output.push(line.chars().collect());
    }

    return output;
}
