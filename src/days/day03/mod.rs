use std::collections::BTreeSet;

use crate::utils;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn part_one(input: &str) -> Option<u32> {
    let sack_compartments = input
        .lines()
        .flat_map(|sack| {
            let (first, second) = sack.split_at(sack.len() / 2);
            vec![first.chars().collect(), second.chars().collect()]
        })
        .collect();

    Some(sum_of_common_items(sack_compartments, 2))
}

pub fn part_two(input: &str) -> Option<u32> {
    let sacks = input.lines().map(|sack| sack.chars().collect()).collect();
    Some(sum_of_common_items(sacks, 3))
}

/// Finds the common items in rucksack groups of a given size and returns the sum of their priorities.
fn sum_of_common_items(sacks: Vec<BTreeSet<char>>, chunk_size: usize) -> u32 {
    sacks.chunks(chunk_size).fold(0, |priority_acc, group| {
        let intersection = utils::intersection(group.iter());
        priority_acc + item_to_priority(intersection.into_iter().next().unwrap())
    })
}

/// Gets the priority of a given item type.
fn item_to_priority(character: char) -> u32 {
    ALPHABET.find(character).unwrap() as u32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_INPUT), Some(157));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_INPUT), Some(70));
    }
}
