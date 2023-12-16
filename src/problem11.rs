use std::collections::BTreeSet;
use std::ops::Bound::Included;

#[allow(dead_code)]
pub fn part1_sample_input() -> Vec<String> {
    let sample =
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    crate::utils::get_lines_from_string(sample.to_string())
}

#[allow(dead_code)]
pub fn part2_sample_input() -> Vec<String> {
    part1_sample_input()
}

fn get_grid(input: Vec<String>) -> Vec<Vec<char>> {
    let mut res = vec![];
    for line in input {
        res.push(line.chars().collect::<Vec<char>>());
    }
    res
}

fn get_galaxies(grid: Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut res = vec![];
    for (r, row) in grid.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == '#' {
                res.push((r as i32, c as i32));
            }
        }
    }
    res
}

pub fn part1(input: Vec<String>) {
    let grid = get_grid(input);
    let mut new_grid1 = vec![vec![]; grid.len()];
    let mut new_grid2 = vec![];

    for j in 0..grid[0].len() {
        (0..grid.len()).for_each(|i| new_grid1[i].push(grid[i][j]));
        if (0..grid.len()).all(|i| grid[i][j] == '.') {
            (0..grid.len()).for_each(|i| new_grid1[i].push(grid[i][j]));
        }
    }

    for i in 0..new_grid1.len() {
        new_grid2.push(new_grid1[i].clone());
        if new_grid1[i].iter().all(|x| *x == '.') {
            new_grid2.push(new_grid1[i].clone());
        }
    }

    let galaxies = get_galaxies(new_grid2);

    let mut res = 0;

    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let first = galaxies[i];
            let second = galaxies[j];
            res += (first.0 - second.0).abs() + (first.1 - second.1).abs();
        }
    }

    println!("{:?}", res);
}

pub fn part2(input: Vec<String>) {
    let grid = get_grid(input);
    let galaxies = get_galaxies(grid.clone());

    let mut empty_rows = BTreeSet::new();
    let mut empty_cols = BTreeSet::new();

    for i in 0..grid.len() {
        if grid[i].iter().all(|row| *row == '.') {
            empty_rows.insert(i);
        }
    }

    for j in 0..grid[0].len() {
        if (0..grid.len()).all(|i| grid[i][j] == '.') {
            empty_cols.insert(j);
        }
    }
    let num_replacements = 1000000u64;

    let mut res = 0u64;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let first = galaxies[i];
            let second = galaxies[j];
            let row0 = std::cmp::min(first.0, second.0) as usize;
            let row1 = std::cmp::max(first.0, second.0) as usize;
            let col0 = std::cmp::min(first.1, second.1) as usize;
            let col1 = std::cmp::max(first.1, second.1) as usize;
            let num_empty_rows =
                empty_rows.range((Included(&row0), Included(&row1))).count() as i32;
            let num_empty_cols =
                empty_cols.range((Included(&col0), Included(&col1))).count() as i32;
            // println!("{:?},{:?},{:?},{:?}", first, second, num_empty_rows, num_empty_cols);
            res += (first.0 - second.0).abs() as u64 - num_empty_rows as u64 +
                    (num_empty_rows as u64) * num_replacements +
                    (first.1 - second.1).abs() as u64 - num_empty_cols as u64 +
                    (num_empty_cols as u64) * num_replacements;
        }
    }
    println!("{}", res);
}