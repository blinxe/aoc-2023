use crate::utils::input::read_input;

fn parse_input(input: &str) -> Vec<(usize, (u32, u32, u32))> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(i, line)| (i + 1, line.split(": ").last().unwrap()))
        .map(|(i, draws)| {
            let draws = draws
                .split("; ")
                .map(|d| {
                    d.split(", ")
                        .map(|col| {
                            let mut spl = col.split(" ");
                            (
                                spl.next().unwrap().parse::<u32>().unwrap(),
                                spl.last().unwrap(),
                            )
                        })
                        .collect::<Vec<(u32, &str)>>()
                })
                .collect::<Vec<Vec<(u32, &str)>>>();
            (
                i,
                (
                    draws
                        .iter()
                        .map(|draw| {
                            draw.iter()
                                .filter_map(|(n, col)| if *col == "red" { Some(*n) } else { None })
                                .next()
                                .unwrap_or_default()
                        })
                        .max()
                        .unwrap_or_default(),
                    draws
                        .iter()
                        .map(|draw| {
                            draw.iter()
                                .filter_map(
                                    |(n, col)| if *col == "green" { Some(*n) } else { None },
                                )
                                .next()
                                .unwrap_or_default()
                        })
                        .max()
                        .unwrap_or_default(),
                    draws
                        .iter()
                        .map(|draw| {
                            draw.iter()
                                .filter_map(|(n, col)| if *col == "blue" { Some(*n) } else { None })
                                .next()
                                .unwrap_or_default()
                        })
                        .max()
                        .unwrap_or_default(),
                ),
            )
        })
        .collect()
}

fn solve_part_1(input: &str) {
    let data = parse_input(input);

    let (rmax, gmax, bmax) = (12u32, 13u32, 14u32);

    let answer: u32 = data
        .iter()
        .filter_map(|(i, (r, g, b))| {
            if *r <= rmax && *g <= gmax && *b <= bmax {
                Some(*i as u32)
            } else {
                None
            }
        })
        .sum();

    println!("{:?}", answer);
}

fn solve_part_2(input: &str) {
    let data = parse_input(input);

    let power: u32 = data.iter().map(|(_, (r, g, b))| r * g * b).sum();

    println!("{:?}", power);
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
    const EXAMPLE_1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

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
