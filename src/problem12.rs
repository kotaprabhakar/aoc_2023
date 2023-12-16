#[allow(dead_code)]
pub fn part1_sample_input() -> Vec<String> {
    let sample =
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    crate::utils::get_lines_from_string(sample.to_string())
}

#[allow(dead_code)]
pub fn part2_sample_input() -> Vec<String> {
    part1_sample_input()
}

fn parse_line(line: &String) -> (Vec<char>, Vec<i32>) {
    let line_parts = line.split(' ').map(|x| x.to_string()).collect::<Vec<String>>();
    let springs = line_parts[0].chars().collect::<Vec<char>>();
    let group_counts = line_parts[1].split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    (springs, group_counts)
}

fn get_groups(springs: &Vec<char>) -> Vec<i32> {
    springs
        .split(|c| *c == '.' || *c == '?')
        .filter(|x| x.len() != 0)
        .map(|x| x.len() as i32)
        .collect::<Vec<i32>>()
}

fn recurse(springs: &mut Vec<char>, idx1: i32, group_counts: &Vec<i32>, count: &mut i64) {
    let mut check = false;
    for idx in (idx1 as usize)..springs.len() {
        if springs[idx] == '?' {
            check = true;
            springs[idx] = '#';
            recurse(springs, idx1 + 1, group_counts, count);
            springs[idx] = '.';
            recurse(springs, idx1 + 1, group_counts, count);
            springs[idx] = '?';
            break;
        }
    }
    if check == false && get_groups(&(*springs)) == *group_counts {
        // println!("{:?}", springs);
        *count += 1;
    }
}

#[allow(dead_code)]
pub fn part1(input: Vec<String>) {
    let mut res = 0;
    for line in input {
        let (mut springs, group_counts) = parse_line(&line);
        let mut count = 0;
        recurse(&mut springs, 0, &group_counts, &mut count);
        res += count;
    }
    println!("{}", res);
}

#[allow(dead_code)]
pub fn part2(input: Vec<String>) {

}