mod hash_code;
mod utils;
mod problem1;
mod problem2;
mod problem3;
mod problem4;
mod problem5;

pub fn main_input(problem_number: i32) -> Vec<String> {
    utils::get_lines_from_string(utils::get_input_from_aoc(problem_number))
}

fn main() {
    // problem1::part1(main_input(1));
    // problem1::part2(main_input(1));
    // problem2::part1(main_input(2));
    // problem2::part2(main_input(2));
    // problem3::part2(main_input(3));
    // problem4::part1(main_input(4));
    // problem4::part2(main_input(4));
    problem5::part2(main_input(5));
}
