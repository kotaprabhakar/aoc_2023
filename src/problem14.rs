use std::collections::HashSet;

#[allow(dead_code)]
pub fn part1_sample_input() -> Vec<String> {
    let sample =
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    crate::utils::get_lines_from_string(sample.to_string())
}

#[allow(dead_code)]
pub fn part2_sample_input() -> Vec<String> {
    part1_sample_input()
}

pub fn get_grid(input: Vec<String>) -> Vec<Vec<char>> {
    let mut res = vec![];
    for line in input {
        res.push(line.chars().collect::<Vec<char>>());
    }
    res
}

#[allow(dead_code)]
pub fn part1(input: Vec<String>) {
    let grid = get_grid(input);
    let mut dp = vec![vec![0; grid[0].len()]; grid.len()];
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0.. grid[0].len() {
            if grid[i][j] == 'O' {
                if i == 0 {
                    dp[i][j] = 0;
                } else {
                    for r in (0..i).rev() {
                        if grid[r][j] == '#' {
                            dp[i][j] = r + 1;
                        } else if grid[r][j] == 'O' {
                            dp[i][j] = dp[r][j] + 1;
                        } else if r == 0 {
                            dp[i][j] = 0;
                        } else if grid[r][j] == '.' {
                            continue;
                        }
                        break;
                    }
                }
                res += grid.len() - dp[i][j];
            }
        }
    }
    println!("{:?}", res);
}

fn to_string(grid: &Vec<Vec<char>>) -> String {
    let mut res = String::new();
    grid.iter().for_each(|row| res = format!("{}{}", res, row.iter().cloned().collect::<String>()));
    res
}
fn rotate(grid: &mut Vec<Vec<char>>, delta: (i32, i32)) {
    let mut range_i = (0..grid.len()).collect::<Vec<usize>>();
    let mut range_j = (0..grid[0].len()).collect::<Vec<usize>>();
    if delta.0 == 1 {
        range_i = (0..grid.len()).rev().collect();
    }
    if delta.1 == 1 {
        range_j = (0..grid[0].len()).rev().collect();
    }
    for &i in range_i.iter() {
        for &j in range_j.iter() {
            if grid[i][j] == 'O' {
                let (mut pre_i, mut pre_j) = (i as i32, j as i32);
                let (mut new_i, mut new_j) = (i as i32 + delta.0, j as i32 + delta.1 );
                while new_i >= 0 && new_i < grid.len() as i32 && new_j >= 0 && new_j < grid[0].len() as i32 {
                    if grid[new_i as usize][new_j as usize] == '#' || grid[new_i as usize][new_j as usize] == 'O' {
                        break;
                    }
                    grid[pre_i as usize][pre_j as usize] = '.';
                    grid[new_i as usize][new_j as usize] = 'O';
                    pre_i += delta.0;
                    pre_j += delta.1;
                    new_i += delta.0;
                    new_j += delta.1;
                }
            }
        }
    }
}

fn cycle(grid: &mut Vec<Vec<char>>) {
    // print_grid(&grid);
    rotate(grid, (-1, 0));
    // print_grid(&grid);
    rotate(grid, (0, -1));
    // print_grid(&grid);
    rotate(grid, (1, 0));
    // print_grid(&grid);
    rotate(grid, (0, 1));
    // print_grid(&grid);
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            print!("{} ", grid[i][j]);
        }
        println!();
    }
    println!();
}

fn get_score(grid: &Vec<Vec<char>>) -> i32 {
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                res += grid.len() - i;
            }
        }
    }
    res as i32
}

#[allow(dead_code)]
pub fn part2(input: Vec<String>) {
    let mut grid = get_grid(input);
    let mut set: Vec<String> = vec![];
    let mut scores = vec![];
    let mut temp = to_string(&grid);
    while !set.contains(&temp) {
        set.push(temp.to_string());
        scores.push(get_score(&grid));
        cycle(&mut grid);
        temp = to_string(&grid);
    }
    let idx = set.iter().position(|x| x.clone() == temp).unwrap();
    println!("{}", scores[idx + (1000000000 - idx) % (set.len() - idx)]);
}