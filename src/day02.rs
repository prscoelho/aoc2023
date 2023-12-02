use crate::runner::Solve;

pub struct Day02;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
struct ColorSet {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_color_set(text: &str) -> ColorSet {
    let mut result = ColorSet::default();
    for (number, color) in text.split(", ").filter_map(|str| str.split_once(' ')) {
        let reveal_value = number.parse::<u32>().expect("reveal number is not a value");
        match color {
            "red" => result.red = reveal_value,
            "blue" => result.blue = reveal_value,
            "green" => result.green = reveal_value,
            _ => panic!("expected a color"),
        }
    }

    result
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Game {
    game_id: u32,
    sets: Vec<ColorSet>,
}

fn parse_line(line: &str) -> Game {
    let (game_id_text, rest) = line.split_once(": ").expect("line must have a `: `");
    let game_id: u32 = game_id_text.strip_prefix("Game ").unwrap().parse().unwrap();

    let sets = rest.split("; ").map(parse_color_set).collect();

    Game { game_id, sets }
}

fn parse_input(input: &str) -> Vec<Game> {
    input.trim().lines().map(parse_line).collect()
}

fn maximum(game: &Game) -> ColorSet {
    let mut result = ColorSet::default();

    for set in game.sets.iter() {
        result.red = u32::max(result.red, set.red);
        result.blue = u32::max(result.blue, set.blue);
        result.green = u32::max(result.green, set.green);
    }

    result
}

fn power(set: &ColorSet) -> u32 {
    set.red * set.blue * set.green
}

impl Solve<u32, u32> for Day02 {
    fn part1(input: &str) -> u32 {
        let games = parse_input(input);

        games
            .into_iter()
            .filter(|game| {
                let max_set = maximum(game);
                max_set.red <= 12 && max_set.green <= 13 && max_set.blue <= 14
            })
            .map(|game| game.game_id)
            .sum()
    }
    fn part2(input: &str) -> u32 {
        let games = parse_input(input);

        games
            .into_iter()
            .map(|game| maximum(&game))
            .map(|set| power(&set))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

    #[test]
    fn parses_example() {
        let result = parse_input(EXAMPLE).get(0).unwrap().clone();
        let expected = Game {
            game_id: 1,
            sets: vec![
                ColorSet {
                    blue: 3,
                    red: 4,
                    green: 0,
                },
                ColorSet {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                ColorSet {
                    green: 2,
                    red: 0,
                    blue: 0,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p1() {
        let result = Day02::part1(EXAMPLE);
        let expected = 8;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day02::part2(EXAMPLE);
        let expected = 2286;

        assert_eq!(result, expected);
    }
}
