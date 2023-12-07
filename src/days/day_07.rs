use std::{cmp::Ordering, collections::HashMap};

use crate::utils::input::read_input;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    High,
    OnePair,
    TwoPairs,
    Three,
    Full,
    Four,
    Five,
}

use HandType::*;

fn get_type(h: &str) -> HandType {
    if h.chars().all(|c| c == h.chars().nth(0).unwrap()) {
        return Five;
    }

    for c in h.chars().take(2) {
        if h.chars().filter(|c2| *c2 == c).count() == 4 {
            return Four;
        }
    }

    for c in h.chars().take(3) {
        if h.chars().filter(|c2| *c2 == c).count() == 3 {
            let mut it = h.chars().filter(|c2| *c2 != c);
            if it.next() == it.next() {
                return Full;
            }
            return Three;
        }
    }

    let mut pairs = 0;
    for c in h.chars() {
        if h.chars().filter(|c2| *c2 == c).count() == 2 {
            pairs += 1;
        }
    }
    match pairs {
        4 => TwoPairs,
        2 => OnePair,
        _ => High,
    }
}

fn compare_hands(h1: &str, h2: &str) -> Ordering {
    let scores: HashMap<char, u32> = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);

    let t1 = get_type(h1);
    let t2 = get_type(h2);

    let mut cmp = t1.cmp(&t2);
    if cmp == Ordering::Equal {
        let it1 = h1.chars().map(|c| scores.get(&c).unwrap());
        let it2 = h2.chars().map(|c| scores.get(&c).unwrap());
        cmp = it1.cmp(it2);
    }

    cmp
}

fn solve_part_1(input: &str) {
    let mut data = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bid)| (hand, bid.parse::<u32>().unwrap()))
        .collect::<Vec<(&str, u32)>>();

    data.sort_by(|(h1, _), (h2, _)| compare_hands(h1, h2));

    for d in &data {
        println!("{:?}", d);
    }

    let total = data
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u32 + 1) * *bid)
        .sum::<u32>();

    println!("{}", total);
}

fn get_type_p2(h: &str) -> HandType {
    for c in h.chars() {
        if h.chars().filter(|c2| *c2 == c || *c2 == 'J').count() == 5 {
            return Five;
        }
    }

    for c in h.chars() {
        if h.chars().filter(|c2| *c2 == c || *c2 == 'J').count() == 4 {
            return Four;
        }
    }

    for c in h.chars() {
        if h.chars().filter(|c2| *c2 == c || *c2 == 'J').count() == 3 {
            let mut it = h.chars().filter(|c2| *c2 != c && *c2 != 'J');
            if it.next() == it.next() {
                return Full;
            }
            return Three;
        }
    }

    let mut pairs = 0;
    for c in h.chars() {
        if c == 'J' {
            pairs += 2;
        } else if h.chars().filter(|c2| *c2 == c).count() == 2 {
            pairs += 1;
        }
    }
    match pairs {
        4 => TwoPairs,
        2 => OnePair,
        _ => High,
    }
}

fn compare_hands_p2(h1: &str, h2: &str) -> Ordering {
    let scores: HashMap<char, u32> = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 1), // <- weakest card
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);

    let t1 = get_type_p2(h1);
    let t2 = get_type_p2(h2);

    let mut cmp = t1.cmp(&t2);
    if cmp == Ordering::Equal {
        let it1 = h1.chars().map(|c| scores.get(&c).unwrap());
        let it2 = h2.chars().map(|c| scores.get(&c).unwrap());
        cmp = it1.cmp(it2);
    }

    cmp
}

fn solve_part_2(input: &str) {
    let mut data = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bid)| (hand, bid.parse::<u32>().unwrap()))
        .collect::<Vec<(&str, u32)>>();

    data.sort_by(|(h1, _), (h2, _)| compare_hands_p2(h1, h2));

    for d in &data {
        println!("{:?}", d);
    }

    let total = data
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u32 + 1) * *bid)
        .sum::<u32>();

    println!("{}", total);
}

pub fn part_1() {
    let input = read_input(module_path!());
    solve_part_1(input.as_str());
}

pub fn part_2() {
    let input = read_input(module_path!());
    solve_part_2(input.as_str());
}

#[cfg(test)]
mod test {
    const EXAMPLE_1: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part_1() {
        super::solve_part_1(EXAMPLE_1);
    }

    const EXAMPLE_2: &str = EXAMPLE_1;

    #[test]
    fn test_part_2() {
        super::solve_part_2(EXAMPLE_2);
    }
}
