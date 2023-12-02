#[allow(dead_code)]
pub fn part1_sample_input() -> Vec<String> {
    let sample = "1abc2\n\
    pqr3stu8vwx\n\
    a1b2c3d4e5f\n\
    treb7uchet";
    crate::utils::get_lines_from_string(sample.to_string())
}

#[allow(dead_code)]
pub fn part2_sample_input() -> Vec<String> {
    let sample = "two1nine\n\
    eightwothree\n\
    abcone2threexyz\n\
    xtwone3four\n\
    4nineeightseven2\n\
    zoneight234\n\
    7pqrstsixteen";
    crate::utils::get_lines_from_string(sample.to_string())
}

#[allow(dead_code)]
pub fn part1(input: Vec<String>) {
    let digits = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut res = 0;
    for line in input {
        let mut first = -1;
        let mut second = 0;
        for c in line.as_bytes() {
            for (idx, d) in digits.iter().enumerate() {
                if *c == *d as u8 {
                    if first == -1 {
                        first = idx as i32;
                    }
                    second = idx as i32;
                }
            }
        }
        res += first * 10 + second;
    }
    println!("{}", res)
}

#[allow(dead_code)]
pub fn part2(input: Vec<String>) {
    let digits = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let numbers = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut res = 0;
    for line in input {
        let mut first = -1;
        let mut second = 0;
        for (i, c) in line.as_bytes().iter().enumerate() {
            for (idx, d) in digits.iter().enumerate() {
                if *c == *d as u8 {
                    if first == -1 {
                        first = idx as i32;
                    }
                    second = idx as i32;
                }
            }
            for (idx, &n) in numbers.iter().enumerate() {
                if (i + n.len()) <= line.len() && n.eq(&line[i..(i + n.len())]) {
                    if first == -1 {
                        first = idx as i32;
                    }
                    second = idx as i32;
                }
            }
        }
        res += first * 10 + second;
    }
    println!("{}", res);
}