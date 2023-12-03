use std::collections::HashMap;

#[allow(dead_code)]
pub fn part1_sample_input() -> Vec<String> {
    let sample = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    crate::utils::get_lines_from_string(sample.to_string())
}

#[allow(dead_code)]
pub fn part2_sample_input() -> Vec<String> {
    part1_sample_input()
}

#[allow(dead_code)]
pub fn part1(input: Vec<String>) {
    let color_map = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut res = 0;
    for (idx, line) in input.iter().enumerate() {
        let vec1: Vec<&str> = line.split(": ").collect();
        let draws = vec1[1];
        let mut check = true;
        for draw in draws.split("; ") {
            for each_color in draw.split(", ") {
                let vec2: Vec<&str> = each_color.split(' ').collect();
                let count = vec2[0].parse::<i32>().unwrap();
                let color = vec2[1];
                if *color_map.get(color).unwrap() < count {
                    check = false;
                }
            }
        }
        if check == true {
            res += idx + 1;
        }
    }
    println!("{}", res);
}

#[allow(dead_code)]
pub fn part2(input: Vec<String>) {
    let mut res = 0;
    for line in input.iter() {
        let mut hm = HashMap::new();
        let vec1: Vec<&str> = line.split(": ").collect();
        let draws = vec1[1];
        for draw in draws.split("; ") {
            for each_color in draw.split(", ") {
                let vec2: Vec<&str> = each_color.split(' ').collect();
                let count = vec2[0].parse::<i32>().unwrap();
                let color = vec2[1];
                let entry = hm.entry(color).or_insert(count);
                *entry = std::cmp::max(*entry, count);
            }
        }
        res += hm.get("red").unwrap() * hm.get("green").unwrap() * hm.get("blue").unwrap();
    }
    println!("{}", res);
}