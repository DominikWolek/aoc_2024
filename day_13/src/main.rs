use std::{
    cmp::min,
    collections::HashMap,
    env::{self},
    fs::File,
    io::{BufRead, BufReader, Read},
};

type Val = usize;
// #[derive(Debug, Partial_Eq)]
type Position = (Val, Val);
type Vector = (Val, Val);
type Input = Vec<Machine>;

#[derive(Debug)]
struct Machine {
    button_a: Vector,
    button_b: Vector,
    prize: Position,
}

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

const A_PRIZE: usize = 3;
const B_PRIZE: usize = 1;
const MAX_PART_1: usize = 100;
fn part_1(input: &Input) -> usize {
    input
        .iter()
        .map(|machine| {
            let (a_cnt, b_cnt) = calc_cheapest(machine, MAX_PART_1);
            return a_cnt * A_PRIZE + b_cnt * B_PRIZE;
        })
        .sum()
}

type RecState = Vector;

type MemMap = HashMap<RecState, Option<Vector>>;

fn calc_cheapest(machine: &Machine, max_iter: usize) -> Vector {
    let mut mem_map: MemMap = HashMap::new();

    let start_state = (0_usize, 0_usize);

    match do_calc_cheapest(start_state, max_iter, machine, &mut mem_map) {
        Some(x) => x,
        None => (0, 0),
    }
}

fn do_calc_cheapest(
    push_vec: RecState,
    max_iter: usize,
    machine: &Machine,
    mem_map: &mut MemMap,
) -> Option<Vector> {
    match mem_map.get(&push_vec) {
        None => {
            if position(push_vec, machine) == machine.prize {
                return Some(push_vec);
            } else if push_vec.0 == max_iter || push_vec.1 == max_iter {
                return None;
            } else {
                let pushed_a = (push_vec.0 + 1, push_vec.1);
                let pushed_b = (push_vec.0, push_vec.1 + 1);
                let a_state = pushed_a;
                let a_vec = do_calc_cheapest(a_state, max_iter, machine, mem_map);
                let b_state = pushed_b;
                let b_vec = do_calc_cheapest(b_state, max_iter, machine, mem_map);

                mem_map.insert(a_state, a_vec);
                mem_map.insert(b_state, b_vec);

                if a_vec.is_none() {
                    return b_vec;
                } else if b_vec.is_none() {
                    return a_vec;
                } else {
                    return vec![a_vec, b_vec].iter().map(|x| x.expect("")).min();
                }
            }
        }
        Some(val) => {
            return *val;
        }
    }
}

fn position((a_cnt, b_cnt): Vector, machine: &Machine) -> Position {
    let a_vec = (machine.button_a.0 * a_cnt, machine.button_a.1 * a_cnt);
    let b_vec = (machine.button_b.0 * b_cnt, machine.button_b.1 * b_cnt);

    return (a_vec.0 + b_vec.0, a_vec.1 + b_vec.1);
}

fn part_2(input: &Input) -> usize {
    let mut output: usize = 0;

    // for i in input {

    // }

    return output;
}

fn get_input() -> Input {
    let args: Vec<String> = env::args().collect();
    let input_path = format!("src/{}", &args[1]);
    let mut file = File::open(input_path).expect("Failed to open file");

    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    buf.split("\n\n")
        .map(|x| parse_machine(x))
        .collect::<Vec<Machine>>()
}

fn parse_machine(x: &str) -> Machine {
    let lines = x.split("\n").collect::<Vec<&str>>();
    Machine {
        button_a: parse_line(lines[0], BUTTON_PREFIX_LEN),
        button_b: parse_line(lines[1], BUTTON_PREFIX_LEN),
        prize: parse_line(lines[2], PRIZE_PREFIX_LEN),
    }
}

//prefix = "Button _: "
const BUTTON_PREFIX_LEN: usize = 10;
//prefix = "Prize: "
const PRIZE_PREFIX_LEN: usize = 7;
//short = "__"
const SHORT_PREFIX_LEN: usize = 2;

fn parse_line(line: &str, line_prefix_len: usize) -> Position {
    let num_vec = line
        .split_at(line_prefix_len)
        .1
        .split(", ")
        .map(|x| {
            x.split_at(SHORT_PREFIX_LEN)
                .1
                .parse::<Val>()
                .expect("parse error")
        })
        .collect::<Vec<Val>>();
    (num_vec[0], num_vec[1])
}
