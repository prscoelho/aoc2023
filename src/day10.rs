use std::collections::{BTreeMap, BTreeSet};

use crate::runner::Solve;

pub struct Day10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    row: i32,
    col: i32,
}

enum Pipe {
    Vertical,
    Horizontal,
    NorthToEast, // 'L'
    NorthToWest, // 'J'
    SouthToWest, // '7'
    SouthToEast, // 'F'
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        use Pipe::*;
        match value {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => NorthToEast,
            'J' => NorthToWest,
            '7' => SouthToWest,
            'F' => SouthToEast,
            _ => panic!("Unexpected pipe char {}", value),
        }
    }
}

fn add_positions(p1: Position, p2: Position) -> Position {
    Position {
        row: p1.row + p2.row,
        col: p1.col + p2.col,
    }
}
fn sub_positions(p1: Position, p2: Position) -> Position {
    Position {
        row: p1.row - p2.row,
        col: p1.col - p2.col,
    }
}

// to - from = x

impl Pipe {
    fn accessible(&self, from: Position, to: Position) -> bool {
        let diff = sub_positions(to, from);
    }
}

fn calculate_start_pipe(connections: &BTreeMap<Position, Pipe>, position: Position) -> Pipe {
    let top_position = add_positions(position, Position { row: -1, col: 0 });
    let bot_position = add_positions(position, Position { row: 1, col: 0 });
    let left_position = add_positions(position, Position { row: 0, col: -1 });
    let right_position = add_positions(position, Position { row: 0, col: 1 });

    let top_pipe = connections.get(&top_position);
    let bot_pipe = connections.get(&bot_position);
    let left_pipe = connections.get(&left_position);
    let right_pipe = connections.get(&right_position);
}

struct Grid {
    connections: BTreeMap<Position, Pipe>,
    start: Position,
}

fn parse_input(input: &str) -> Grid {
    let mut start: Option<Position> = None;
    let mut connections = BTreeMap::new();
    for (row, line) in input.trim().lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let position = Position {
                col: col as i32,
                row: row as i32,
            };
            match ch {
                'S' => start = Some(position),
                '.' => {}
                _ => {
                    connections.insert(position, ch.into());
                }
            };
        }
    }

    // find what type of pipe the start node is

    Grid {
        start: start.unwrap(),
        connections,
    }
}

impl Solve<i32, i32> for Day10 {
    fn part1(input: &str) -> i32 {
        0
    }
    fn part2(input: &str) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_EXAMPLE: &str = r#".....
.S-7.
.|.|.
.L-J.
.....
"#;

    const COMPLEX_EXAMPLE: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;

    #[test]
    fn simple_example_p1() {
        let result = Day10::part1(SIMPLE_EXAMPLE);
        let expected = 4;

        assert_eq!(result, expected);
    }

    #[test]
    fn complex_example_p1_() {
        let result = Day10::part1(COMPLEX_EXAMPLE);
        let expected = 8;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day10::part2(SIMPLE_EXAMPLE);
        let expected = 0;

        assert_eq!(result, expected);
    }
}
