use std::collections::HashMap;

use crate::runner::Solve;

pub struct Day08;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Maps {
    instructions: Vec<Instruction>,
    network: HashMap<String, (String, String)>,
}

fn parse_maps(input: &str) -> Maps {
    let (instructions, network) = input.trim().split_once("\n\n").unwrap();

    let instructions = instructions
        .trim()
        .chars()
        .map(|ch| {
            if ch == 'L' {
                Instruction::Left
            } else {
                Instruction::Right
            }
        })
        .collect();

    let network = network
        .trim()
        .split('\n')
        .map(|line| {
            let (node, leftright) = line.split_once(" = ").unwrap();
            let node = node.to_string();

            let (left, right) = leftright.split_once(", ").unwrap();
            let left = left.strip_prefix('(').unwrap().to_string();
            let right = right.strip_suffix(')').unwrap().to_string();

            (node, (left, right))
        })
        .collect();

    Maps {
        instructions,
        network,
    }
}

fn cost_to_finish(maps: &Maps, start: &str, goal: impl Fn(&str) -> bool) -> usize {
    let mut current = start.to_string();
    let mut instruction = maps.instructions.iter().cycle().enumerate();
    loop {
        let (step, instruction) = instruction.next().unwrap();
        if goal(&current) {
            return step;
        }
        let node = maps.network.get(&current).unwrap();
        if instruction == &Instruction::Left {
            current = node.0.clone();
        } else {
            current = node.1.clone();
        }
    }
}

fn gcd(x: usize, y: usize) -> usize {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

impl Solve<usize, usize> for Day08 {
    fn part1(input: &str) -> usize {
        let maps = parse_maps(input);
        cost_to_finish(&maps, "AAA", |s| s == "ZZZ")
    }
    fn part2(input: &str) -> usize {
        let maps = parse_maps(input);

        let starts: Vec<_> = maps
            .network
            .keys()
            .filter(|s| s.ends_with('A'))
            .cloned()
            .collect();

        let costs: Vec<usize> = starts
            .iter()
            .map(|s| cost_to_finish(&maps, s, |s| s.ends_with('Z')))
            .collect();

        costs.into_iter().reduce(lcm).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

    const EXAMPLE2: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;

    #[test]
    fn parses_example() {
        // todo
    }

    #[test]
    fn example_p1() {
        let result = Day08::part1(EXAMPLE);
        let expected = 6;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day08::part2(EXAMPLE2);
        let expected = 6;

        assert_eq!(result, expected);
    }
}
