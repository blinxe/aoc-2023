use std::iter::zip;

use crate::utils::input::read_input;

fn solve_part_1(input: &str) {
    let mut it = input.lines();

    let times: Vec<u32> = it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let distances: Vec<u32> = it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let mut result = 1;

    for (t, d) in zip(times.iter(), distances.iter()) {
        let mut count = 0;

        for i in 0..*t {
            let d_calc = (*t - i) * i;

            if d_calc > *d {
                count += 1;
            }
        }
        result *= count;
    }
    println!("{:?}", result);
}

fn solve_part_2(input: &str) {
    let mut it = input.lines();

    let time: u64 = it
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse()
        .unwrap();

    let distance: u64 = it
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse()
        .unwrap();

    let mut count = 0;

    for i in 0..time {
        let d_calc = (time - i) * i;

        if d_calc > distance {
            count += 1;
        }
    }
    println!("{:?}", count);
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
    const EXAMPLE_1: &str = "Time:      7  15   30
Distance:  9  40  200";

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
