use std::{
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

type Base = isize;
type Input = Vec<Base>;
type DiscMap = Vec<Base>;
type DiscFiles = Vec<(usize, Base)>;

const EMPTY: Base = -1;

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> usize {
    let mut disc_map: DiscMap = get_map(input);

    let len = disc_map.len();
    let mut free = 0;

    let mut take = len - 1;

    loop {
        for i in 0..len {
            if disc_map[i] == EMPTY {
                free = i;
                break;
            }
        }

        let mut reversed: Vec<usize> = (0..len).collect();
        reversed.reverse();

        for i in reversed {
            if disc_map[i] != EMPTY {
                take = i;
                break;
            }
        }

        if free < take {
            disc_map[free] = disc_map[take];
            disc_map[take] = EMPTY;
        } else {
            break;
        }
    }

    return calc_checksum(&disc_map);
}

fn part_2(input: &Input) -> usize {
    let mut disc_files: DiscFiles = get_disc_files(input);

    let mut take_val: isize = disc_files.last().expect("").1;
    let mut take_in = disc_files.len() - 1 as usize;

    while take_val >= 0 {
        loop {
            if disc_files[take_in].1 == take_val {
                break;
            } else {
                take_in -= 1;
            }
        }

        let take_size = disc_files[take_in].0;
        if take_val != EMPTY {
            for free in 0..take_in {
                let free_val = disc_files[free].1;
                let free_size = disc_files[free].0;
                if free_val == EMPTY && free_size >= take_size {
                    if free_size == take_size {
                        disc_files[free].1 = take_val;
                        disc_files[take_in].1 = EMPTY;
                    } else {
                        disc_files.remove(free);
                        disc_files.insert(free, (take_size, take_val));
                        disc_files.insert(free + 1, (free_size - take_size, EMPTY));
                        disc_files[take_in + 1].1 = EMPTY;

                        take_in += 1;
                    }
                    break;
                }
            }
        }
        take_val -= 1;
    }

    return calc_files_checksum(&disc_files);
}

fn calc_files_checksum(disc_files: &[(usize, isize)]) -> usize {
    let x = disc_files
        .iter()
        .map(|(size, val)| {
            let mut x = Vec::new();
            for _ in 0..*size {
                if *val == EMPTY {
                    x.push(0);
                } else {
                    x.push(*val);
                }
            }
            return x;
        })
        .flatten()
        .collect();

    return calc_checksum(&x);
}

fn get_disc_files(input: &Input) -> DiscFiles {
    let mut output = DiscFiles::new();
    let mut ids = true;
    let mut current_id: Base = 0;

    for curr_len in input {
        if ids {
            output.push((*curr_len as usize, current_id));
            current_id += 1;
        } else {
            output.push((*curr_len as usize, EMPTY));
        }
        ids = !ids;
    }
    return output;
}

fn calc_checksum(disc_map: &DiscMap) -> usize {
    let mut curr_id = 0;
    let mut output: usize = 0;

    for i in disc_map {
        if *i != EMPTY {
            output += *i as usize * curr_id as usize;
            curr_id += 1;
        } else {
            break;
        }
    }

    return output;
}

fn get_map(input: &[Base]) -> DiscMap {
    let mut output = DiscMap::new();
    let mut ids = true;
    let mut current_id = 0;

    for i in input {
        if ids {
            for _ in 0..*i {
                output.push(current_id);
            }
            current_id += 1;
        } else {
            for _ in 0..*i {
                output.push(EMPTY);
            }
        }
        ids = !ids;
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
        let numbers = line
            .chars()
            .map(|x| format!("{}", x).parse::<Base>().expect(""))
            .collect::<DiscMap>();

        output.push(numbers);
    }

    let output = output[0].clone();

    return output;
}
