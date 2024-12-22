use std::{
    collections::HashMap,
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
};

type Input = Vec<usize>;
type Price = u8;
type Change = i8;
type ChangeSeq = (Change, Change, Change, Change);

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

const GENERATIONS_CNT: usize = 2000;
const PRUNE_BASE: usize = 16_777_216;
const CHANGE_RANGE: std::ops::RangeInclusive<i8> = -9..=9;

fn part_1(input: &Input) -> usize {
    input
        .iter()
        .map(|initial| calc_secret(*initial, GENERATIONS_CNT))
        .sum()
}

fn part_2(input: &Input) -> usize {
    let changes_maps = input
        .iter()
        .map(|initial| changes_map(*initial))
        .collect::<Vec<_>>();

    let mut max = 0;
    for ch_1 in CHANGE_RANGE {
        for ch_2 in CHANGE_RANGE {
            for ch_3 in CHANGE_RANGE {
                for ch_4 in CHANGE_RANGE {
                    let change_seq: ChangeSeq = (ch_1, ch_2, ch_3, ch_4);

                    let result = bananas(&changes_maps, change_seq);

                    if result > max {
                        max = result;
                    }
                }
            }
        }
    }

    return max;
}

fn bananas(changes_maps: &Vec<HashMap<ChangeSeq, Price>>, change_seq: ChangeSeq) -> usize {
    let seq_sum = change_seq.0 + change_seq.1 + change_seq.2 + change_seq.3;
    if CHANGE_RANGE.contains(&seq_sum) {
        changes_maps
            .iter()
            .map(|mapa| mapa.get(&change_seq).unwrap_or(&0))
            .map(|x| *x as usize)
            .sum()
    } else {
        0
    }
}

fn changes_map(initial: usize) -> HashMap<ChangeSeq, Price> {
    let prices = secrets(initial)
        .iter()
        .map(|secret| (secret % 10) as Price)
        .collect::<Vec<_>>();

    let changes = changes(&prices);
    zip(changes, &prices[4..])
        .map(|(x, y)| (x, *y))
        .rev()
        .collect()
}

fn changes(prices: &Vec<Price>) -> Vec<ChangeSeq> {
    let (mut ch_1, mut ch_2, mut ch_3, mut ch_4): ChangeSeq = (0, 0, 0, 0);

    let mut out = vec![];
    for i in 1..prices.len() {
        ch_1 = ch_2;
        ch_2 = ch_3;
        ch_3 = ch_4;
        ch_4 = prices[i] as Change - prices[i - 1] as Change;

        if i >= 4 {
            out.push((ch_1, ch_2, ch_3, ch_4));
        }
    }

    return out;
}

fn secrets(initial: usize) -> Vec<usize> {
    let mut out = vec![];

    let mut current = initial;
    for _ in 0..2000 {
        out.push(current);
        current = calc_secret(current, 1);
    }
    out
}

fn calc_secret(secret: usize, generations_cnt: usize) -> usize {
    if generations_cnt == 0 {
        return secret;
    } else {
        let mut current = ((secret << 6) ^ secret) % PRUNE_BASE;
        current = ((current >> 5) ^ current) % PRUNE_BASE;
        current = ((current * 2048) ^ current) % PRUNE_BASE;

        return calc_secret(current, generations_cnt - 1);
    }
}

fn get_input() -> Input {
    let args: Vec<String> = env::args().collect();
    let input_path = format!("src/{}", &args[1]);
    let file = File::open(input_path).expect("Failed to open file");

    BufReader::new(file)
        .lines()
        .map(|line| line.expect("").parse::<usize>().expect("parse error"))
        .collect()
}
