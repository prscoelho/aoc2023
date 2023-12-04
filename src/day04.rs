use std::collections::HashSet;

use crate::runner::Solve;

pub struct Day04;

#[derive(Debug)]
struct Card {
    left: HashSet<i32>,
    right: HashSet<i32>,
}

fn parse_numbers(text: &str) -> HashSet<i32> {
    text.split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_card(line: &str) -> Card {
    let (_, rest) = line.split_once(": ").unwrap();

    let (left, right) = rest.split_once(" | ").unwrap();

    Card {
        left: parse_numbers(left),
        right: parse_numbers(right),
    }
}

fn calculate_winning_numbers(card: &Card) -> usize {
    card.right.intersection(&card.left).count()
}

impl Solve<i32, i32> for Day04 {
    fn part1(input: &str) -> i32 {
        let cards: Vec<_> = input.trim().lines().map(parse_card).collect();

        cards
            .iter()
            .map(calculate_winning_numbers)
            .filter(|&matches| matches > 0)
            .map(|matches| 2i32.pow(matches.saturating_sub(1) as u32))
            .sum()
    }
    fn part2(input: &str) -> i32 {
        let cards: Vec<_> = input.trim().lines().map(parse_card).collect();

        let matching_count: Vec<_> = cards.iter().map(calculate_winning_numbers).collect();

        let mut card_count = vec![1; cards.len()];

        for (idx, count) in matching_count.into_iter().enumerate() {
            let times = card_count[idx];

            for j in 0..count {
                card_count[idx + j + 1] += times;
            }
        }

        card_count.into_iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    #[test]
    fn example_p1() {
        let result = Day04::part1(EXAMPLE);
        let expected = 13;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day04::part2(EXAMPLE);
        let expected = 30;

        assert_eq!(result, expected);
    }
}
