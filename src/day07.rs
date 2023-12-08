use std::cmp::Ordering;

use crate::runner::Solve;

pub struct Day07;

struct Hand {
    hand: String,
    counts: [usize; 13],
    bid: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn get_type(&self) -> HandType {
        let counts: [usize; 5] = [
            self.counts.iter().filter(|&&v| v == 1).count(),
            self.counts.iter().filter(|&&v| v == 2).count(),
            self.counts.iter().filter(|&&v| v == 3).count(),
            self.counts.iter().filter(|&&v| v == 4).count(),
            self.counts.iter().filter(|&&v| v == 5).count(),
        ];

        Hand::calculate_hand(counts)
    }

    fn calculate_hand(counts: [usize; 5]) -> HandType {
        match counts {
            [_, _, _, _, 1] => HandType::FiveOfAKind,
            [_, _, _, 1, 0] => HandType::FourOfAKind,
            [_, 1, 1, 0, 0] => HandType::FullHouse,
            [_, _, 1, 0, 0] => HandType::ThreeOfAKind,
            [_, 2, 0, 0, 0] => HandType::TwoPair,
            [_, 1, 0, 0, 0] => HandType::OnePair,
            [_, 0, 0, 0, 0] => HandType::HighCard,
            _ => panic!("Unexpected hand {:?}", counts),
        }
    }

    fn get_type_wildcard(&self) -> HandType {
        let jokers = self.counts[9];

        let mut counts_without_joker = self.counts;
        counts_without_joker[9] = 0;

        *counts_without_joker.iter_mut().max().unwrap() += jokers;

        let counts: [usize; 5] = [
            counts_without_joker.iter().filter(|&&v| v == 1).count(),
            counts_without_joker.iter().filter(|&&v| v == 2).count(),
            counts_without_joker.iter().filter(|&&v| v == 3).count(),
            counts_without_joker.iter().filter(|&&v| v == 4).count(),
            counts_without_joker.iter().filter(|&&v| v == 5).count(),
        ];

        Hand::calculate_hand(counts)
    }
}

fn get_card_strength(ch: char) -> usize {
    match ch {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'J' => 9,
        'T' => 8,
        '9' => 7,
        '8' => 6,
        '7' => 5,
        '6' => 4,
        '5' => 3,
        '4' => 2,
        '3' => 1,
        '2' => 0,
        _ => panic!("Unexpected card"),
    }
}

fn get_card_strength_wildcard(ch: char) -> usize {
    if ch == 'J' {
        0
    } else {
        get_card_strength(ch) + 1
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_type().cmp(&other.get_type()).then_with(|| {
            let self_hand: Vec<_> = self.hand.chars().collect();
            let other_hand: Vec<_> = other.hand.chars().collect();
            for idx in 0..5 {
                let self_strength = get_card_strength(self_hand[idx]);
                let other_strength = get_card_strength(other_hand[idx]);

                let comparison = self_strength.cmp(&other_strength);
                if comparison != Ordering::Equal {
                    return comparison;
                }
            }

            Ordering::Equal
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Hand {}

fn cmp_wildcard(left: &Hand, right: &Hand) -> Ordering {
    left.get_type_wildcard()
        .cmp(&right.get_type_wildcard())
        .then_with(|| {
            let left_hand: Vec<_> = left.hand.chars().collect();
            let right_hand: Vec<_> = right.hand.chars().collect();
            for idx in 0..5 {
                let left_strength = get_card_strength_wildcard(left_hand[idx]);
                let right_strength = get_card_strength_wildcard(right_hand[idx]);

                let comparison = left_strength.cmp(&right_strength);
                if comparison != Ordering::Equal {
                    return comparison;
                }
            }

            Ordering::Equal
        })
}

const CARDS: &str = "23456789TJQKA";

fn parse_input(input: &str) -> Vec<Hand> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();

            let mut counts = [0; 13];

            for (idx, ch) in CARDS.chars().enumerate() {
                counts[idx] = hand.chars().filter(|&hand_ch| ch == hand_ch).count();
            }

            Hand {
                hand: String::from(hand),
                bid: bid.parse().unwrap(),
                counts,
            }
        })
        .collect()
}

impl Solve<u32, u32> for Day07 {
    fn part1(input: &str) -> u32 {
        let mut hands = parse_input(input);

        hands.sort();

        hands
            .into_iter()
            .enumerate()
            .map(|(idx, hand)| (idx as u32 + 1) * hand.bid)
            .sum()
    }
    fn part2(input: &str) -> u32 {
        let mut hands = parse_input(input);

        hands.sort_by(cmp_wildcard);

        hands
            .into_iter()
            .enumerate()
            .map(|(idx, hand)| (idx as u32 + 1) * hand.bid)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

    #[test]
    fn example_p1() {
        let result = Day07::part1(EXAMPLE);
        let expected = 6440;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day07::part2(EXAMPLE);
        let expected = 5905;

        assert_eq!(result, expected);
    }
}
