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

fn apply_map_range(range: &Range<u64>, m: &Mapping) -> (Option<Range<u64>>, Vec<Range<u64>>) {
    let (rs, re) = (range.start, range.end);
    let (ms, me) = (m[1], m[1] + m[2]);

    let remapped;

    let mut unchanged = Vec::new();

    if me <= rs || ms >= re {
        // no overlap
        // range:                |------|
        // mapping:    |------|     OR     |------|
        remapped = None;
        unchanged.push(rs..re);
    } else {
        // range:           |--
        // mapping:    |--
        let rms = rs.max(ms);
        // range:      --|
        // mapping:         --|
        let rme = re.min(me);
        if rme < rms {
            remapped = None;
        } else {
            remapped = Some(rms - ms + m[0]..rme - ms + m[0]);
        }

        if rs < rms {
            unchanged.push(rs..rms)
        }
        if re > rme {
            unchanged.push(rme..re)
        }
    }

    (remapped, unchanged)
}

fn apply_map_ranges(ranges: Vec<Range<u64>>, m: &Mapping) -> (Vec<Range<u64>>, Vec<Range<u64>>) {
    let mut unchanged = Vec::new();
    let mut remapped = Vec::new();
    for r in ranges {
        let (new_remapped, mut new_unchanged) = apply_map_range(&r, m);
        //println!("{:?} -> {:?} / {:?}", r, new_remapped, new_unchanged);
        unchanged.append(&mut new_unchanged);
        if let Some(rm) = new_remapped {
            remapped.push(rm);
        }
    }

    (unchanged, remapped)
}

fn apply_transform_ranges(ranges: Vec<Range<u64>>, t: &Transform) -> Vec<Range<u64>> {
    let mut unchanged = ranges;
    let mut remapped = Vec::new();
    for m in t {
        let mut new;
        (unchanged, new) = apply_map_ranges(unchanged, m);
        remapped.append(&mut new);
    }

    remapped.extend(unchanged.into_iter().filter(|r| r.start != r.end));
    remapped
}

fn solve_part_2(input: &str) {
    let (seeds, process) = parse_input(input);

    let mut ranges = seeds
        .chunks(2)
        .map(|pair| pair[0]..pair[0] + pair[1])
        .collect::<Vec<Range<u64>>>();

    for t in process {
        ranges = apply_transform_ranges(ranges, &t);
    }

    let min = ranges.iter().min_by_key(|r| r.start).unwrap();

    println!("{}", min.start);
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
