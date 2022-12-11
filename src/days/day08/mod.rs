use std::str::Chars;

pub fn part_one(input: &str) -> Option<u32> {
    let (rows, columns) = parse_input(input);
    let mut visible_count = 0;

    for (y, row) in rows.iter().enumerate() {
        for (x, tree_height) in row.iter().enumerate() {
            let condition = |height| height < tree_height;
            let visible_top = columns[x].iter().take(y).all(condition);
            let visible_bottom = columns[x].iter().skip(y + 1).all(condition);
            let visible_left = row.iter().take(x).all(condition);
            let visible_right = row.iter().skip(x + 1).all(condition);

            if visible_top || visible_bottom || visible_left || visible_right {
                visible_count += 1;
            }
        }
    }

    Some(visible_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rows, columns) = parse_input(input);
    let mut best_scenic_score = 0;

    for (y, row) in rows.iter().enumerate() {
        for (x, tree_height) in row.iter().enumerate() {
            let view_up = get_visible_count(columns[x].iter().take(y).rev(), *tree_height);
            let view_bottom = get_visible_count(columns[x].iter().skip(y + 1), *tree_height);
            let view_left = get_visible_count(row.iter().take(x).rev(), *tree_height);
            let view_right = get_visible_count(row.iter().skip(x + 1), *tree_height);

            let scenic_score = (view_up * view_bottom * view_left * view_right) as u32;
            if scenic_score > best_scenic_score {
                best_scenic_score = scenic_score;
            }
        }
    }

    Some(best_scenic_score)
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let lines = input.lines();
    let rows: Vec<Vec<u8>> = lines.map(|line| chars_to_digits(line.chars())).collect();
    let columns: Vec<Vec<u8>> = (0..rows[0].len()).map(|x| get_column(&rows, x)).collect();
    (rows, columns)
}

fn chars_to_digits(chars: Chars) -> Vec<u8> {
    chars.map(|c| c.to_digit(10).unwrap() as u8).collect()
}

fn get_visible_count<'a, I: Iterator<Item = &'a u8>>(row: I, max_height: u8) -> usize {
    let mut viewing_distance = 0;
    for height in row {
        viewing_distance += 1;
        if *height >= max_height {
            break;
        }
    }
    viewing_distance
}

fn get_column<T: Clone + Copy>(table: &[Vec<T>], index: usize) -> Vec<T> {
    table.iter().map(|row| row[index]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_INPUT), Some(21));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_INPUT), Some(8));
    }
}
