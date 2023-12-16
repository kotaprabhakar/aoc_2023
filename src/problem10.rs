use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
pub fn part1_sample_input() -> Vec<String> {
    let sample =
        "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    crate::utils::get_lines_from_string(sample.to_string())
}

#[allow(dead_code)]
pub fn part2_sample_input() -> Vec<String> {
    let sample =
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    crate::utils::get_lines_from_string(sample.to_string())
}

fn get_data() -> (Vec<char>, HashMap<char, ((i32, i32), (i32, i32))>, HashMap<char, (HashSet<char>, HashSet<char>)>) {
    let pipes = vec!['-', '|', 'L', 'J', 'F', '7'];
    let neighbor_indices = HashMap::from(
        [
            ('-', ((0, -1), (0, 1))),
            ('|', ((-1, 0), (1, 0))),
            ('L', ((-1, 0), (0, 1))),
            ('J', ((-1, 0), (0, -1))),
            ('F', ((0, 1), (1, 0))),
            ('7', ((0, -1), (1, 0)))
        ]
    );
    let neighbor_pipes = HashMap::from(
        [
            ('-', (HashSet::from(['-', 'F', 'L']), HashSet::from(['-', '7', 'J']))),
            ('|', (HashSet::from(['|', '7', 'F']), HashSet::from(['|', 'L', 'J']))),
            ('L', (HashSet::from(['|', '7', 'F']), HashSet::from(['-', '7', 'J']))),
            ('J', (HashSet::from(['|', '7', 'F']), HashSet::from(['-', 'F', 'L']))),
            ('F', (HashSet::from(['-', '7', 'J']), HashSet::from(['|', 'L', 'J']))),
            ('7', (HashSet::from(['-', 'F', 'L']), HashSet::from(['|', 'L', 'J'])))
        ]
    );
    (pipes, neighbor_indices, neighbor_pipes)
}

fn get_grid(input: Vec<String>) -> Vec<Vec<char>> {
    let mut grid = vec![];
    input.iter().for_each(|line| grid.push(line.trim().chars().collect::<Vec<char>>()));
    grid
}

fn get_loop(input: Vec<String>) -> Vec<(i32, i32)> {
    let mut res = vec![];
    let (pipes, neighbor_indices, neighbor_pipes) = get_data();

    let mut start = (0i32, 0i32);
    let mut pipe_at_start = '\0';

    let mut grid = get_grid(input);

    grid.iter()
        .enumerate()
        .for_each(|(idx1, row)| row.iter().enumerate().for_each(|(idx2, &col)| { if col == 'S' { start = (idx1 as i32, idx2 as i32); } }));

    for pipe in pipes.iter() {
        let curr_ch = *pipe;
        let delta0 = neighbor_indices[&curr_ch].0;
        let delta1 = neighbor_indices[&curr_ch].1;
        let neighbor0 = (start.0 + delta0.0, start.1 + delta0.1);
        let neighbor1 = (start.0 + delta1.0, start.1 + delta1.1);
        if neighbor0.0 >= 0 && neighbor0.0 < grid.len() as i32 && neighbor0.1 >= 0 && neighbor0.1 < grid[0].len() as i32 &&
            neighbor1.0 >= 0 && neighbor1.0 < grid.len() as i32 && neighbor1.1 >= 0 && neighbor1.1 < grid[0].len() as i32 &&
            neighbor_pipes[&curr_ch].0.contains(&grid[neighbor0.0 as usize][neighbor0.1 as usize]) &&
            neighbor_pipes[&curr_ch].1.contains(&grid[neighbor1.0 as usize][neighbor1.1 as usize]) {
            pipe_at_start = pipe.clone();
            break;
        }
    }
    grid[start.0 as usize][start.1 as usize] = pipe_at_start;

    let mut prev = start.clone();
    let mut curr = (prev.0 + neighbor_indices[&pipe_at_start].0.0, prev.1 + neighbor_indices[&pipe_at_start].0.1);
    res.push(prev);
    while curr != start {
        res.push(curr);
        let curr_ch = grid[curr.0 as usize][curr.1 as usize];
        let delta0 = neighbor_indices[&curr_ch].0;
        let delta1 = neighbor_indices[&curr_ch].1;
        let neighbor0 = (curr.0 + delta0.0, curr.1 + delta0.1);
        let neighbor1 = (curr.0 + delta1.0, curr.1 + delta1.1);
        let temp = curr;
        curr = if prev == neighbor0 {  neighbor1 } else { neighbor0 };
        prev = temp;
    }
    res
}

pub fn part1(input: Vec<String>) {
    let loop_indices = get_loop(input);
    println!("{:?}", if (loop_indices.len() + 1) % 2 == 0 { (loop_indices.len() + 1) / 2 } else { loop_indices.len() / 2 });
}

pub fn part2(input: Vec<String>) {
    let mut grid = get_grid(input.clone());
    let mut loop_indices = get_loop(input);
    let mut area = 0;

    // Shoelace theorem
    loop_indices.push(loop_indices.first().unwrap().clone());
    for ((x1, y1), (x2, y2)) in loop_indices.iter().zip(loop_indices.iter().skip(1)) {
        area += x1 * y2 - x2 * y1;
    }
    area /= 2;

    // Pick's theorem
    println!("{:?}", area.abs() - (loop_indices.len() as i32 - 1) / 2 + 1);
}