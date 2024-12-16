use std::{
    collections::{hash_map, HashMap},
    env::{self},
    fs::File,
    i32,
    io::{BufRead, BufReader},
};

type Input = Vec<Robot>;

type Position = (i32, i32);
type Vector = (i32, i32);

#[derive(Debug, PartialEq, Clone)]
struct Robot {
    position: Position,
    velocity: Vector,
}

// const SPACE_WIDTH: i32 = 11;
// const SPACE_HEIGHT: i32 = 7;
const SPACE_WIDTH: i32 = 101;
const SPACE_HEIGHT: i32 = 103;

fn main() {
    let mut input = get_input();
    println!("Part 1: {}", part_1(&mut input));
    //     println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &mut Input) -> i32 {
    for robot in &mut *input {
        robot.travel_n_times(100);
    }
    return safety_factor(input);
}

fn print_map(robots: &Input) {
    let mut hash_map = HashMap::new();

    for robot in robots {
        let cur_val = hash_map.get(&robot.position).unwrap_or(&0);
        hash_map.insert(robot.position, cur_val + 1);
    }

    for y in 0..SPACE_HEIGHT {
        for x in 0..SPACE_WIDTH {
            match hash_map.get(&(x, y)) {
                Some(num) => print!("{num}"),
                None => print!("."),
            }
        }
        println!();
    }
}

fn safety_factor(robots: &Input) -> i32 {
    let mut q_1 = 0;
    let mut q_2 = 0;
    let mut q_3 = 0;
    let mut q_4 = 0;

    let x_middle = SPACE_WIDTH / 2;
    let y_middle = SPACE_HEIGHT / 2;

    for robot in robots {
        let (x, y) = robot.position;
        if x < x_middle && y < y_middle {
            q_1 += 1;
        } else if x > x_middle && y < y_middle {
            q_2 += 1;
        } else if x < x_middle && y > y_middle {
            q_3 += 1;
        } else if x > x_middle && y > y_middle {
            q_4 += 1;
        }
    }

    dbg!(q_1);
    dbg!(q_2);
    dbg!(q_3);
    dbg!(q_4);

    return q_1 * q_2 * q_3 * q_4;
}

impl Robot {
    fn travel(&mut self) {
        let (x, y) = self.position;
        let (v_x, v_y) = self.velocity;

        self.position = (
            teleport(x + v_x, SPACE_WIDTH),
            teleport(y + v_y, SPACE_HEIGHT),
        );
    }

    fn travel_n_times(&mut self, n: i32) {
        for _ in 0..n {
            self.travel();
        }
    }
}

fn teleport(val: i32, maximum: i32) -> i32 {
    (val + maximum) % maximum
}

fn get_input() -> Input {
    let args: Vec<String> = env::args().collect();
    let input_path = format!("src/{}", &args[1]);
    let file = File::open(input_path).expect("Failed to open file");
    let lines = BufReader::new(file).lines();

    let mut output = Vec::new();

    for line_res in lines {
        let line = line_res.expect("");
        let (pos_str, vec_str) = line.split_once(" ").unwrap();

        let robot = Robot {
            position: parse_pair(pos_str),
            velocity: parse_pair(vec_str),
        };
        output.push(robot);
    }

    return output;
}

fn parse_pair(pos_str: &str) -> (i32, i32) {
    let (left, right) = pos_str.split_at(2).1.split_once(",").unwrap();
    (
        left.parse::<i32>().expect(""),
        right.parse::<i32>().expect(""),
    )
}
