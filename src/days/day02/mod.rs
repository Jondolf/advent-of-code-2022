/// An outcome with its score as the value
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl Outcome {
    fn from_symbol(symbol: &str) -> Option<Self> {
        match symbol {
            "X" => Some(Self::Loss),
            "Y" => Some(Self::Draw),
            "Z" => Some(Self::Win),
            _ => None,
        }
    }
}

/// A shape with its score as the value
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn from_symbol(symbol: &str) -> Option<Self> {
        match symbol {
            "A" | "X" => Some(Self::Rock),
            "B" | "Y" => Some(Self::Paper),
            "C" | "Z" => Some(Self::Scissors),
            _ => None,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_score = 0;

    for round in parse_input(input) {
        let opponent_shape = Shape::from_symbol(round.0).unwrap();
        let player_shape = Shape::from_symbol(round.1).unwrap();
        total_score += player_shape as u32 + get_outcome(player_shape, opponent_shape) as u32;
    }

    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_score = 0;

    for round in parse_input(input) {
        let desired_outcome = Outcome::from_symbol(round.1).unwrap();
        let opponent_shape = Shape::from_symbol(round.0).unwrap();
        let player_shape = shape_for_desired_outcome(desired_outcome, opponent_shape);
        total_score += player_shape as u32 + get_outcome(player_shape, opponent_shape) as u32;
    }

    Some(total_score)
}

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    let rounds = input.split('\n');
    rounds.map(|round| round.split_once(' ').unwrap()).collect()
}

/// Gets the outcome of playing a given shape against another given shape.
fn get_outcome(shape: Shape, against_shape: Shape) -> Outcome {
    match shape {
        Shape::Rock => match against_shape {
            Shape::Rock => Outcome::Draw,
            Shape::Paper => Outcome::Loss,
            Shape::Scissors => Outcome::Win,
        },
        Shape::Paper => match against_shape {
            Shape::Rock => Outcome::Win,
            Shape::Paper => Outcome::Draw,
            Shape::Scissors => Outcome::Loss,
        },
        Shape::Scissors => match against_shape {
            Shape::Rock => Outcome::Loss,
            Shape::Paper => Outcome::Win,
            Shape::Scissors => Outcome::Draw,
        },
    }
}

/// Finds the shape required to get a given outcome against a given shape.
fn shape_for_desired_outcome(outcome: Outcome, against_shape: Shape) -> Shape {
    match outcome {
        // Play shape that wins
        Outcome::Win => match against_shape {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
        // Play shape that loses
        Outcome::Loss => match against_shape {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
        // Play the same shape
        Outcome::Draw => against_shape,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"A Y
B X
C Z"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_INPUT), Some(15));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_INPUT), Some(12));
    }
}
