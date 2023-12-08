use crate::runner::Solve;

pub struct Day06;

struct Race {
    time: u64,
    distance: u64,
}

fn parse_input(input: &str) -> Vec<Race> {
    let numbers: Vec<Vec<u64>> = input
        .trim()
        .split('\n')
        .map(|s| s.split_once(':').unwrap().1)
        .map(|s| s.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();

    let n = numbers[0].len();
    (0..n)
        .map(|race_idx| Race {
            time: numbers[0][race_idx],
            distance: numbers[1][race_idx],
        })
        .collect()
}

fn simulate_race(time_remaining: u64, speed: u64) -> u64 {
    speed * time_remaining
}

fn calculate_record_breaks(race: Race) -> u64 {
    let mut result = 0;
    for time_spend_holding in 0..=race.time {
        let time_remaining = race.time - time_spend_holding;
        let distance = simulate_race(time_remaining, time_spend_holding);
        if distance > race.distance {
            result += 1;
        }
    }
    result
}

fn parse_input_kernel_fix(input: &str) -> Race {
    let numbers: Vec<u64> = input
        .trim()
        .split('\n')
        .map(|s| s.split_once(':').unwrap().1)
        .map(|s| {
            let mut s = String::from(s);
            s.retain(|ch| ch.is_ascii_digit());
            s
        })
        .map(|s| s.parse().unwrap())
        .collect();

    Race {
        time: numbers[0],
        distance: numbers[1],
    }
}
impl Solve<u64, u64> for Day06 {
    fn part1(input: &str) -> u64 {
        let races = parse_input(input);
        races.into_iter().map(calculate_record_breaks).product()
    }
    fn part2(input: &str) -> u64 {
        let race = parse_input_kernel_fix(input);
        calculate_record_breaks(race)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Time:      7  15   30
Distance:  9  40  200
"#;

    #[test]
    fn example_p1() {
        let result = Day06::part1(EXAMPLE);
        let expected = 288;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day06::part2(EXAMPLE);
        let expected = 71503;

        assert_eq!(result, expected);
    }
}
