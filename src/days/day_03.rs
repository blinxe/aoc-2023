use std::collections::HashMap;

use crate::utils::input::read_input;

fn has_neighboring_symbol(lines: &Vec<&str>, ln: usize, pos: usize, len: usize) -> bool {
    lines
        .iter()
        .skip(if ln > 0 { ln - 1 } else { 0 })
        .take(if ln > 0 { 3 } else { 2 })
        .any(|l| {
            l.chars()
                .skip(if pos > 0 { pos - 1 } else { 0 })
                .take(if pos > 0 { len + 2 } else { len + 1 })
                .any(|c| c != '.' && !c.is_digit(10))
        })
}

fn solve_part_1(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0;

    for (ln, l) in lines.iter().enumerate() {
        let mut it = l.char_indices();
        while let Some((pos, c)) = it.next() {
            if !c.is_digit(10) {
                continue;
            };
            let mut n = c.to_digit(10).unwrap();
            let mut len: usize = 1;
            while let Some((_, c)) = it.next() {
                if !c.is_digit(10) {
                    break;
                };
                n = 10 * n + c.to_digit(10).unwrap();
                len += 1;
            }
            if has_neighboring_symbol(&lines, ln, pos, len) {
                total += n;
            };
        }
    }

    println!("{}", total);
}

fn get_neighboring_gears(
    lines: &Vec<&str>,
    ln: usize,
    pos: usize,
    len: usize,
) -> Vec<(usize, usize)> {
    lines
        .iter()
        .enumerate()
        .skip(if ln > 0 { ln - 1 } else { 0 })
        .take(if ln > 0 { 3 } else { 2 })
        .map(|(ln, l)| {
            l.char_indices()
                .skip(if pos > 0 { pos - 1 } else { 0 })
                .take(if pos > 0 { len + 2 } else { len + 1 })
                .filter(|(_, c)| *c == '*')
                .map(|(cn, _)| (ln, cn))
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect()
}

fn solve_part_2(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for (ln, l) in lines.iter().enumerate() {
        let mut it = l.char_indices();
        while let Some((pos, c)) = it.next() {
            if !c.is_digit(10) {
                continue;
            };
            let mut n = c.to_digit(10).unwrap();
            let mut len: usize = 1;
            while let Some((_, c)) = it.next() {
                if !c.is_digit(10) {
                    break;
                };
                n = 10 * n + c.to_digit(10).unwrap();
                len += 1;
            }
            let mygears = get_neighboring_gears(&lines, ln, pos, len);
            for g in mygears {
                if gears.contains_key(&g) {
                    gears.get_mut(&g).unwrap().push(n);
                } else {
                    gears.insert(g, Vec::from([n]));
                };
            }
        }
    }

    let total: u32 = gears
        .iter()
        .filter_map(|(_, v)| {
            if v.len() == 2 {
                Some(v.iter().product::<u32>())
            } else {
                None
            }
        })
        .sum();

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
    const EXAMPLE_1: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

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
