use std::{
    collections::HashMap,
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

use num_bigint::BigUint;

#[derive(Debug, Clone, PartialEq)]
enum Op {
    AND,
    OR,
    XOR,
}

type Register = String;
type Registers = HashMap<Register, bool>;

// #[derive(Debug, Clone, PartialEq)]
type Operation = (Register, Register, Op, Register);
type Operations = Vec<Operation>;
type Input = (Registers, Operations);

fn main() {
    let (registers, operations) = get_input();
    println!("Part 1: {}", part_1(&registers, &operations));
    println!("Part 2: {}", part_2(&registers, &operations));
}

fn part_1(registers: &Registers, operations: &Operations) -> BigUint {
    let mut registers = registers.clone();
    let operations = toposort(&registers, &operations);

    for operation in operations {
        do_operation(&operation, &mut registers);
    }

    num_from_zregs(&registers)
}

fn part_2(registers: &Registers, operations: &Operations) -> usize {
    0
}

fn toposort(registers: &Registers, operations: &Operations) -> Operations {
    let mut done = Vec::new();

    for (reg, _val) in registers {
        done.push(reg.clone());
    }

    let mut to_do = HashMap::new();

    for (x, y, _, result) in operations {
        to_do.insert(result.clone(), vec![x.clone(), y.clone()]);
    }

    while !to_do.is_empty() {
        let to_pop = find_empty(&to_do, &done);
        to_do.remove(&to_pop);
        done.push(to_pop);
    }

    let mut sorted = Vec::new();
    for result in done {
        if let Some(index) = operations.iter().position(|(_, _, _, res)| result == *res) {
            sorted.push(operations[index].clone());
        }
    }

    return sorted;
}

fn find_empty(to_do: &HashMap<String, Vec<String>>, done: &Vec<String>) -> String {
    let to_pop = to_do
        .iter()
        .find(|(_, args)| args.iter().all(|x| done.contains(x)))
        .unwrap()
        .0;

    to_pop.clone()
}

fn num_from_zregs(registers: &Registers) -> BigUint {
    let mut z_regs = registers
        .keys()
        .filter(|x| x.chars().nth(0).unwrap() == 'z')
        .collect::<Vec<_>>();

    z_regs.sort();

    let bytes = z_regs
        .iter()
        .rev()
        .map(|reg| registers.get(*reg).unwrap())
        .map(|x| (*x as u8).to_string().chars().collect::<Vec<_>>())
        .flatten()
        .collect::<String>();

    BigUint::parse_bytes(bytes.as_bytes(), 2).unwrap()
}

fn get_input() -> Input {
    let args: Vec<String> = env::args().collect();
    let input_path = format!("src/{}", &args[1]);
    let file = File::open(input_path).expect("Failed to open file");

    let lines = BufReader::new(file)
        .lines()
        .map(|line| line.expect(""))
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    let registers = lines
        .iter()
        .filter(|line| line.chars().nth(3).unwrap() == ':')
        .map(|register| {
            let (left, right) = register.split_once(": ").unwrap();

            (left.to_string(), parse_bool(right))
        })
        .collect();

    let operations = lines
        .iter()
        .filter(|line| line.chars().nth(3).unwrap() != ':')
        .map(|operation| {
            let (left, result) = operation.split_once(" -> ").unwrap();
            let (x, right) = left.split_once(" ").unwrap();
            let (op_str, y) = right.split_once(" ").unwrap();

            (
                x.to_string(),
                y.to_string(),
                parse_op(op_str),
                result.to_string(),
            )
        })
        .collect();

    return (registers, operations);
}

fn parse_bool(right: &str) -> bool {
    match right {
        "1" => true,
        "0" => false,
        _ => panic!(),
    }
}

fn do_operation(operation: &Operation, registers: &mut Registers) {
    let (x_str, y_str, op, result) = operation;

    let x = registers.get(x_str).unwrap();
    let y = registers.get(y_str).unwrap();

    registers.insert(result.to_string(), do_op(*x, *y, op.clone()));
}

fn do_op(x: bool, y: bool, op: Op) -> bool {
    match op {
        Op::AND => x && y,
        Op::OR => x || y,
        Op::XOR => !(x == y),
    }
}

fn parse_op(op: &str) -> Op {
    match op {
        "OR" => Op::OR,
        "AND" => Op::AND,
        "XOR" => Op::XOR,
        _ => panic!(),
    }
}
