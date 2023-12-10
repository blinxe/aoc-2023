use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Neg},
};

use crate::utils::input::read_input;

type Grid = Vec<Vec<char>>;
// type Coord = (i32, i32);
// type Dir = (i32, i32);

#[derive(Debug, Copy, PartialEq, Eq)]
struct Dir {
    h: i32,
    v: i32,
}
impl Neg for Dir {
    type Output = Dir;

    fn neg(self) -> Self::Output {
        Dir {
            h: -self.h,
            v: -self.v,
        }
    }
}
impl Clone for Dir {
    fn clone(&self) -> Self {
        Self {
            h: self.h,
            v: self.v,
        }
    }
}

#[derive(Debug, Copy, PartialEq, Eq, Hash)]
struct Coord {
    l: i32,
    c: i32,
}
impl Add<Dir> for Coord {
    type Output = Coord;

    fn add(self, rhs: Dir) -> Self::Output {
        Coord {
            l: self.l + rhs.v,
            c: self.c + rhs.h,
        }
    }
}
impl AddAssign<Dir> for Coord {
    fn add_assign(&mut self, rhs: Dir) {
        *self = *self + rhs;
    }
}
impl Clone for Coord {
    fn clone(&self) -> Self {
        Self {
            l: self.l,
            c: self.c,
        }
    }
}

const N: Dir = Dir { h: 0, v: -1 };
const S: Dir = Dir { h: 0, v: 1 };
const W: Dir = Dir { h: -1, v: 0 };
const E: Dir = Dir { h: 1, v: 0 };

fn id_pipe(p: char) -> Option<(Dir, Dir)> {
    match p {
        '|' => Some((N, S)),
        '-' => Some((W, E)),
        'L' => Some((N, E)),
        'J' => Some((N, W)),
        '7' => Some((W, S)),
        'F' => Some((E, S)),
        _ => None,
    }
}

fn parse_input(input: &str) -> Grid {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn find_start(grid: &Grid) -> Coord {
    for (ln, line) in grid.iter().enumerate() {
        for (cn, chr) in line.iter().enumerate() {
            if *chr == 'S' {
                return Coord {
                    l: ln as i32,
                    c: cn as i32,
                };
            }
        }
    }
    unreachable!();
}

fn first_dir(grid: &Grid, pos: Coord) -> Dir {
    for d in [N, S, E, W] {
        let neighbor = pos + d;
        if neighbor.l < 0
            || neighbor.l >= grid.len() as i32
            || neighbor.c < 0
            || neighbor.c >= grid[0].len() as i32
        {
            continue;
        }
        if let Some(pipe) = id_pipe(grid[neighbor.l as usize][neighbor.c as usize]) {
            if pipe.0 == -d || pipe.1 == -d {
                return d;
            }
        }
    }

    unreachable!();
}

fn next_dir(grid: &Grid, pos: Coord, from: Dir) -> Dir {
    let pipe = id_pipe(grid[pos.l as usize][pos.c as usize]).unwrap();
    let new_dir;
    if pipe.0 == from {
        new_dir = pipe.1;
    } else {
        new_dir = pipe.0;
    };

    new_dir
}

fn solve_part_1(input: &str) {
    let grid = parse_input(input);
    let spos = find_start(&grid);

    let mut dir = first_dir(&grid, spos);
    let mut pos = spos + dir;
    let mut steps = 1;
    while pos != spos {
        dir = next_dir(&grid, pos, -dir);
        pos += dir;
        steps += 1;
    }

    println!("{}", steps / 2);
}

fn find_s_substitute(grid: &Grid, pos: Coord) -> char {
    let mut dirs = Vec::with_capacity(2);
    for d in [N, S, E, W] {
        let neighbor = pos + d;
        if neighbor.l < 0
            || neighbor.l >= grid.len() as i32
            || neighbor.c < 0
            || neighbor.c >= grid[0].len() as i32
        {
            continue;
        }
        if let Some(pipe) = id_pipe(grid[neighbor.l as usize][neighbor.c as usize]) {
            if pipe.0 == -d || pipe.1 == -d {
                dirs.push(d);
            }
        }
    }

    for c in "|-LJ7F".chars() {
        let pipe = id_pipe(c).unwrap();
        if (pipe.0 == dirs[0] || pipe.0 == dirs[1]) && (pipe.1 == dirs[0] || pipe.1 == dirs[1]) {
            return c;
        }
    }

    unreachable!();
}

fn get_v_component(c: char) -> i32 {
    match c {
        '|' => 2,
        'L' => 1,
        'J' => -1,
        '7' => 1,
        'F' => -1,
        _ => 0,
    }
}

fn solve_part_2(input: &str) {
    let mut grid = parse_input(input);
    let spos = find_start(&grid);
    let c = find_s_substitute(&grid, spos);
    grid[spos.l as usize][spos.c as usize] = c;

    let mut dir = first_dir(&grid, spos);
    let mut pos = spos + dir;
    let mut path_cells = HashSet::<Coord>::new();
    path_cells.insert(spos);
    while pos != spos {
        path_cells.insert(pos);
        dir = next_dir(&grid, pos, -dir);
        pos += dir;
    }

    let mut nest_size = 0;
    let mut in_nest = false;
    let mut v_crosses = 0;
    for (ln, l) in grid.iter().enumerate() {
        for (lc, _) in l.iter().enumerate() {
            let pos = Coord {
                l: ln as i32,
                c: lc as i32,
            };
            if path_cells.contains(&pos) {
                let c = grid[pos.l as usize][pos.c as usize];
                // print!("{}", c);
                v_crosses += get_v_component(c);
                if v_crosses == 2 || v_crosses == -2 {
                    in_nest = !in_nest;
                    v_crosses = 0;
                }
            } else if in_nest {
                // print!("I");
                nest_size += 1;
            } else {
                // print!("O");
            }
        }
        // println!();
    }

    println!("{}", nest_size);
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
    const EXAMPLE_1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    #[test]
    fn test_part_1() {
        super::solve_part_1(EXAMPLE_1);
    }

    const EXAMPLE_2: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_part_2() {
        super::solve_part_2(EXAMPLE_2);
    }
}
