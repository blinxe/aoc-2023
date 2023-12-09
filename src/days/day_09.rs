use crate::utils::input::read_input;

type Suite = Vec<i32>;

fn parse_input(input: &str) -> Vec<Suite> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .split(' ')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Suite>()
        })
        .collect()
}

fn get_differences(s: Suite) -> Suite {
    let mut result = Suite::new();

    for (idx, _) in s.iter().enumerate() {
        if idx < s.len() - 1 {
            result.push(s[idx + 1] - s[idx]);
        }
    }
    result
}

fn solve_part_1(_input: &str) {
    let suites = parse_input(_input);
    let mut count: i32 = 0;

    for s in suites {
        let mut diffs: Vec<Suite> = Vec::new();

        diffs.push(s.clone());

        let mut diff = get_differences(s.clone());

        while diff.iter().any(|n| *n != 0) {
            diffs.push(diff.clone());
            diff = get_differences(diff.clone());
        }

        while !diffs.is_empty() {
            let last_nb = diff.last().unwrap().clone();

            diff = diffs.pop().unwrap();
            diff.push(diff.last().unwrap() + last_nb);
        }

        count += diff.last().unwrap();
    }
    println!("{}", count);
}

fn solve_part_2(_input: &str) {
    let suites = parse_input(_input);
    let mut count: i32 = 0;

    for s in suites {
        let mut diffs: Vec<Suite> = Vec::new();

        diffs.push(s.clone());

        let mut diff = get_differences(s.clone());

        while diff.iter().any(|n| *n != 0) {
            diffs.push(diff.clone());
            diff = get_differences(diff.clone());
        }

        while !diffs.is_empty() {
            let first_nb = diff.first().unwrap().clone();

            diff = diffs.pop().unwrap();
            diff.insert(0, diff.first().unwrap() - first_nb);
        }

        count += diff.first().unwrap();
    }
    println!("{}", count);
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
    const EXAMPLE_1: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

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
