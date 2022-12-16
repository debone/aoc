use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum Shapes {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq, Clone, Copy)]
enum RoundOutcomes {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn whichFight(mine: RoundOutcomes, their: Shapes) -> Shapes {
    match (mine, their) {
        (RoundOutcomes::Draw, _) => their,
        (RoundOutcomes::Win, Shapes::Paper) => Shapes::Scissors,
        (RoundOutcomes::Win, Shapes::Rock) => Shapes::Paper,
        (RoundOutcomes::Win, Shapes::Scissors) => Shapes::Rock,
        (RoundOutcomes::Lose, Shapes::Paper) => Shapes::Rock,
        (RoundOutcomes::Lose, Shapes::Rock) => Shapes::Scissors,
        (RoundOutcomes::Lose, Shapes::Scissors) => Shapes::Paper,
    }
}

fn fight(mine: Shapes, their: Shapes) -> RoundOutcomes {
    match (mine, their) {
        (Shapes::Rock, Shapes::Paper) => RoundOutcomes::Lose,
        (Shapes::Rock, Shapes::Scissors) => RoundOutcomes::Win,
        (Shapes::Paper, Shapes::Scissors) => RoundOutcomes::Lose,
        (Shapes::Paper, Shapes::Rock) => RoundOutcomes::Win,
        (Shapes::Scissors, Shapes::Rock) => RoundOutcomes::Lose,
        (Shapes::Scissors, Shapes::Paper) => RoundOutcomes::Win,
        (_, _) => RoundOutcomes::Draw,
    }
}

pub fn first() {
    let contents = fs::read_to_string("day_2/input.txt").unwrap();

    let mut totalScore = 0;

    for line in contents.lines() {
        let their = match line.chars().nth(0).unwrap() {
            'A' => Shapes::Rock,
            'B' => Shapes::Paper,
            'C' => Shapes::Scissors,
            _ => Shapes::Paper,
        };

        let mine = match line.chars().nth(2).unwrap() {
            'X' => Shapes::Rock,
            'Y' => Shapes::Paper,
            'Z' => Shapes::Scissors,
            _ => Shapes::Paper,
        };

        totalScore += match mine {
            Shapes::Rock => 1,
            Shapes::Paper => 2,
            Shapes::Scissors => 3,
        };
        totalScore += match fight(mine, their) {
            RoundOutcomes::Lose => 0,
            RoundOutcomes::Draw => 3,
            RoundOutcomes::Win => 6,
        }
    }

    println!("{totalScore}");
}

pub fn second() {
    let contents = fs::read_to_string("day_2/input.txt").unwrap();

    let mut totalScore = 0;

    for line in contents.lines() {
        let their = match line.chars().nth(0).unwrap() {
            'A' => Shapes::Rock,
            'B' => Shapes::Paper,
            'C' => Shapes::Scissors,
            _ => Shapes::Paper,
        };

        let mineOutcome = match line.chars().nth(2).unwrap() {
            'X' => RoundOutcomes::Lose,
            'Y' => RoundOutcomes::Draw,
            'Z' => RoundOutcomes::Win,
            _ => RoundOutcomes::Lose,
        };

        let mine = whichFight(mineOutcome, their);

        totalScore += match mineOutcome {
            RoundOutcomes::Lose => 0,
            RoundOutcomes::Draw => 3,
            RoundOutcomes::Win => 6,
        };
        totalScore += match mine {
            Shapes::Rock => 1,
            Shapes::Paper => 2,
            Shapes::Scissors => 3,
        };
    }

    println!("{totalScore}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
