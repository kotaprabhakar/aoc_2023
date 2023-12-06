#[allow(dead_code)]
pub fn part1_sample_input() -> Vec<String> {
    let sample =
        "Time:      7  15   30
Distance:  9  40  200";
    crate::utils::get_lines_from_string(sample.to_string())
}

#[allow(dead_code)]
pub fn part2_sample_input() -> Vec<String> {
    part1_sample_input()
}

pub fn part1(input: Vec<String>) {
    assert_eq!(2, input.len());
    let t = input[0][6..].trim().split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let r = input[1][9..].trim().split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut res = 1.0;
    for (&t, &r) in t.iter().zip(r.iter()) {
        let mut f = (t as f64 - (t as f64 * t as f64 - 4.0 * r as f64).sqrt()) / 2.0;
        let mut s = (t as f64 + (t as f64 * t as f64 - 4.0 * r as f64).sqrt()) / 2.0;
        f =
            if (f as i64) as f64 == f {
                f.ceil() + 1.0
            } else {
                f.ceil()
            };
        s =
            if (s as i64) as f64 == s {
                s.floor() - 1.0
            } else {
                s.floor()
            };
        if s >= f {
            res *= (s - f + 1.0);
        }
    }
    println!("{}", res);
}

pub fn part2(input: Vec<String>) {
    let t = input[0][6..].trim().split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
        .fold("".to_string(), |acc, e| format!("{}{}", acc, e))
        .parse::<i64>()
        .unwrap();
    let r = input[1][9..].trim().split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
        .fold("".to_string(), |acc, e| format!("{}{}", acc, e))
        .parse::<i64>()
        .unwrap();
    // println!("{},{}", t, r);
    let mut f = (t as f64 - (t as f64 * t as f64 - 4.0 * r as f64).sqrt()) / 2.0;
    let mut s = (t as f64 + (t as f64 * t as f64 - 4.0 * r as f64).sqrt()) / 2.0;
    f =
        if (f as i64) as f64 == f {
            f.ceil() + 1.0
        } else {
            f.ceil()
        };
    s =
        if (s as i64) as f64 == s {
            s.floor() - 1.0
        } else {
            s.floor()
        };
    println!("{}", s - f + 1.0);
}