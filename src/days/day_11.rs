use std::collections::HashSet;

use crate::utils::input::read_input;

fn solve_part_1(input: &str) {
    let mut glxs: HashSet<(i64, i64)> = HashSet::<(i64, i64)>::new();

    for (l, line) in input.lines().enumerate() {
        for (c, chr) in line.chars().enumerate() {
            if chr == '#' {
                glxs.insert((l as i64, c as i64));
            }
        }
    }

    let empty_lines = (0..input.lines().count())
        .into_iter()
        .filter(|l| glxs.iter().all(|(gl, _)| *gl != *l as i64))
        .map(|l| l as i64)
        .collect::<Vec<i64>>();
    let empty_cols = (0..input.lines().next().unwrap().len())
        .into_iter()
        .filter(|c| glxs.iter().all(|(_, gc)| *gc != *c as i64))
        .map(|c| c as i64)
        .collect::<Vec<i64>>();

    let mut total = 0i64;

    for (i, g1) in glxs.iter().enumerate() {
        let ex_l_1 = empty_lines.iter().filter(|l| **l < g1.0).count() as i64;
        let ex_c_1 = empty_cols.iter().filter(|c| **c < g1.1).count() as i64;
        for g2 in glxs.iter().skip(i + 1) {
            let ex_l_2 = empty_lines.iter().filter(|l| **l < g2.0).count() as i64;
            let ex_c_2 = empty_cols.iter().filter(|c| **c < g2.1).count() as i64;
            let dist =
                i64::abs(g2.0 - g1.0 + ex_l_2 - ex_l_1) + i64::abs(g2.1 - g1.1 + ex_c_2 - ex_c_1);
            total += dist;
        }
    }

    println!("{}", total);
}

fn solve_part_2(input: &str) {
    let mut glxs: HashSet<(i64, i64)> = HashSet::<(i64, i64)>::new();

    for (l, line) in input.lines().enumerate() {
        for (c, chr) in line.chars().enumerate() {
            if chr == '#' {
                glxs.insert((l as i64, c as i64));
            }
        }
    }

    let empty_lines = (0..input.lines().count())
        .into_iter()
        .filter(|l| glxs.iter().all(|(gl, _)| *gl != *l as i64))
        .map(|l| l as i64)
        .collect::<Vec<i64>>();
    let empty_cols = (0..input.lines().next().unwrap().len())
        .into_iter()
        .filter(|c| glxs.iter().all(|(_, gc)| *gc != *c as i64))
        .map(|c| c as i64)
        .collect::<Vec<i64>>();

    let mut total = 0i64;

    for (i, g1) in glxs.iter().enumerate() {
        let ex_l_1 = empty_lines.iter().filter(|l| **l < g1.0).count() as i64;
        let ex_c_1 = empty_cols.iter().filter(|c| **c < g1.1).count() as i64;
        for g2 in glxs.iter().skip(i + 1) {
            let ex_l_2 = empty_lines.iter().filter(|l| **l < g2.0).count() as i64;
            let ex_c_2 = empty_cols.iter().filter(|c| **c < g2.1).count() as i64;
            let dist = i64::abs(g2.0 - g1.0 + (ex_l_2 - ex_l_1) * 999_999)
                + i64::abs(g2.1 - g1.1 + (ex_c_2 - ex_c_1) * 999_999);
            total += dist;
        }
    }

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
    const EXAMPLE_1: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

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
