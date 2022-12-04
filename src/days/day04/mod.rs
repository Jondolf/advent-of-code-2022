use std::ops::RangeInclusive;

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = parse_input(input).into_iter();
    Some(pairs.fold(0, |acc, pair| acc + ranges_fully_overlap(pair) as u32))
}

pub fn part_two(input: &str) -> Option<u32> {
    let pairs = parse_input(input).into_iter();
    Some(pairs.fold(0, |acc, pair| acc + ranges_overlap(pair) as u32))
}

fn parse_input(input: &str) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    input.lines().map(parse_pair_string).collect()
}

/// Parses a string like "2-4,6-8" into a pair of inclusive ranges.
fn parse_pair_string(pair: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let pair = pair.split_once(',').unwrap();
    let first = pair.0.split_once('-').unwrap();
    let second = pair.1.split_once('-').unwrap();
    let first_range = first.0.parse::<usize>().unwrap()..=first.1.parse::<usize>().unwrap();
    let second_range = second.0.parse::<usize>().unwrap()..=second.1.parse::<usize>().unwrap();
    (first_range, second_range)
}

/// Checks if ranges fully overlap.
fn ranges_fully_overlap((a, b): (RangeInclusive<usize>, RangeInclusive<usize>)) -> bool {
    a.contains(b.start()) && a.contains(b.end()) || b.contains(a.start()) && b.contains(a.end())
}

/// Checks if ranges overlap at all.
fn ranges_overlap((a, b): (RangeInclusive<usize>, RangeInclusive<usize>)) -> bool {
    a.contains(b.start()) || b.contains(a.start()) || a.contains(b.end()) || b.contains(a.end())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_INPUT), Some(2));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_INPUT), Some(4));
    }
}
