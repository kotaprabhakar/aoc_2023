use std::collections::VecDeque;

#[allow(dead_code)]
pub fn part1_sample_input() -> Vec<String> {
    let sample =
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    crate::utils::get_lines_from_string(sample.to_string())
}

#[allow(dead_code)]
pub fn part2_sample_input() -> Vec<String> {
    part1_sample_input()
}

fn hash(label: &str) -> u64 {
    let mut curr_val = 0u64;
    for c in label.chars() {
        curr_val += c as u64;
        curr_val *= 17;
        curr_val %= 256;
    }
    curr_val
}

#[allow(dead_code)]
pub fn part1(input: Vec<String>) {
    let mut res = 0u64;
    for str in input[0].split(',').map(|x| x.to_string()).collect::<Vec<String>>() {
        res += hash(str.as_str());
    }
    println!("{:?}", res);
}

#[allow(dead_coded)]
pub fn part2(input: Vec<String>) {
    let mut boxes: Vec<Vec<Vec<String>>> = vec![vec![]; 256];
    for str in input[0].split(',').map(|x| x.to_string()).collect::<Vec<String>>(){
        let label =
            if str.contains("-") {
                str.split("-").map(|s| s.to_string()).collect::<Vec<String>>()
            } else {
                str.split("=").map(|s| s.to_string()).collect::<Vec<String>>()
            };
        let box_idx = hash(label[0].as_str()) as usize;
        match boxes[box_idx].iter().enumerate().find(|(idx, s)| s[0] == label[0]) {
            Some((idx, _)) => {
                if str.contains("-") {
                    boxes[box_idx].remove(idx);
                } else {
                    boxes[box_idx][idx] = label.clone();
                }
            },
            None => {
                if str.contains("=") {
                    boxes[box_idx].push(label.clone());
                }
            }
        };
    }
    let mut res = 0;
    for (box_idx, curr_box) in boxes.iter().enumerate() {
        for (slot_idx, lens) in curr_box.iter().enumerate() {
            res += (box_idx + 1) * (slot_idx + 1) * (lens[1].parse::<usize>().unwrap());
        }
    }
    println!("{:?}", res);
}