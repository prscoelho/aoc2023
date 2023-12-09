use crate::runner::Solve;

pub struct Day09;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

fn get_differences(values: &[i64]) -> Vec<i64> {
    if values.is_empty() {
        return Vec::with_capacity(0);
    }
    let mut result = Vec::with_capacity(values.len() - 1);
    for idx in 1..values.len() {
        let diff = values[idx] - values[idx - 1];
        result.push(diff);
    }

    result
}

fn predict(history: &[i64]) -> i64 {
    let mut lasts = vec![history.last().cloned().unwrap_or(0)];

    let mut current: Vec<i64> = history.to_owned();

    loop {
        current = get_differences(&current);

        lasts.push(*current.last().unwrap());

        if current.iter().all(|v| v == &0) {
            break;
        }
    }

    lasts.into_iter().sum()
}

fn predict_left(history: &[i64]) -> i64 {
    let mut firsts = vec![history.first().cloned().unwrap_or(0)];
    let mut current: Vec<i64> = history.to_owned();

    loop {
        current = get_differences(&current);

        firsts.push(*current.first().unwrap());

        if current.iter().all(|v| v == &0) {
            break;
        }
    }

    firsts.into_iter().rev().reduce(|acc, v| v - acc).unwrap()
}

impl Solve<i64, i64> for Day09 {
    fn part1(input: &str) -> i64 {
        let histories = parse_input(input);
        histories.iter().map(|h| predict(h)).sum()
    }
    fn part2(input: &str) -> i64 {
        let histories = parse_input(input);
        histories.iter().map(|h| predict_left(h)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    #[test]
    fn example_p1() {
        let result = Day09::part1(EXAMPLE);
        let expected = 114;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day09::part2(EXAMPLE);
        let expected = 2;

        assert_eq!(result, expected);
    }
}
