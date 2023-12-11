use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
pub fn part1_sample_input() -> Vec<String> {
    let sample =
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    crate::utils::get_lines_from_string(sample.to_string())
}

#[allow(dead_code)]
pub fn part2_sample_input() -> Vec<String> {
    let sample =
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    crate::utils::get_lines_from_string(sample.to_string())
}

fn parse_input(input: Vec<String>) -> (Vec<u8>, HashMap<String, (String, String)>) {
    let mut directions = vec![];
    let mut dictionary = HashMap::new();
    for raw_line in input {
        let line = raw_line.trim().to_string();
        if !line.is_empty() {
            if !line.contains("=") {
                directions = line.as_bytes().iter().map(|&x| x).collect::<Vec<u8>>();
            } else {
                let nexts = line[7..15].split(", ").map(|x| x.to_string()).collect::<Vec<String>>();
                dictionary.entry(line[0..3].to_string()).or_insert((nexts[0].to_string(), nexts[1].to_string()));
            }
        }
    }
    (directions, dictionary)
}

fn get_num_steps(directions: &Vec<u8>, dictionary: &HashMap<String, (String, String)>, start: &str, end: &str) -> i64 {
    let mut i = 0;
    let mut node = start;
    let mut res = 0;
    loop {
        res += 1;
        if node == end {
            break;
        }
        if directions[i as usize] == ('R' as u8) {
            node = dictionary[node].1.as_str();
        } else {
            node = dictionary[node].0.as_str();
        }
        i = (i + 1) % (directions.len() as i64);
    }
    res - 1
}

pub fn part1(input: Vec<String>) {
    let (directions, dictionary) = parse_input(input);

    println!("{}", get_num_steps(&directions, &dictionary, "AAA", "ZZZ"));
}

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        return std::cmp::max(a, b);
    }
    gcd(std::cmp::max(a, b) % std::cmp::min(a, b), std::cmp::min(a, b))
}

fn lcm(vec: &Vec<i64>) -> i64 {
    let mut res = 1;
    for &x in vec.iter() {
        res = x * res / gcd(res, x);
    }
    res
}

pub fn part2(input: Vec<String>) {
    let (directions, dictionary) = parse_input(input);
    let mut strings_a = vec![];
    let mut strings_z = HashSet::new();
    for (source, _) in dictionary.iter() {
        if source.chars().last().unwrap() == 'A' {
            strings_a.push(source);
        } else if source.chars().last().unwrap() == 'Z' {
            strings_z.insert(source);
        }
    }
    let mut res = vec![i64::MAX; strings_a.len()];
    for (i, &s) in strings_a.iter().enumerate() {
        let mut node = s;
        let mut idx = 0;
        let mut temp = 0;
        loop {
            temp += 1;
            if strings_z.contains(node) {
                res[i] = temp - 1;
                break;
            }
            if directions[idx as usize] == ('R' as u8) {
                node = &dictionary[node].1;
            } else {
                node = &dictionary[node].0;
            }
            idx = (idx + 1) % (directions.len() as i64);
        }
    }
    println!("{:?}", lcm(&res));
}