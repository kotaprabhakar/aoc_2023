use std::cmp::Ordering;
use std::collections::HashMap;
use reqwest::get;

#[allow(dead_code)]
pub fn part1_sample_input() -> Vec<String> {
    let sample =
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    crate::utils::get_lines_from_string(sample.to_string())
}

#[allow(dead_code)]
pub fn part2_sample_input() -> Vec<String> {
    part1_sample_input()
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    None,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

fn get_type_part1(hand: &str) -> HandType {
    let mut counts = HashMap::<char, i64>::new();
    hand.chars().for_each(|ch| { let val = counts.entry(ch).or_insert(0); *val += 1; } );
    let mut sorted_counts = counts.iter().map(|(x, y)| (*y, *x)).collect::<Vec<(i64, char)>>();
    sorted_counts.sort();
    sorted_counts.reverse();
    if sorted_counts.len() == 1 && sorted_counts[0].0 == 5 {
        return HandType::FiveOfAKind;
    } else if sorted_counts.len() == 2 && sorted_counts[0].0 == 4 && sorted_counts[1].0 == 1 {
        return HandType::FourOfAKind;
    } else if sorted_counts.len() == 2 && sorted_counts[0].0 == 3 && sorted_counts[1].0 == 2 {
        return HandType::FullHouse;
    } else if sorted_counts.len() == 3 && sorted_counts[0].0 == 3 && sorted_counts[1].0 == 1 && sorted_counts[2].0 == 1 {
        return HandType::ThreeOfAKind;
    }  else if sorted_counts.len() == 3 && sorted_counts[0].0 == 2 && sorted_counts[1].0 == 2 && sorted_counts[2].0 == 1 {
        return HandType::TwoPair;
    } else if sorted_counts.len() == 4 && sorted_counts[0].0 == 2
                && sorted_counts[1].0 == 1 && sorted_counts[2].0 == 1 && sorted_counts[3].0 == 1 {
        return HandType::OnePair;
    } else if sorted_counts.len() == 5 && sorted_counts[0].0 == 1 && sorted_counts[1].0 == 1
                && sorted_counts[2].0 == 1 && sorted_counts[3].0 == 1 && sorted_counts[4].0 == 1 {
        return HandType::HighCard;
    }
    HandType::None
}

fn compare_hands_part1(hand1: &(HandType, String, i64), hand2: &(HandType, String, i64)) -> Ordering {
    if hand1.0 != hand2.0 {
        return (hand1.0 as i64).cmp(&(hand2.0 as i64));
    }
    let card_order: HashMap<char, i32> =
    HashMap::from([('A', 13), ('K', 12), ('Q', 11), ('J', 10), ('T', 9), ('9', 8),
                        ('8', 7), ('7', 6), ('6', 5), ('5', 4), ('4', 3), ('3', 2),
                        ('2', 1)]);

    let hand1_chars = hand1.1.chars().collect::<Vec<char>>();
    let hand2_chars = hand2.1.chars().collect::<Vec<char>>();
    for (ch1, ch2) in hand1_chars.iter().zip(hand2_chars.iter()) {
        if card_order[ch1] != card_order[ch2] {
            return card_order[ch1].cmp(&card_order[ch2]);
        }
    }
    Ordering::Equal
}

pub fn part1(input: Vec<String>) {
    let mut hands = vec![];
    for line in input {
        let parsed_line = line.split(' ').collect::<Vec<&str>>();
        assert_eq!(2, parsed_line.len());
        let hand = parsed_line[0].to_string();
        let bid = parsed_line[1];
        hands.push((get_type_part1(hand.as_str()), hand, bid.parse::<i64>().unwrap()));
    }
    hands.sort_by(compare_hands_part1);
    let mut res = 0;
    for (idx, hand) in hands.iter().enumerate() {
        res += (idx as i64 + 1) * hand.2;
    }
    println!("{}", res);
}

fn get_type_part2(hand: &str) -> HandType {
    let mut res = HandType::None;
    for c in ["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2"] {
        let new_hand = hand.replace("J", c);
        res = std::cmp::max(res, get_type_part1(new_hand.as_str()));
    }
    res
}

fn compare_hands_part2(hand1: &(HandType, String, i64), hand2: &(HandType, String, i64)) -> Ordering {
    if hand1.0 != hand2.0 {
        return (hand1.0 as i64).cmp(&(hand2.0 as i64));
    }
    let card_order: HashMap<char, i32> =
        HashMap::from([('A', 13), ('K', 12), ('Q', 11), ('J', 0), ('T', 9), ('9', 8),
            ('8', 7), ('7', 6), ('6', 5), ('5', 4), ('4', 3), ('3', 2),
            ('2', 1)]);

    let hand1_chars = hand1.1.chars().collect::<Vec<char>>();
    let hand2_chars = hand2.1.chars().collect::<Vec<char>>();
    for (ch1, ch2) in hand1_chars.iter().zip(hand2_chars.iter()) {
        if card_order[ch1] != card_order[ch2] {
            return card_order[ch1].cmp(&card_order[ch2]);
        }
    }
    Ordering::Equal
}

pub fn part2(input: Vec<String>) {
    let mut hands = vec![];
    for line in input {
        let parsed_line = line.split(' ').collect::<Vec<&str>>();
        assert_eq!(2, parsed_line.len());
        let hand = parsed_line[0].to_string();
        let bid = parsed_line[1];
        hands.push((get_type_part2(hand.as_str()), hand, bid.parse::<i64>().unwrap()));
    }
    hands.sort_by(compare_hands_part2);
    let mut res = 0;
    for (idx, hand) in hands.iter().enumerate() {
        res += (idx as i64 + 1) * hand.2;
    }
    println!("{}", res);
}