use std::collections::HashMap;

use crate::utils::input::read_input;

type Grid = Vec<Vec<char>>;

fn parse(input: &str) -> Grid {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn _display(g: &Grid) {
    for r in g {
        for c in r {
            print!("{}", *c);
        }
        println!();
    }
    println!();
}

fn stack_up(g: &Grid) -> Grid {
    let mut new = vec![vec!['.'; g[0].len()]; g.len()];
    for c in 0..g[0].len() {
        let mut top = 0;
        for r in 0..g.len() {
            match g[r][c] {
                'O' => {
                    new[r][c] = '.';
                    new[top][c] = 'O';
                    top += 1;
                }
                '#' => {
                    new[r][c] = '#';
                    top = r + 1;
                }
                _ => (),
            }
        }
    }

    new
}

fn weigh(g: &Grid) -> usize {
    g.iter()
        .enumerate()
        .map(|(rn, row)| {
            row.iter()
                .filter_map(|c| if *c == 'O' { Some(g.len() - rn) } else { None })
                .sum::<usize>()
        })
        .sum()
}

fn solve_part_1(input: &str) {
    let mut grid = parse(input);
    // _display(&grid);
    grid = stack_up(&grid);
    // _display(&grid);
    println!("{}", weigh(&grid));
}

fn rotate_right(g: &Grid) -> Grid {
    (0..g[0].len())
        .map(|cn| g.iter().rev().map(|row| row[cn]).collect())
        .collect()
}

fn step(g: &Grid) -> Grid {
    let mut new = g.clone();
    for _ in 0..4 {
        new = stack_up(&new);
        new = rotate_right(&new);
    }
    // _display(&grid);

    new
}

fn solve_part_2(input: &str) {
    let mut grid = parse(input);

    let mut seen = HashMap::new();

    let mut cycle_start = 0;
    let mut i = 0;
    while i < 1_000_000_000 {
        i += 1;
        grid = step(&grid);
        if let Some(already_there) = seen.insert(grid.clone(), i) {
            cycle_start = already_there;
            break;
        }
    }

    let cycle_len = i - cycle_start;
    let final_step = cycle_start + (1_000_000_000 - i) % cycle_len;

    let final_grid = seen
        .iter()
        .find(|(_, step)| **step == final_step)
        .unwrap()
        .0;

    println!("{}", weigh(final_grid));
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
    const EXAMPLE_1: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

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
