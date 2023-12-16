use std::collections::VecDeque;

use crate::utils::input::read_input;

type Grid = Vec<Vec<char>>;
type DirFlags = [bool; 4];
type Seen = Vec<Vec<DirFlags>>;
type Coord = (i32, i32);
type Dir = (i32, i32);

const UP: Dir = (-1, 0);
const DOWN: Dir = (1, 0);
const LEFT: Dir = (0, -1);
const RIGHT: Dir = (0, 1);

fn parse(input: &str) -> Grid {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn dir_id(d: (i32, i32)) -> usize {
    match d {
        UP => 0,
        DOWN => 1,
        LEFT => 2,
        RIGHT => 3,
        _ => unreachable!(),
    }
}

fn get_next_dirs(c: char, d: Dir) -> Vec<Dir> {
    let mut out = Vec::new();

    match c {
        '.' => out.push(d),
        '/' => out.push(match d {
            UP => RIGHT,
            RIGHT => UP,
            DOWN => LEFT,
            LEFT => DOWN,
            _ => unreachable!(),
        }),
        '\\' => out.push(match d {
            UP => LEFT,
            LEFT => UP,
            DOWN => RIGHT,
            RIGHT => DOWN,
            _ => unreachable!(),
        }),
        '-' => match d {
            UP | DOWN => {
                out.push(LEFT);
                out.push(RIGHT);
            }
            _ => out.push(d),
        },
        '|' => match d {
            LEFT | RIGHT => {
                out.push(UP);
                out.push(DOWN);
            }
            _ => out.push(d),
        },
        _ => unreachable!(),
    }

    out
}

fn is_in_grid(g: &Grid, c: Coord) -> bool {
    c.0 >= 0 && c.0 < g[0].len() as i32 && c.1 >= 0 && c.1 < g.len() as i32
}

fn traverse(g: &Grid, seen: &mut Seen, start_pos: Coord, start_dir: Dir) {
    let mut queue = VecDeque::from([(start_pos, start_dir)]);
    while let Some((pos, dir)) = queue.pop_front() {
        seen[pos.0 as usize][pos.1 as usize][dir_id(dir)] = true;
        for d in get_next_dirs(g[pos.0 as usize][pos.1 as usize], dir) {
            let next = (pos.0 + d.0, pos.1 + d.1);
            if is_in_grid(&g, next) && !seen[next.0 as usize][next.1 as usize][dir_id(d)] {
                queue.push_back((next, d));
            }
        }
    }
}

fn count_energized(seen: &Seen) -> usize {
    seen.iter()
        .map(|row| row.iter().filter(|dirs| dirs.contains(&true)).count())
        .sum()
}

fn solve_part_1(input: &str) {
    let g = parse(input);
    let mut seen = vec![vec![[false, false, false, false]; g[0].len()]; g.len()];

    traverse(&g, &mut seen, (0, 0), (0, 1));
    let total = count_energized(&seen);

    println!("{}", total);
}

fn solve_part_2(input: &str) {
    let g = parse(input);
    let mut max = 0;

    for r in 0..g.len() {
        let tmpg = g.clone();
        let mut seen = vec![vec![[false, false, false, false]; g[0].len()]; g.len()];
        traverse(&tmpg, &mut seen, (r as i32, 0), (0, 1));
        max = max.max(count_energized(&seen));

        let tmpg = g.clone();
        let mut seen = vec![vec![[false, false, false, false]; g[0].len()]; g.len()];
        traverse(&tmpg, &mut seen, (r as i32, g[0].len() as i32 - 1), (0, -1));
        max = max.max(count_energized(&seen));
    }

    for c in 0..g[0].len() {
        let tmpg = g.clone();
        let mut seen = vec![vec![[false, false, false, false]; g[0].len()]; g.len()];
        traverse(&tmpg, &mut seen, (0, c as i32), (1, 0));
        max = max.max(count_energized(&seen));

        let tmpg = g.clone();
        let mut seen = vec![vec![[false, false, false, false]; g[0].len()]; g.len()];
        traverse(&tmpg, &mut seen, (g.len() as i32 - 1, c as i32), (-1, 0));
        max = max.max(count_energized(&seen));
    }

    println!("{}", max);
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
    const EXAMPLE_1: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

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
