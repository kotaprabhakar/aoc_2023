use std::collections::BTreeMap;

#[allow(dead_code)]
pub fn part1_sample_input() -> Vec<String> {
    let sample = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4";
    crate::utils::get_lines_from_string(sample.to_string())
}

#[allow(dead_code)]
pub fn part2_sample_input() -> Vec<String> {
    part1_sample_input()
}

fn process_input_part1(input: Vec<String>) -> (Vec<i64>, Vec<BTreeMap<i64, (i64, i64)>>) {
    let mut seeds = vec![];
    let mut maps = vec![BTreeMap::new(); 7];
    let mut idx: i64 = -1;
    for line in input {
        if !line.trim().is_empty() {
            if line.contains("seeds") {
                seeds = 
                    line
                        .split(':')
                        .collect::<Vec<&str>>()[1]
                        .trim()
                        .split(' ')
                        .map(|x| x.trim().parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
            } else if line.contains("map") {
                idx += 1;
            } else {
                let numbers = 
                    line    
                        .split(' ')
                        .filter(|x| !x.is_empty())
                        .map(|x| x.trim().parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                assert!(numbers.len() == 3);
                maps[idx as usize].insert(numbers[1], (numbers[0], numbers[2]));
            }
        }
    }
    (seeds, maps)
}

#[allow(dead_code)]
pub fn part1(input: Vec<String>) {
    let (seeds, maps) = process_input_part1(input);
    let mut res = i64::MAX;

    for seed in seeds {
        let mut temp = seed;
        for map in maps.iter() {
            let y = 
                map
                    .range((std::collections::Bound::Unbounded, std::collections::Bound::Included(&temp)))
                    .last();
            match y {
                Some(z) => {
                    if temp < z.0 + z.1.1 {
                        temp = z.1.0 + (temp - z.0);
                    }
                },
                None => ()
            }
        }
        res = std::cmp::min(res, temp);
    }
    println!("{}", res);
}

fn process_input_part2(input: Vec<String>) -> (Vec<(i64, i64)>, Vec<Vec<(i64, i64, i64)>>) {
    let mut idx = -1i64;
    let mut seed_ranges = vec![];
    let mut range_maps = vec![vec![]; 7];
    for untrimmed_line in input {
        let line = untrimmed_line.trim();
        if line.contains("seeds") {
            let iter = line[7..].split(' ').filter(|s| !s.is_empty()).map(|s| s.trim().parse::<i64>().unwrap());
            seed_ranges = iter.clone().step_by(2).zip(iter.skip(1).step_by(2)).map(|(x, y)| (x, x + y)).collect::<Vec<(i64, i64)>>();
            seed_ranges.sort();
        } else if line.contains("map") {
            idx += 1;
        } else if !line.is_empty() {
            let triple = line[0..].split(' ').filter(|s| !s.is_empty()).map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            range_maps[idx as usize].push((triple[1], triple[1] + triple[2], triple[0]));
        }
    }
    range_maps.iter_mut().for_each(|range| range.sort());

    (seed_ranges, range_maps)
}

fn get_new_seed_ranges(seed_range: &(i64, i64), range_map: &Vec<(i64, i64, i64)>) -> Vec<(i64, i64)> {
    let mut res = vec![];
    // println!("{:?}", seed_range);
    if seed_range.0 < range_map[0].0 {
        res.push((seed_range.0, range_map[0].0));
    }
    for (&range1, &range2) in range_map.iter().zip(range_map.iter().skip(1)) {
        let limit1 = std::cmp::max(seed_range.0, range1.0);
        let limit2 = std::cmp::min(seed_range.1, range1.1);
        let limit3 = std::cmp::max(seed_range.0, range1.1);
        let limit4 = std::cmp::min(seed_range.1, range2.0);
        if limit1 < limit2 {
            res.push((range1.2 + limit1 - range1.0, range1.2 + limit2 - range1.0));
        }
        if limit3 < limit4 {
            res.push((limit3, limit4));            
        }
    }
    if seed_range.1 > range_map.last().unwrap().0 {
        let limit1 = std::cmp::max(seed_range.0, range_map.last().unwrap().0);
        let limit2 = std::cmp::min(seed_range.1, range_map.last().unwrap().1);
        if limit1 < limit2 {
            res.push((range_map.last().unwrap().2 + limit1 - range_map.last().unwrap().0, range_map.last().unwrap().2 + limit2 - range_map.last().unwrap().0));
        }
        if seed_range.1 > range_map.last().unwrap().1 {
            res.push((std::cmp::max(seed_range.0, range_map.last().unwrap().1), seed_range.1));
        }
    }
    // res.sort();
    res
}

#[allow(dead_code)]
pub fn part2(input: Vec<String>) {
    let (mut seed_ranges, range_maps) = process_input_part2(input);
    // println!("{:?}", seed_ranges);
    // println!("{:?}", range_maps);
    for range_map in range_maps.iter() {
        let mut new_seed_ranges = vec![];
        for seed_range in seed_ranges.iter() {
            let temp = get_new_seed_ranges(seed_range, range_map);
            new_seed_ranges.extend(temp.iter());
        }
        new_seed_ranges.sort();
        seed_ranges = new_seed_ranges;
    }
    println!("{}", seed_ranges[0].0);
}