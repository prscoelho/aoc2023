use std::env;

use crate::runner::{parse_day, read_day_input, Solve};
mod runner;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("error: missing day to run.");
        return;
    }

    let day = match parse_day(&args[1]) {
        Ok(day) => day,
        Err(e) => {
            eprintln!("Failed to parse day: {}", e);
            return;
        }
    };
    let input = read_day_input(day);

    let (p1, p2) = match day {
        1 => day01::Day01::solve(&input),
        2 => day02::Day02::solve(&input),
        3 => day03::Day03::solve(&input),
        4 => day04::Day04::solve(&input),
        5 => day05::Day05::solve(&input),
        6 => day06::Day06::solve(&input),
        7 => day07::Day07::solve(&input),
        8 => day08::Day08::solve(&input),
        9 => day09::Day09::solve(&input),
        _ => {
            eprintln!("Not implemented yet");
            return;
        }
    };

    println!("Running day: {:02}", day);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
