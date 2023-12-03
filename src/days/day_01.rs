use crate::utils::input::read_input;

fn solve_part_1(input: &str) {
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

    println!("{}", total);
}

fn solve_part_2(input: &str) {
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
    const EXAMPLE_1: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    #[test]
    fn test_part_1() {
        super::solve_part_1(EXAMPLE_1);
    }

    const EXAMPLE_2: &str = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

    #[test]
    fn test_part_2() {
        super::solve_part_2(EXAMPLE_2);
    }
}
