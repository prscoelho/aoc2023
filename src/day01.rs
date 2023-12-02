use crate::runner::Solve;

fn calibration_value(text: &str) -> u32 {
    let it = text
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap());

    it.clone().next().unwrap() * 10 + it.clone().next_back().unwrap()
}

const TEXT_NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_digit(text: &str) -> Option<u32> {
    let first_char = text.chars().next().unwrap();

    if first_char.is_ascii_digit() {
        return first_char.to_digit(10);
    } else {
        for (idx, text_number) in TEXT_NUMBERS.iter().enumerate() {
            if text.starts_with(text_number) {
                return idx.try_into().ok();
            }
        }
    }
    None
}

fn calibration_value_part2(text: &str) -> u32 {
    let mut numbers = Vec::new();

    for idx in 0..text.len() {
        if let Some(number) = parse_digit(&text[idx..]) {
            numbers.push(number);
        }
    }

    numbers.first().unwrap() * 10 + numbers.last().unwrap()
}

pub struct Day01;

impl Solve<u32, u32> for Day01 {
    fn part1(input: &str) -> u32 {
        input.trim().split('\n').map(calibration_value).sum()
    }
    fn part2(input: &str) -> u32 {
        input.trim().split('\n').map(calibration_value_part2).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;
    const EXAMPLE2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

    #[test]
    fn example_p1() {
        let result = Day01::part1(EXAMPLE1);
        let expected = 142;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day01::part2(EXAMPLE2);
        let expected = 281;

        assert_eq!(result, expected);
    }
}
