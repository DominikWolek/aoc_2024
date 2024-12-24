use std::{
    collections::{HashMap, HashSet},
    env::{self},
    fs::File,
    io::{BufRead, BufReader},
};

type Computer = String;
type Connection = (Computer, Computer);
type Input = Vec<Connection>;

const HISTORIAN_INITIAL: char = 't';

fn main() {
    let input = get_input();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> usize {
    let connections = connections(input);
    let threes = threes(&connections);

    threes
        .iter()
        .filter(|(a, b, c)| initial(a) || initial(b) || initial(c))
        .count()
}

fn part_2(input: &Input) -> String {
    let connections = connections(input);

    let large_set = connections
        .keys()
        .map(|init| largest(init, &connections))
        .max_by(|left, right| usize::cmp(&left.len(), &right.len()))
        .unwrap();

    let mut password = large_set.iter().map(|x| x.clone()).collect::<Vec<String>>();
    password.sort();
    // .join(",");

    return password.join(",");
}

fn largest(init: &String, connections: &HashMap<String, HashSet<String>>) -> HashSet<Computer> {
    let mut set = HashSet::new();
    set.insert(init.clone());

    let empty = HashSet::new();
    let mut queue = connections
        .get(init)
        .unwrap_or(&empty)
        .iter()
        .collect::<Vec<_>>();

    while !queue.is_empty() {
        let curr = queue.pop().unwrap();
        let connected = connections.get(curr).unwrap();

        if set.iter().all(|computer| connected.contains(computer)) {
            set.insert(curr.clone());

            for connected in connected {
                queue.push(connected);
            }
        }
    }

    return set;
}

fn initial(string: &String) -> bool {
    string.chars().nth(0) == Some(HISTORIAN_INITIAL)
}

type Three = (Computer, Computer, Computer);

fn threes(connections: &HashMap<Computer, HashSet<Computer>>) -> HashSet<Three> {
    let mut set = HashSet::new();

    for (a, connected) in connections {
        for b in connected {
            for c in connections.get(b).unwrap_or(&HashSet::new()) {
                if connected.contains(c) {
                    let mut vector = vec![a, b, c];
                    vector.sort();
                    set.insert((vector[0].clone(), vector[1].clone(), vector[2].clone()));
                }
            }
        }
    }

    return set;
}

fn connections(input: &Input) -> HashMap<Computer, HashSet<Computer>> {
    let mut mapa = HashMap::new();
    for (left, right) in input {
        add_connection(&mut mapa, left.clone(), right.clone());
        add_connection(&mut mapa, right.clone(), left.clone());
    }

    return mapa;
}

fn add_connection(mapa: &mut HashMap<Computer, HashSet<Computer>>, left: String, right: String) {
    let mut curr = mapa.get(&left).unwrap_or(&HashSet::new()).clone();
    curr.insert(right);
    mapa.insert(left, curr);
}

fn get_input() -> Input {
    let args: Vec<String> = env::args().collect();
    let input_path = format!("src/{}", &args[1]);
    let file = File::open(input_path).expect("Failed to open file");
    let lines = BufReader::new(file).lines();

    let mut output = Vec::new();

    for line_res in lines {
        let line = line_res.expect("");
        let (left, right) = line.split_at(2);

        output.push((String::from(left), String::from(&right[1..])));
    }

    return output;
}
