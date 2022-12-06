pub fn part_one(input: &str) -> Option<u32> {
    find_unique_sequence_end(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_unique_sequence_end(input, 14)
}

fn find_unique_sequence_end(input: &str, sequence_length: usize) -> Option<u32> {
    let chars = input.chars().collect::<Vec<char>>();
    let mut seq_windows = chars.windows(sequence_length).enumerate();

    'outer: while let Some((sequence_start, sequence)) = seq_windows.next() {
        // Iterate sequence in reverse. When a duplicate value is found, we can skip until the current index.
        for (i, char_1) in sequence.iter().rev().enumerate() {
            for (j, char_2) in sequence[0..sequence.len() - i - 1].iter().rev().enumerate() {
                // Duplicate value found, skip to current index if the current index is larger than the minimum sequence length.
                if char_1 == char_2 {
                    let advance_by = sequence_length - j - i - 1;
                    for _ in 0..advance_by - 1 {
                        seq_windows.next();
                    }
                    continue 'outer;
                }
            }
        }
        return Some(sequence_start as u32 + sequence_length as u32);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part_one(input), Some(7));
        assert_eq!(part_two(input), Some(19));
    }

    #[test]
    fn example_2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part_one(input), Some(5));
        assert_eq!(part_two(input), Some(23));
    }

    #[test]
    fn example_3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part_one(input), Some(6));
        assert_eq!(part_two(input), Some(23));
    }

    #[test]
    fn example_4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part_one(input), Some(10));
        assert_eq!(part_two(input), Some(29));
    }

    #[test]
    fn example_5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part_one(input), Some(11));
        assert_eq!(part_two(input), Some(26));
    }
}
