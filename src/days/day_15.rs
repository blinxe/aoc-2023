use crate::utils::input::read_input;

fn hash(s: &str) -> u8 {
    s.bytes()
        .fold(0u8, |acc, b| acc.wrapping_add(b).wrapping_mul(17))
}

fn solve_part_1(input: &str) {
    let hsum: u32 = input.trim().split(',').map(|init| hash(init) as u32).sum();

    println!("{}", hsum);
}

fn get_box(init: &str) -> u8 {
    init.bytes()
        .take_while(|b| *b != b'-' && *b != b'=')
        .fold(0u8, |acc, b| acc.wrapping_add(b).wrapping_mul(17))
}

fn solve_part_2(input: &str) {
    let mut boxes: Vec<Vec<(&str, u8)>> = vec![vec![]; 256];
    let inits = input.trim().split(',').collect::<Vec<_>>();

    for init in inits {
        let b = &mut boxes[get_box(init) as usize];
        if init.ends_with('-') {
            let label = &init[0..init.len() - 1];
            if let Some(pos) = b.iter().position(|(lbl, _)| *lbl == label) {
                b.remove(pos);
            }
        } else {
            let label = &init[0..init.len() - 2];
            let foc = init.bytes().last().unwrap() - b'0';
            if let Some(pos) = b.iter().position(|(lbl, _)| *lbl == label) {
                b[pos].1 = foc;
            } else {
                b.push((label, foc));
            }
        }
    }

    let power: usize = boxes
        .iter()
        .enumerate()
        .map(|(bn, b)| {
            b.iter()
                .enumerate()
                .map(|(ln, (_, foc))| (bn + 1) * (ln + 1) * *foc as usize)
                .sum::<usize>()
        })
        .sum();

    println!("{}", power);
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
    const EXAMPLE_1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

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
