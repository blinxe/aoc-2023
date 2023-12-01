use crate::utils::input::read_input;

fn parse_input() -> String {
    let input = read_input(module_path!());

    input
}

fn get_total(input: String) -> u32 {
    let lines = input.lines();

    let mut total: u32 = 0;

    for line in lines {
        let mut first: u32 = 0;
        for c in line.as_bytes().iter() {
            if c.is_ascii_digit() {
                first = (10 * (*c - '0' as u8)) as u32;
                break;
            }
        }
        for c in line.as_bytes().iter().rev() {
            if c.is_ascii_digit() {
                first += (*c - '0' as u8) as u32;
                break;
            }
        }

        total += first;
    }

    return total;
}

fn get_total_2(input: String) -> u32 {
    let lines = input.lines();

    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut total: u32 = 0;

    for line in lines {
        let mut first: u32 = 0;
        for (i, c) in line.as_bytes().iter().enumerate() {
            if c.is_ascii_digit() {
                first = (10 * (*c - '0' as u8)) as u32;
                break;
            } else {
                let mut found = false;
                let sub = &line[i..];
                for (j, w) in words.iter().enumerate() {
                    if sub.starts_with(w) {
                        first = (10 * j) as u32;
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
        }
        for (i, c) in line.as_bytes().iter().enumerate().rev() {
            if c.is_ascii_digit() {
                first += (*c - '0' as u8) as u32;
                break;
            } else {
                let mut found = false;
                let sub = &line[i..];
                for (j, w) in words.iter().enumerate() {
                    if sub.starts_with(w) {
                        first += j as u32;
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
        }

        total += first;
    }

    return total;
}

pub fn part_1() {
    let input = parse_input();

    let total = get_total(input);
    println!("{}", total);
}

pub fn part_2() {
    let input = parse_input();

    let total = get_total_2(input);
    println!("{}", total);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_part_1() {
        super::part_1();
    }

    #[test]
    fn test_part_2() {
        super::part_2();
    }
}

#[cfg(test)]
fn pro_solution(input: &str, is_part_2: bool) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            if is_part_2 {
                line.to_string()
                    .replace("one", "one1one")
                    .replace("two", "two2two")
                    .replace("three", "three3three")
                    .replace("four", "four4four")
                    .replace("five", "five5five")
                    .replace("six", "six6six")
                    .replace("seven", "seven7seven")
                    .replace("eight", "eight8eight")
                    .replace("nine", "nine9nine")
            } else {
                line.to_string()
            }
        })
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum()
}
