use std::collections::HashSet;

#[allow(dead_code)]
pub fn part1_sample_input() -> Vec<String> {
    let sample = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    crate::utils::get_lines_from_string(sample.to_string())
}

#[allow(dead_code)]
pub fn part2_sample_input() -> Vec<String> {
    part1_sample_input()
}

fn get_matchings(input: Vec<String>) -> Vec<i32> {
    let mut res = vec![];
    for card in input {
        let numbers = card.trim().split(": ").collect::<Vec<&str>>()[1].to_string();
        let split_numbers = numbers.trim().split("|").collect::<Vec<&str>>();
        let winning = 
            split_numbers[0]
                .trim()
                .split(" ")
                .filter(|&s| s != "")
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect::<HashSet<i32>>();
        let current = 
            split_numbers[1]
                .trim()
                .split(" ")
                .filter(|&s| s != "")
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
        let mut count = 0;
        for number in current {
            if winning.contains(&number) {
                count += 1;
            }
        }
        res.push(count);
    }
    res
}

#[allow(dead_code)]
pub fn part1(input: Vec<String>) {
    let matchings = get_matchings(input);
    println!("{}", matchings.iter().fold(0, |acc, &e| acc + if e == 0 { 0 } else { 1 << (e - 1) }));
} 

#[allow(dead_code)]
pub fn part2(input: Vec<String>) {
    let matchings = get_matchings(input);
    let mut res = vec![0; matchings.len() + 1];
    for i in (0..matchings.len()).rev() {
        res[i] = 2 * res[i + 1] - res[i + matchings[i] as usize + 1] + 1;
    }
    println!("{}", res[0]);
}