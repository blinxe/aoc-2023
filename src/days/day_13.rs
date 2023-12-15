use std::iter::zip;

use crate::utils::input::read_input;

fn find_mirror(p: &Vec<Vec<char>>) -> Option<i32> {
    for ((i, l1), l2) in zip(p.iter().enumerate(), p.iter().skip(1)) {
        if l1 != l2 {
            continue;
        }
        let i = i as i32;
        let mut diff = 1;
        let mut mirror = true;
        while (i - diff) >= 0 && i + 1 + diff < p.len() as i32 && mirror {
            if p[(i - diff) as usize] != p[(i + 1 + diff) as usize] {
                mirror = false;
            }
            diff += 1;
        }
        if mirror {
            return Some(i);
        }
    }
    None
}

fn solve_part_1(input: &str) {
    let patterns: Vec<Vec<Vec<char>>> = input
        .split("\n\n")
        .map(|p| p.lines().map(|l| l.chars().collect()).collect())
        .collect();

    let mut result = 0;

    for p in patterns {
        let mut partial_res = 0;
        let mirror = find_mirror(&p);
        if mirror.is_some() {
            partial_res += 100 * (mirror.unwrap() + 1);
        }

        let mut flip_p: Vec<Vec<char>> = Vec::new();
        for c in 0..p[0].len() {
            flip_p.push(p.iter().map(|l| l[c]).collect());
        }

        let mirror = find_mirror(&flip_p);
        if mirror.is_some() {
            partial_res += mirror.unwrap() + 1;
        }
        if partial_res == 0 {
            println!("{:?}", p);
        } else {
            result += partial_res;
        }
    }

    println!("{}", result);
}

fn count_diff(l1: &Vec<char>, l2: &Vec<char>) -> usize {
    zip(l1, l2).filter(|(c1, c2)| c1 != c2).count()
}

fn find_mirror_with_smudge(p: &Vec<Vec<char>>) -> Option<i32> {
    for ((i, l1), l2) in zip(p.iter().enumerate(), p.iter().skip(1)) {
        let mut diff = count_diff(l1, l2);
        if diff > 1 {
            continue;
        }
        let i = i as i32;
        let mut delta = 1;
        let mut mirror = true;
        while (i - delta) >= 0 && i + 1 + delta < p.len() as i32 && mirror {
            diff += count_diff(&p[(i - delta) as usize], &p[(i + 1 + delta) as usize]);
            if diff > 1 {
                mirror = false;
            }
            delta += 1;
        }
        if mirror && diff == 1 {
            return Some(i);
        }
    }
    None
}

fn solve_part_2(input: &str) {
    let patterns: Vec<Vec<Vec<char>>> = input
        .split("\n\n")
        .map(|p| p.lines().map(|l| l.chars().collect()).collect())
        .collect();

    let mut result = 0;

    for p in patterns {
        let mut partial_res = 0;
        let mirror = find_mirror_with_smudge(&p);
        if mirror.is_some() {
            partial_res += 100 * (mirror.unwrap() + 1);
        }

        let mut flip_p: Vec<Vec<char>> = Vec::new();
        for c in 0..p[0].len() {
            flip_p.push(p.iter().map(|l| l[c]).collect());
        }

        let mirror = find_mirror_with_smudge(&flip_p);
        if mirror.is_some() {
            partial_res += mirror.unwrap() + 1;
        }
        if partial_res == 0 {
            println!("{:?}", p);
        } else {
            result += partial_res;
        }
    }

    println!("{}", result);
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
    const EXAMPLE_1: &str = "#.##..##.
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
