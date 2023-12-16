#[allow(dead_code)]
pub fn part1_sample_input() -> Vec<String> {
    let sample =
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    crate::utils::get_lines_from_string(sample.to_string())
}

#[allow(dead_code)]
pub fn part2_sample_input() -> Vec<String> {
    part1_sample_input()
}

fn get_grids(input: Vec<String>) -> Vec<Vec<Vec<char>>> {
    let mut res = vec![];
    let mut temp = vec![];
    for line in input {
        if !line.trim().is_empty() {
            temp.push(line.chars().collect::<Vec<char>>());
        } else {
            res.push(temp.clone());
            temp = vec![];
        }
    }
    if !temp.is_empty() {
        res.push(temp.clone());
    }
    res
}

fn are_columns_equal(grid: &Vec<Vec<char>>, j1: usize, j2: usize) -> bool {
    for i in 0..grid.len() {
        if grid[i][j1] != grid[i][j2] {
            return false;
        }
    }
    true
}

fn are_rows_equal(grid: &Vec<Vec<char>>, i1: usize, i2: usize) -> bool {
    grid[i1] == grid[i2]
}

#[allow(dead_code)]
pub fn part1(input: Vec<String>) {
    let mut res = 0;
    for grid in get_grids(input) {
        let mut found = false;
        for (j1, j2) in (0..(grid[0].len() - 1)).zip(1..grid[0].len()) {
            let num_left = j2;
            let num_right = grid[0].len() - j2;
            let num_cols = std::cmp::min(num_left, num_right);
            let mut temp_found = true;
            for (k1, k2) in ((j1 + 1 - num_cols)..(j1 + 1)).zip((j2..(j2 + num_cols)).rev()) {
                if !are_columns_equal(&grid, k1, k2) {
                    temp_found = false;
                    break;
                }
            }
            if temp_found {
                res += j2;
                found = true;
                break;
            }
        }
        if found {
            continue;
        }
        for (i1, i2) in (0..(grid.len() - 1)).zip(1..grid.len()) {
            let num_up = i2;
            let num_down = grid.len() - i2;
            let num_rows = std::cmp::min(num_up, num_down);
            let mut temp_found = true;
            for (k1, k2) in ((i1 + 1 - num_rows)..(i1 + 1)).zip((i2..(i2 + num_rows)).rev()) {
                if !are_rows_equal(&grid, k1, k2) {
                    temp_found = false;
                    break;
                }
            }
            if temp_found {
                res += 100 * i2;
                found = true;
                break;
            }
        }
    }
    println!("{:?}", res);
}

fn get_diff_rows(grid: &Vec<Vec<char>>, i1: usize, i2: usize) -> Option<usize> {
    let mut count = 0;
    for j in 0..grid[0].len() {
        if grid[i1][j] != grid[i2][j] {
            count += 1;
        }
        if count > 1 {
            return None;
        }
    }
    Some(count)
}

fn get_diff_cols(grid: &Vec<Vec<char>>, j1: usize, j2: usize) -> Option<usize> {
    let mut count = 0;
    for i in 0..grid.len() {
        if grid[i][j1] != grid[i][j2] {
            count += 1;
        }
        if count > 1 {
            return None;
        }
    }
    Some(count)
}

fn get_mirror_row(grid: &Vec<Vec<char>>, in_diff: usize) -> Option<usize> {
    for (i1, i2) in (0..(grid.len() - 1)).zip(1..grid.len()) {
        let num_rows = std::cmp::min(i2, grid.len() - i2);
        let mut count = 0i32;
        for (k1, k2) in ((i1 + 1 - num_rows)..(i1 + 1)).zip((i2..(i2 + num_rows)).rev()) {
            match get_diff_rows(grid, k1, k2) {
                Some(diff) => count += diff as i32,
                None => { count = -1; break; }
            }
            if count > 1 {
                break;
            }
        }
        if count == in_diff as i32 {
            return Some(i1);
        }
    }
    None
}

fn get_mirror_col(grid: &Vec<Vec<char>>, in_diff: usize) -> Option<usize> {
    for (j1, j2) in (0..(grid[0].len() - 1)).zip(1..grid[0].len()) {
        let num_cols = std::cmp::min(j2, grid[0].len() - j2);
        let mut count = 0i32;
        for (k1, k2) in ((j1 + 1 - num_cols)..(j1 + 1)).zip((j2..(j2 + num_cols)).rev()) {
            match get_diff_cols(grid, k1, k2) {
                Some(diff) => count += diff as i32,
                None => { count = -1; break; }
            }
            if count > 1 {
                break;
            }
        }
        if count == in_diff as i32 {
            return Some(j1);
        }
    }
    None
}

#[allow(dead_code)]
pub fn part2(input: Vec<String>) {
    let mut res = 0;
    let grids = get_grids(input);
    for grid in grids {
        match get_mirror_row(&grid, 1) {
            Some(i) => res += 100 * (i + 1),
            None => ()
        }
        match get_mirror_col(&grid, 1) {
            Some(j) => res += j + 1,
            None => ()
        }
    }
    println!("{:?}", res);
}