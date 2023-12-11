mod hash_code;
mod utils;
mod problem1;
mod problem2;
mod problem3;
mod problem4;
mod problem5;
mod problem6;
mod problem7;
mod problem8;
mod problem9;
mod problem10;

pub fn main_input(problem_number: i32) -> Vec<String> {
    utils::get_lines_from_string(utils::get_input_from_aoc(problem_number))
}

fn main() {
    // problem1::part1(main_input(1));
    // problem1::part2(main_input(1));
    // problem2::part1(main_input(2));
    // problem2::part2(main_input(2));
    // problem3::part1(main_input(3));
    // problem3::part2(main_input(3));
    // problem4::part1(main_input(4));
    // problem4::part2(main_input(4));
    // problem5::part1(main_input(5));
    // problem5::part2(main_input(5));
    // problem6::part1(main_input(6));
    // problem6::part2(main_input(6));
    // problem7::part1(main_input(7));
    // problem7::part2(main_input(7));
    // problem8::part1(main_input(8));
    // problem8::part2(main_input(8));
    // problem9::part1(main_input(9));
    // problem9::part2(main_input(9));
    problem10::part1(problem10::part1_sample_input());
    problem10::part2(problem10::part2_sample_input());
}
