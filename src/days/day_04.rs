use std::collections::HashSet;

use crate::utils::input::read_input;

fn get_matches(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(':').unwrap().1.split_once('|').unwrap();
            let left: HashSet<u32> = left
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let right: HashSet<u32> = right
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            left.intersection(&right).cloned().collect::<Vec<u32>>()
        })
        .collect()
}

fn solve_part_1(input: &str) {
    let total = get_matches(input)
        .iter()
        .filter_map(|v: &Vec<u32>| {
            if v.len() > 0 {
                Some(2u32.pow(v.len() as u32 - 1))
            } else {
                None
            }
        })
        .sum::<u32>();

    println!("{}", total);
}

fn solve_part_2(input: &str) {
    let matches = get_matches(input);

    let mut numbers = vec![1; matches.len()];

    for (i, m) in matches.iter().enumerate() {
        for j in 1..=m.len() {
            numbers[i + j] += numbers[i];
        }
    }

    println!("{}", numbers.iter().sum::<u32>());
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
    const EXAMPLE_1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

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
