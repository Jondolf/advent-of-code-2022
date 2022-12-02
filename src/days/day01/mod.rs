use crate::utils;

pub fn part_one(input: &str) -> Option<u32> {
    let calory_lists = parse_input(input);
    let mut calory_totals: Vec<u32> = sub_sum(&calory_lists);
    calory_totals.sort_by(|a, b| b.cmp(a)); // Sort descending
    Some(calory_totals[0])
}

pub fn part_two(input: &str) -> Option<u32> {
    let calory_lists = parse_input(input);
    let mut calory_totals: Vec<u32> = sub_sum(&calory_lists);
    calory_totals.sort_by(|a, b| b.cmp(a)); // Sort descending
    Some(calory_totals[0..3].iter().sum())
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let lists = input.split("\n\n");
    lists.map(|l| utils::parse_strings(l.split('\n'))).collect()
}

/// Iterates over a slice of number vectors and returns a vector with the items of the sub vectors summed.
pub fn sub_sum(items: &[Vec<u32>]) -> Vec<u32> {
    items.iter().map(|vec| vec.iter().sum()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_INPUT), Some(24_000));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_INPUT), Some(45_000));
    }
}
