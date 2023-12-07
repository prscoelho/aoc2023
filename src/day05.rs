use std::collections::HashSet;

use crate::runner::Solve;

pub struct Day05;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Range {
    start: u64,
    end: u64,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Map {
    src: Range,
    dst: Range,
}

struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Vec<Map>>,
}

fn parse_input(input: &str) -> Almanac {
    let mut tokens = input.trim().split("\n\n");

    let seeds = tokens.next().unwrap();
    let seeds = seeds
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let maps = tokens
        .map(|s| {
            s.split('\n')
                .skip(1)
                .map(|s| {
                    let mut tokens = s
                        .split_ascii_whitespace()
                        .map(|s| s.parse::<u64>().unwrap());
                    let dst_start = tokens.next().unwrap();
                    let src_start = tokens.next().unwrap();
                    let len = tokens.next().unwrap();

                    let src = Range {
                        start: src_start,
                        end: src_start + len,
                    };
                    let dst = Range {
                        start: dst_start,
                        end: dst_start + len,
                    };

                    Map { src, dst }
                })
                .collect()
        })
        .collect();

    Almanac { seeds, maps }
}

fn find_next(position: u64, maps: &[Map]) -> u64 {
    for map in maps {
        if position >= map.src.start && position < map.src.end {
            let diff = position - map.src.start;
            return map.dst.start + diff;
        }
    }

    position
}

fn find_location(start: u64, almanac: &Almanac) -> u64 {
    let mut position = start;

    for placement in almanac.maps.iter() {
        position = find_next(position, placement);
    }

    position
}

fn range_intersection(mut range1: Range, mut range2: Range) -> Option<Range> {
    if range1.start > range2.start {
        std::mem::swap(&mut range1, &mut range2);
    }

    if range2.start >= range1.end {
        None
    } else {
        Some(Range {
            start: range2.start,
            end: range1.end.min(range2.end),
        })
    }
}

/// removes range2 from range1
/// range2 intersecting range1 is implied maybe rework this later
fn subtract_range(range1: Range, range2: Range) -> Vec<Range> {
    let mut result = Vec::new();

    let left = Range {
        start: range1.start,
        end: range2.start,
    };
    let right = Range {
        start: range2.end,
        end: range1.end,
    };

    if left.start != left.end {
        result.push(left);
    }

    if right.start != right.end {
        result.push(right)
    }

    result
}

fn find_next_ranges(start: HashSet<Range>, maps: &[Map]) -> HashSet<Range> {
    let mut input_ranges = start;
    let mut output_ranges = HashSet::new();

    for map in maps {
        for current_range in input_ranges.clone() {
            if let Some(range) = range_intersection(current_range, map.src) {
                let diff_left = range.start - map.src.start;
                let diff_right = range.end - map.src.start;

                let result_left = map.dst.start + diff_left;
                let result_right = map.dst.start + diff_right;

                let result = Range {
                    start: result_left,
                    end: result_right,
                };

                for excess in subtract_range(current_range, range) {
                    input_ranges.insert(excess);
                }
                input_ranges.remove(&current_range);
                output_ranges.insert(result);
            }
        }
    }

    for excess in input_ranges {
        output_ranges.insert(excess);
    }

    output_ranges
}

fn find_locations_ranged(start: Range, almanac: &Almanac) -> HashSet<Range> {
    let mut positions = HashSet::new();
    positions.insert(start);

    for placement in almanac.maps.iter() {
        positions = find_next_ranges(positions, placement);
    }

    positions
}

impl Solve<u64, u64> for Day05 {
    fn part1(input: &str) -> u64 {
        let almanac = parse_input(input);
        almanac
            .seeds
            .iter()
            .map(|&seed| find_location(seed, &almanac))
            .min()
            .unwrap()
    }
    fn part2(input: &str) -> u64 {
        let almanac = parse_input(input);
        almanac
            .seeds
            .chunks_exact(2)
            .map(|slice| Range {
                start: slice[0],
                end: slice[0] + slice[1],
            })
            .flat_map(|seed_range| find_locations_ranged(seed_range, &almanac))
            .min()
            .unwrap()
            .start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    #[test]
    fn parses_example() {
        // todo
    }

    #[test]
    fn example_p1() {
        let result = Day05::part1(EXAMPLE);
        let expected = 35;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day05::part2(EXAMPLE);
        let expected = 46;

        assert_eq!(result, expected);
    }
}
