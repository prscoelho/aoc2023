use crate::runner::Solve;

pub struct Day03;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Number {
    value: u32,
    start: usize,
    end: usize,
    row: usize,
}

fn is_symbol(ch: char) -> bool {
    !ch.is_ascii_digit() && ch != '.'
}

struct Engine {
    symbols: Vec<(usize, usize, char)>,
    numbers: Vec<Number>,
}

fn parse_engine(input: &str) -> Engine {
    let mut symbols = Vec::new();
    let mut numbers = Vec::new();

    for (row, line) in input.trim().lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if is_symbol(ch) {
                symbols.push((row, col, ch));
            }
        }

        let mut number = None;
        for (col, ch) in line.chars().enumerate() {
            match (number.take(), !ch.is_ascii_digit()) {
                (None, true) => {}
                (None, false) => {
                    number = Some(Number {
                        value: ch.to_digit(10).unwrap(),
                        start: col,
                        end: col,
                        row,
                    });
                }
                (Some(value), true) => numbers.push(value),
                (Some(mut value), false) => {
                    value.end += 1;
                    value.value = value.value * 10 + ch.to_digit(10).unwrap();

                    number = Some(value);
                }
            }
        }
        if let Some(number) = number {
            numbers.push(number);
        }
    }

    Engine { symbols, numbers }
}

fn is_adjacent(position: &(usize, usize, char), number: &Number) -> bool {
    (position.0 as isize - number.row as isize).abs() <= 1
        && position.1 >= number.start.saturating_sub(1)
        && position.1 <= number.end + 1
}

impl Solve<u32, u32> for Day03 {
    fn part1(input: &str) -> u32 {
        let engine = parse_engine(input);

        engine
            .numbers
            .into_iter()
            .filter(|number| {
                engine
                    .symbols
                    .iter()
                    .any(|symbol| is_adjacent(symbol, number))
            })
            .map(|number| number.value)
            .sum()
    }
    fn part2(input: &str) -> u32 {
        let engine = parse_engine(input);

        engine
            .symbols
            .into_iter()
            .filter(|symbol| symbol.2 == '*')
            .filter_map(|symbol| {
                let adjacent: Vec<_> = engine
                    .numbers
                    .iter()
                    .filter(|number| is_adjacent(&symbol, number))
                    .collect();

                if adjacent.len() == 2 {
                    Some(adjacent[0].value * adjacent[1].value)
                } else {
                    None
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

    #[test]
    fn example_p1() {
        let result = Day03::part1(EXAMPLE);
        let expected = 4361;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day03::part2(EXAMPLE);
        let expected = 467835;

        assert_eq!(result, expected);
    }
}
