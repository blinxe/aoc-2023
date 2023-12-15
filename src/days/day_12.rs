use std::iter::zip;

use crate::utils::input::read_input;

fn is_attempt_possible(attempt: &str, states: &str) -> bool {
    zip(attempt.chars(), states.chars()).all(|(ca, cs)| cs == '?' || cs == ca)
}

fn build_attempts(slots: u32, groups: &[u32]) -> Vec<String> {
    let block: String = "#".repeat(groups[0] as usize);

    let end_req: u32;
    // calculate how many slots must remain available
    if groups.len() > 1 {
        end_req = groups[1..groups.len()].iter().sum::<u32>() + groups.len() as u32 - 1;
    } else {
        end_req = 0;
    }
    // calculate how far the block can be added
    let max_offset = slots - end_req - groups[0];

    let mut attempts = Vec::new();

    for off in 0..=max_offset {
        let start = ".".repeat(off as usize) + block.as_str();
        if groups.len() > 1 {
            let ends: Vec<String> =
                build_attempts(slots - 1 - off - groups[0], &groups[1..groups.len()]);
            for e in ends {
                attempts.push(start.clone() + "." + e.as_str())
            }
        } else {
            attempts.push(start.clone() + ".".repeat((slots - off - groups[0]) as usize).as_str())
        }
    }

    attempts
}

fn solve_part_1(input: &str) {
    let records: Vec<(&str, Vec<u32>)> = input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(states, numbers)| {
            (
                states,
                numbers
                    .split(',')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            )
        })
        .collect();

    let total: usize = records
        .iter()
        .map(|(states, groups)| {
            build_attempts(states.chars().count() as u32, groups)
                .iter()
                .filter(|attempt| is_attempt_possible(attempt, states))
                .count()
        })
        .sum();

    println!("{:?}", total);
}

fn solve_part_2(input: &str) {
    let _records: Vec<(String, Vec<u32>)> = input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(states, numbers)| {
            let unfolded_states = vec![states]
                .into_iter()
                .cycle()
                .take(5)
                .collect::<Vec<&str>>()
                .join("?");
            let folded = numbers
                .split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let len = 5 * folded.len();
            let unfolded = folded.into_iter().cycle().take(len).collect();
            (unfolded_states, unfolded)
        })
        .collect();

    // let total: usize = records
    //     .iter()
    //     .map(|(states, groups)| {
    //         build_attempts(states.chars().count() as u32, groups)
    //             .iter()
    //             .filter(|attempt| is_attempt_possible(attempt, states))
    //             .count()
    //     })
    //     .sum();

    // println!("{:?}", total);
    println!("Too slow!");
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
    const EXAMPLE_1: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part_1() {
        super::solve_part_1(EXAMPLE_1);
    }

    const EXAMPLE_2: &str = "????.##?? 1,1,3";

    #[test]
    fn test_part_2() {
        super::solve_part_2(EXAMPLE_2);
    }
}

#[test]
fn testcmp() {
    let v = vec![1, 2, 1];
    let a = build_attempts(8, &v);
    println!("{:?}", a);
}
