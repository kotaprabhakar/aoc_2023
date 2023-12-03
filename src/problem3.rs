use std::collections::HashSet;

#[allow(dead_code)]
pub fn part1_sample_input() -> Vec<String> {
    let sample = "467..114..\n\
    ...*......\n\
    ..35..633.\n\
    ......#...\n\
    617*......\n\
    .....+.58.\n\
    ..592.....\n\
    ......755.\n\
    ...$.*....\n\
    .664.598..";
    crate::utils::get_lines_from_string(sample.to_string())
}

#[allow(dead_code)]
pub fn part2_sample_input() -> Vec<String> {
    part1_sample_input()
}

fn has_symbol_around(input: &Vec<String>, i: i32, f: i32, s: i32) -> bool {
    for k in (i - 1)..(i + 2) {
        for l in (f - 1)..(s + 2) {
            if k > 0 && (k as usize) < input.len() && l > 0 && (l as usize) <= input[0].len() 
                && !(input[k as usize].as_bytes()[l as usize] as char).is_ascii_digit()
                && (input[k as usize].as_bytes()[l as usize] as char) != '.' {
                return true;
            }
        }
    }
    return false;
}

#[allow(dead_code)]
pub fn part1(input: Vec<String>) {
    let mut res = 0;
    for (i, line) in input.iter().enumerate() {
        let mut num = 0;
        let mut f = -1;
        let mut s = -1;
        for (j, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                num = num * 10 + ((c as u32) - ('0' as u32));
                if f == -1 {
                    f = j as i32;
                }
                s = j as i32;
            } else {
                if has_symbol_around(&input, i as i32, f, s) {
                    res += line[f as usize..(s as usize + 1)].parse::<i32>().unwrap();
                }
                num = 0;
                f = -1;
                s = -1;
            }
        }
    }
    println!("{}", res);
}

fn get_number(arr: &Vec<Vec<u8>>, i: i32, j: i32) -> Option<(i32, i32)> {
    let num_rows = arr.len() as i32;
    let num_cols = arr[0].len() as i32;
    if i >= 0 && i < num_rows && j >= 0 && j <= num_cols && (arr[i as usize][j as usize] as char).is_ascii_digit() {
        let from_idx = 
            (0..(j + 1))
                .rev()
                .skip_while(|&idx| (arr[i as usize][idx as usize] as char).is_ascii_digit())
                .next()
                .unwrap_or(-1);
        let to_idx = 
            (j..num_cols)
                .skip_while(|&idx| (arr[i as usize][idx as usize] as char).is_ascii_digit())
                .next()
                .unwrap_or(num_cols);
        return Some((from_idx + 1, to_idx - 1));
    }
    None
}

fn get_numbers(arr: &Vec<Vec<u8>>, i: i32, j: i32) -> Option<[i32; 2]> {
    let mut set = HashSet::new();
    for k in -1..2 {
        for l in -1..2 {
            match get_number(&arr, i + k, j + l) {
                Some((c1, c2)) => { set.insert((i + k, c1, c2)); },
                None => ()
            }
        }
    }
    let vec: Vec<(usize, usize, usize)> = 
        set.iter().map(|&x| (x.0 as usize, x.1 as usize, x.2 as usize)).collect();
    if vec.len() == 2 {
        // print!("{},{},{};", vec[0].0, vec[0].1, vec[0].2);
        // println!("{},{},{}", vec[1].0, vec[1].1, vec[1].2);
        return Some([
            ((vec[0].1)..(vec[0].2 + 1))
                .fold(0 as i32, |acc, e| acc * 10 + (arr[vec[0].0][e] - '0' as u8) as i32),
            ((vec[1].1)..(vec[1].2 + 1))
                .fold(0 as i32, |acc, e| acc * 10 + (arr[vec[1].0][e] - '0' as u8) as i32)
        ]);
    }
    None
}

#[allow(dead_code)]
pub fn part2(input: Vec<String>) {
    let mut grid = vec![];
    for line in input {
        let bytes: Vec<u8> = line.as_bytes().iter().map(|&x| x).collect();
        grid.push(bytes);
    }
    let num_rows = grid.len() as i32;
    let num_cols = grid[0].len() as i32;
    let mut res = 0;
    for i in 0..num_rows {
        for j in 0..num_cols {
            if grid[i as usize][j as usize] == '*' as u8 {
                match get_numbers(&grid, i, j) {
                    Some([number1, number2]) => res += number1 * number2,
                    None => ()
                }
            }
        }
    }
    println!("{}", res);
}