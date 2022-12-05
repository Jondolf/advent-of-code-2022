type Crate = char;

#[derive(Debug)]
struct MoveStep {
    /// How many crates to move from one stack to another.
    quantity: usize,
    /// Which crate stack to move crates from.\
    /// **Note**: Start index is 1.
    from: usize,
    /// Which crate stack to move crates on top of.\
    /// **Note**: Start index is 1.
    to: usize,
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut crates, steps) = parse_input(input);

    for step in steps {
        let lifted = lift_crates(&mut crates[step.from - 1], step.quantity);
        // Crates are lifted one by one, so they are unloaded in reverse order.
        crates[step.to - 1].extend(lifted.iter().rev());
    }

    let top_crates = crates.iter().map(|stack| stack.last().unwrap());
    Some(top_crates.collect::<String>())
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut crates, steps) = parse_input(input);

    for step in steps {
        let lifted = lift_crates(&mut crates[step.from - 1], step.quantity);
        crates[step.to - 1].extend(lifted);
    }

    let top_crates = crates.iter().map(|stack| stack.last().unwrap());
    Some(top_crates.collect::<String>())
}

/// Lifts (removes) crates from a given crate stack and returns the lifted crates as a vector.
fn lift_crates(crate_stack: &mut Vec<Crate>, quantity: usize) -> Vec<Crate> {
    let lifted = crate_stack.drain(crate_stack.len() - quantity..crate_stack.len());
    lifted.collect::<Vec<Crate>>()
}

fn parse_input(input: &str) -> (Vec<Vec<Crate>>, Vec<MoveStep>) {
    let (crates, steps) = input.split_once("\n\n").unwrap();
    (parse_crates(crates), parse_steps(steps))
}

/// Constructs a `Vec<Vec<Crate>>` from a string input. Each subvector is a crate stack with the last item being the crate on top.
fn parse_crates(crates: &str) -> Vec<Vec<Crate>> {
    let width = crates.lines().next().unwrap().trim_end_matches('\n').len();
    let mut stacks: Vec<Vec<Crate>> = vec![];

    // Loop through the crate stacks. The crate labels start with an offset of 1 and are 4 characters apart.
    for (stack_idx, x) in (0..=width).skip(1).step_by(4).enumerate() {
        stacks.push(vec![]);

        // Loop through the rows of the crate stack from bottom to top.
        for row in crates.lines().rev() {
            let crate_name = row.chars().nth(x).unwrap();

            // Add crate to the stack if it's valid.
            if crate_name.is_alphabetic() {
                stacks[stack_idx].push(crate_name);
            }
        }
    }

    stacks
}

/// Parses rearrangement procedure steps from a string into a `Vec<MoveStep>`.
fn parse_steps(steps: &str) -> Vec<MoveStep> {
    steps
        .lines()
        .map(|line| {
            // Step format: "move <num> from <num> to <num>"
            let mut numbers = line.split_whitespace().skip(1).step_by(2);
            MoveStep {
                quantity: numbers.next().unwrap().parse::<usize>().unwrap(),
                from: numbers.next().unwrap().parse::<usize>().unwrap(),
                to: numbers.next().unwrap().parse::<usize>().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_INPUT), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_INPUT), Some("MCD".to_string()));
    }
}
