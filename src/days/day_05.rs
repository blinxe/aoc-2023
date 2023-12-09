use std::ops::Range;

use crate::utils::input::read_input;

type Mapping = Vec<u64>;
type Transform = Vec<Mapping>;
type Process = Vec<Transform>;

fn parse_input(input: &str) -> (Vec<u64>, Process) {
    let seeds = input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let process = input
        .split("\n\n")
        .skip(1)
        .map(|stage| {
            stage
                .lines()
                .skip(1)
                .map(|mapping| {
                    mapping
                        .split(' ')
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    (seeds, process)
}

fn apply_transform(item: u64, mappings: &Transform) -> u64 {
    for m in mappings {
        if (m[1]..m[1] + m[2]).contains(&item) {
            return m[0] + item - m[1];
        }
    }
    return item;
}

fn apply_process(seed: u64, transforms: &Process) -> u64 {
    let mut out = seed;
    for t in transforms {
        out = apply_transform(out, t);
    }
    out
}

fn solve_part_1(input: &str) {
    let (seeds, process) = parse_input(input);

    let min = seeds
        .iter()
        .map(|s| apply_process(*s, &process))
        .min()
        .unwrap();

    println!("{:?}", min);
}

fn apply_transform_single_range(range: Range<u64>, mappings: &Transform) -> Vec<Range<u64>> {
    let mut v = Vec::new();
    for m in mappings {
        let (rs, re) = (range.start, range.end);
        let (ms, me) = (m[1], m[1] + m[2]);
        if re < ms || me < rs {
            continue;
        }
    }

    v
}

fn apply_transform_ranges(ranges: &Vec<Range<u64>>, mappings: &Transform) -> Vec<Range<u64>> {
    ranges
        .iter()
        .flat_map(|r| apply_transform_single_range(r.clone(), mappings))
        .collect()
}

fn apply_process_range(range: Range<u64>, transforms: &Process) -> Vec<Range<u64>> {
    let mut out = vec![range];
    for t in transforms {
        out = apply_transform_ranges(&out, t);
    }

    out
}

fn solve_part_2(input: &str) {
    let (seeds, process) = parse_input(input);

    // let min = seeds
    //     .chunks(2)
    //     .map(|pair| pair[0]..pair[0] + pair[1])
    //     .flat_map(|r| apply_process_range(r, &process))
    //     .min_by_key(|r| r.start)
    //     .unwrap();

    println!("TODO");
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
    const EXAMPLE_1: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_1() {
        super::solve_part_1(EXAMPLE_1);
        let r = 0..10u64;
        let s = std::mem::size_of_val(&r);
        println!("{}", s)
    }

    const EXAMPLE_2: &str = EXAMPLE_1;

    #[test]
    fn test_part_2() {
        super::solve_part_2(EXAMPLE_2);
    }
}
