use std::{
    env::{self},
    fmt::Debug,
    fs::File,
    io::Read,
};

type Input = (Registers, Program);
type Program = Vec<u8>;
type Register = i64;
type Registers = Vec<Register>;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Ins {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

fn main() {
    let (registers, program) = get_input();
    println!("Part 1: {:?}", part_1(&registers, &program));
    // println!("Part 2: {:?}", part_2(&program));
}

fn part_1(registers: &Registers, program: &Program) -> String {
    let mut ptr = 0;
    let mut registers = registers.clone();

    let mut out = String::new();

    while ptr < program.len() {
        let ins = op_to_ins(program[ptr]);
        do_ins(ins, &mut ptr, &program, &mut registers, &mut out);
    }

    out
}

fn get_combo(val: u8, registers: &[i64]) -> i64 {
    match val {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => registers[A],
        5 => registers[B],
        6 => registers[C],
        _ => panic!(),
    }
}

fn do_ins(
    ins: Ins,
    ptr: &mut usize,
    program: &Program,
    registers: &mut Registers,
    out: &mut String,
) {
    let literal = program[*ptr + 1];
    let combo = get_combo(program[*ptr + 1], &registers);

    let mut jump = false;

    match ins {
        Ins::ADV => {
            registers[A] = dv(registers[A], combo);
        }
        Ins::BXL => {
            registers[B] = registers[B] ^ literal as Register;
        }
        Ins::BST => {
            registers[B] = combo % 8;
        }
        Ins::JNZ => {
            if registers[A] != 0 {
                let val = literal as usize;
                *ptr = val;
                jump = true;
            }
        }
        Ins::BXC => {
            registers[B] = registers[B] ^ registers[C];
        }
        Ins::OUT => {
            out.push_str(&(format!("{:?},", combo % 8)));
        }
        Ins::BDV => {
            registers[B] = dv(registers[A], combo);
        }
        Ins::CDV => {
            registers[C] = dv(registers[A], combo);
        }
    }

    if !jump {
        *ptr += 2;
    }
}

fn dv(reg_a: i64, combo: i64) -> i64 {
    let den = 2_i64.pow(combo as u32);
    let result = reg_a / den;
    result
}

fn op_to_ins(opcode: u8) -> Ins {
    match opcode {
        0 => Ins::ADV,
        1 => Ins::BXL,
        2 => Ins::BST,
        3 => Ins::JNZ,
        4 => Ins::BXC,
        5 => Ins::OUT,
        6 => Ins::BDV,
        7 => Ins::CDV,
        _ => panic!(),
    }
}

fn get_input() -> Input {
    let args: Vec<String> = env::args().collect();
    let input_path = format!("src/{}", &args[1]);
    let mut file = File::open(input_path).expect("Failed to open file");

    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    let (registers, program) = buf.split_once("\n\n").unwrap();

    let (_, program) = program.split_once(": ").unwrap();
    let program: Program = program.split(",").map(|num| num.parse().unwrap()).collect();

    let registers = registers
        .split("\n")
        .map(|line| {
            let (_, register) = line.split_once(": ").unwrap();
            register.parse().unwrap()
        })
        .collect();

    return (registers, program);
}
