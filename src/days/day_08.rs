use crate::utils::input::read_input;

#[derive(Debug)]
struct Node {
    name: Vec<char>,
    left: Vec<char>,
    right: Vec<char>,
}
#[derive(Debug)]
struct Content {
    navigation: Vec<char>,
    nodes: Vec<Node>,
}

fn parse_input(input: &str) -> Content {
    let nav = input.split("\n\n").next().unwrap().chars().collect();

    let mut nodes: Vec<Node> = Vec::new();

    for l in input.lines().skip(2) {
        let node = Node {
            name: l.split("=").next().unwrap().trim().chars().collect(),
            left: l
                .split(|c| c == '(' || c == ',')
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .chars()
                .collect(),
            right: l
                .split(|c| c == ')' || c == ',')
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .chars()
                .collect(),
        };
        nodes.push(node);
    }

    let content = Content {
        navigation: nav,
        nodes: nodes,
    };

    content
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn lcm(a: usize, b: usize) -> usize {
    // LCM = a*b / gcd
    return a * (b / gcd(a, b));
}

fn solve_part_1(input: &str) {
    let content = parse_input(input);

    const START: &[char] = &['A', 'A', 'A'];
    const FINISH: &[char] = &['Z', 'Z', 'Z'];

    let mut count: u32 = 0;
    let mut current = content.nodes.iter().find(|n| n.name == START).unwrap();
    let mut it = content.navigation.iter();

    while current.name != FINISH {
        let mut dir = it.next().unwrap_or(&'Z');

        if *dir == 'Z' {
            it = content.navigation.iter();
            dir = it.next().unwrap();
        }

        if *dir == 'L' {
            current = content
                .nodes
                .iter()
                .find(|e| e.name == current.left)
                .unwrap();
        } else {
            current = content
                .nodes
                .iter()
                .find(|e| e.name == current.right)
                .unwrap();
        }
        count += 1;
    }

    println!("{}", count);
}

fn solve_part_2(input: &str) {
    let content = parse_input(input);

    const START: &[char] = &['A', 'A', 'A'];
    const FINISH: &[char] = &['Z', 'Z', 'Z'];

    let currents: Vec<&Node> = content
        .nodes
        .iter()
        .filter(|n| n.name.last() == START.last())
        .collect();

    let mut counts: Vec<usize> = Vec::new();

    for (idx, mut current) in currents.into_iter().enumerate() {
        counts.push(0);
        let mut it = content.navigation.iter();

        while current.name.last() != FINISH.last() {
            let mut dir = it.next().unwrap_or(&'Z');

            if *dir == 'Z' {
                it = content.navigation.iter();
                dir = it.next().unwrap();
            }

            if *dir == 'L' {
                current = content
                    .nodes
                    .iter()
                    .find(|e| e.name == current.left)
                    .unwrap();
            } else {
                current = content
                    .nodes
                    .iter()
                    .find(|e| e.name == current.right)
                    .unwrap();
            }
            counts[idx] += 1;
        }
    }

    let mut result = 1;
    for c in &counts {
        result = lcm(result, *c);
    }
    println!("{}", result);
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
    const EXAMPLE_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part_1() {
        super::solve_part_1(EXAMPLE_1);
    }

    const EXAMPLE_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part_2() {
        super::solve_part_2(EXAMPLE_2);
    }
}
