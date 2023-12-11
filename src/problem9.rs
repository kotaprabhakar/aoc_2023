#[allow(dead_code)]
pub fn part1_sample_input() -> Vec<String> {
    let sample =
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    crate::utils::get_lines_from_string(sample.to_string())
}

#[allow(dead_code)]
pub fn part2_sample_input() -> Vec<String> {
    part1_sample_input()
}

fn get_pascal_triangle() -> Vec<Vec<i64>> {
    const SIZE: i64 = 67;
    let mut res = vec![vec![1]];
    for i in 1.. SIZE {
        let mut vec = vec![1i64];
        for j in 1..i {
            vec.push(res.last().unwrap()[j as usize] + res.last().unwrap()[j as usize - 1]);
        }
        vec.push(1i64);
        res.push(vec);
    }
    res
}

#[allow(dead_code)]
pub fn part1(input: Vec<String>) {
    let mut _pascal_triangle: Vec<Vec<i64>> = get_pascal_triangle();
    let mut res = 0;
    for line in input {
        let mut numbers = vec![line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>()];
        while !numbers.last().unwrap().iter().all(|&x| x == 0) {
            let mut row = vec![];
            for (curr, next) in numbers.last().unwrap().iter().zip(numbers.last().unwrap().iter().skip(1)) {
                row.push(next - curr);
            }
            numbers.push(row);
        }
        let mut temp_res = 0;
        for row in numbers.iter().rev().skip(1) {
            temp_res = row.last().unwrap() + temp_res;
        }
        res += temp_res;
    }
    println!("{}", res);
}

#[allow(dead_code)]
pub fn part2(input: Vec<String>) {
    let mut _pascal_triangle: Vec<Vec<i64>> = get_pascal_triangle();
    let mut res = 0;
    for line in input {
        let mut temp_numbers = line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        temp_numbers.reverse();
        let mut numbers = vec![temp_numbers];
        numbers.reverse();
        while !numbers.last().unwrap().iter().all(|&x| x == 0) {
            let mut row = vec![];
            for (curr, next) in numbers.last().unwrap().iter().zip(numbers.last().unwrap().iter().skip(1)) {
                row.push(next - curr);
            }
            numbers.push(row);
        }
        // println!("{:?}", numbers);
        // println!();
        let mut temp_res = 0;
        for row in numbers.iter().rev().skip(1) {
            temp_res = row.last().unwrap() + temp_res;
        }
        res += temp_res;
    }
    println!("{}", res);
}