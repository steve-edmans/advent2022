use std::{fmt, fs};
use std::fmt::Formatter;
use advent2022::DailyChallenge;
use crate::days::two::Choice::{Paper, Rock, Scissors};

pub struct Two {}

impl DailyChallenge for Two {
    fn run(&self) {
        println!("Day Two");
        let results: Vec<Game> = fs::read_to_string("contents/day_two.txt")
            .expect("Should have been able to read file")
            .lines()
            // .take(10)
            .flat_map(|line| Game::from(line))
            .collect();
        let part_one_results: Vec<u32> = results.
            iter()
            .map(|game| game.score() as u32)
            .collect();
        let part_one: u32 = part_one_results.iter().sum();
        println!("The result of part one is {:?}", part_one);
        let part_two_results: Vec<u32> = results
            .iter()
            .map(|game| game.alternate_score() as u32)
            .collect();
        let part_two: u32 = part_two_results.iter().sum();
        println!("The result of part two is {:?}", part_two);
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors
}

impl Choice {
    fn from(code: &str) -> Result<Choice, &'static str> {
        match code {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissors),
            "X" => Ok(Rock),
            "Y" => Ok(Paper),
            "Z" => Ok(Scissors),
            _ => Err("Unknown Type"),
        }
    }

    fn score(&self) -> u8 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

#[derive(Debug, PartialEq)]
enum MatchResult {
    Win,
    Draw,
    Lose
}

impl MatchResult {
    fn score(&self) -> u8 {
        match self {
            MatchResult::Win => 6,
            MatchResult::Draw => 3,
            MatchResult::Lose => 0,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Game { opponent: Choice, recommended: Choice }

impl Game {
    fn from(line: &str) -> Result<Game, &'static str> {
        let parts: Vec<Choice> = line
            .split(" ")
            .flat_map(|choice| Choice::from(choice))
            .collect();

        match parts.as_slice() {
            [opponent, recommended] => {
                return Ok(Game { opponent: *opponent, recommended: *recommended});
            },
            _ => return Err("Invalid Game Line"),
        }
    }

    fn result(&self) -> MatchResult {
        match (self.recommended, self.opponent) {
            (Rock, Scissors) => MatchResult::Win,
            (Rock, Paper) => MatchResult::Lose,
            (Paper, Rock) => MatchResult::Win,
            (Paper, Scissors) => MatchResult::Lose,
            (Scissors, Rock) => MatchResult::Lose,
            (Scissors, Paper) => MatchResult::Win,
            _ => MatchResult::Draw,
        }
    }

    fn score(&self) -> u8 {
        self.recommended.score() + self.result().score()
    }

    fn alternate_recommendation(&self) -> MatchResult {
        match self.recommended {
            Rock => MatchResult::Lose,
            Paper => MatchResult::Draw,
            Scissors => MatchResult::Win,
        }
    }

    fn alternate_result(&self) -> Choice {
        match (self.opponent, self.alternate_recommendation()) {
            (Rock, MatchResult::Win) => Paper,
            (Rock, MatchResult::Lose) => Scissors,
            (Rock, MatchResult::Draw) => Rock,
            (Paper, MatchResult::Win) => Scissors,
            (Paper, MatchResult::Lose) => Rock,
            (Paper, MatchResult::Draw) => Paper,
            (Scissors, MatchResult::Win) => Rock,
            (Scissors, MatchResult::Lose) => Paper,
            (Scissors, MatchResult::Draw) => Scissors,
        }
    }

    fn alternate_score(&self) -> u8 {
        self.alternate_result().score() + self.alternate_recommendation().score()
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} vs {:?} = {:?}", self.recommended, self.opponent, self.result())
    }
}

#[cfg(test)]
mod tests {
    use crate::days::two::Choice::{Paper, Rock};
    use super::*;

    #[test]
    fn decode_unkown_choice() {
        assert_eq!(Err("Unknown Type"), Choice::from("R"));
    }

    #[test]
    fn decode_a_to_rock() {
        assert_eq!(Ok(Choice::Rock), Choice::from("A"));
    }

    #[test]
    fn decode_b_to_paper() {
        assert_eq!(Ok(Choice::Paper), Choice::from("B"));
    }

    #[test]
    fn decode_c_to_scissors() {
        assert_eq!(Ok(Choice::Scissors), Choice::from("C"));
    }

    #[test]
    fn decode_x_to_rock() {
        assert_eq!(Ok(Choice::Rock), Choice::from("X"));
    }

    #[test]
    fn decode_y_to_paper() {
        assert_eq!(Ok(Choice::Paper), Choice::from("Y"));
    }

    #[test]
    fn decode_z_to_scissors() {
        assert_eq!(Ok(Choice::Scissors), Choice::from("Z"));
    }

    #[test]
    fn score_for_rock_is_1() {
        assert_eq!(Choice::Rock.score(), 1);
    }

    #[test]
    fn score_for_paper_is_2() {
        assert_eq!(Choice::Paper.score(), 2);
    }

    #[test]
    fn score_for_scissors_is_3() {
        assert_eq!(Choice::Scissors.score(), 3);
    }

    #[test]
    fn decode_invalid_line() {
        assert_eq!(Err("Invalid Game Line"), Game::from("E E"));
    }

    #[test]
    fn convert_line() {
        let line = "A Y";
        let expected = Ok(Game { opponent: Rock, recommended: Paper});
        assert_eq!(expected, Game::from(line));
    }

    #[test]
    fn rock_draws_with_rock() {
        let game = Game { opponent: Rock, recommended: Rock };
        assert_eq!(game.result(), MatchResult::Draw);
    }

    #[test]
    fn rock_beats_scissors() {
        let game = Game { opponent: Scissors, recommended: Rock };
        assert_eq!(game.result(), MatchResult::Win);
    }

    #[test]
    fn rock_loses_to_paper() {
        let game = Game { opponent: Paper, recommended: Rock };
        assert_eq!(game.result(), MatchResult::Lose);
    }

    #[test]
    fn paper_beats_rock() {
        let game = Game { opponent: Rock, recommended: Paper };
        assert_eq!(game.result(), MatchResult::Win);
    }

    #[test]
    fn paper_draws_with_paper() {
        let game = Game { opponent: Paper, recommended: Paper };
        assert_eq!(game.result(), MatchResult::Draw);
    }

    #[test]
    fn paper_loses_to_scissors() {
        let game = Game { opponent: Scissors, recommended: Paper };
        assert_eq!(game.result(), MatchResult::Lose);
    }

    #[test]
    fn scissors_loses_to_rock() {
        let game = Game { opponent: Rock, recommended: Scissors };
        assert_eq!(game.result(), MatchResult::Lose);
    }

    #[test]
    fn scissors_beats_paper() {
        let game = Game { opponent: Paper, recommended: Scissors };
        assert_eq!(game.result(), MatchResult::Win);
    }

    #[test]
    fn scissors_draws_with_scissors() {
        let game = Game { opponent: Scissors, recommended: Scissors };
        assert_eq!(game.result(), MatchResult::Draw);
    }

    #[test]
    fn score_for_win() {
        assert_eq!((MatchResult::Win).score(), 6);
    }

    #[test]
    fn score_for_draw() {
        assert_eq!((MatchResult::Draw).score(), 3);
    }

    #[test]
    fn score_for_lose() {
        assert_eq!((MatchResult::Lose).score(), 0);
    }

    #[test]
    fn a_y_game_score() {
        let game = Game::from("A Y").unwrap();
        assert_eq!(game.score(), 8);
    }

    #[test]
    fn b_x_game_score() {
        let game = Game::from("B X").unwrap();
        assert_eq!(game.score(), 1);
    }

    #[test]
    fn c_z_game_score() {
        let game = Game::from("C Z").unwrap();
        assert_eq!(game.score(), 6);
    }

    #[test]
    fn alternate_recommendation_win() {
        let game = Game::from("A Z").unwrap();
        assert_eq!(MatchResult::Win, game.alternate_recommendation());
    }

    #[test]
    fn alternate_recommendation_lose() {
        let game = Game::from("A X").unwrap();
        assert_eq!(MatchResult::Lose, game.alternate_recommendation());
    }

    #[test]
    fn alternate_recommendation_draw() {
        let game = Game::from("A Y").unwrap();
        assert_eq!(MatchResult::Draw, game.alternate_recommendation());
    }

    #[test]
    fn alternate_rock_win() {
        let game = Game::from("A Z").unwrap();
        assert_eq!(game.alternate_result(), Paper);
    }

    #[test]
    fn alternate_rock_lose() {
        let game = Game::from("A X").unwrap();
        assert_eq!(game.alternate_result(), Scissors);
    }

    #[test]
    fn alternate_rock_draw() {
        let game = Game::from("A Y").unwrap();
        assert_eq!(game.alternate_result(), Rock);
    }

    #[test]
    fn alternate_paper_win() {
        let game = Game::from("B Z").unwrap();
        assert_eq!(game.alternate_result(), Scissors);
    }

    #[test]
    fn alternate_paper_lose() {
        let game = Game::from("B X").unwrap();
        assert_eq!(game.alternate_result(), Rock);
    }

    #[test]
    fn alternate_paper_draw() {
        let game = Game::from("B Y").unwrap();
        assert_eq!(game.alternate_result(), Paper);
    }

    #[test]
    fn alternate_scissors_win() {
        let game = Game::from("C Z").unwrap();
        assert_eq!(game.alternate_result(), Rock);
    }

    #[test]
    fn alternate_scissors_lose() {
        let game = Game::from("C X").unwrap();
        assert_eq!(game.alternate_result(), Paper);
    }

    #[test]
    fn alternate_scissors_draw() {
        let game = Game::from("C Y").unwrap();
        assert_eq!(game.alternate_result(), Scissors);
    }

    #[test]
    fn a_x_alternate_score() {
        let game = Game::from("A X").unwrap();
        assert_eq!(game.alternate_score(), 3);
    }

    #[test]
    fn a_y_alternate_score() {
        let game = Game::from("A Y").unwrap();
        assert_eq!(game.alternate_score(), 4);
    }

    #[test]
    fn a_z_alternate_score() {
        let game = Game::from("A Z").unwrap();
        assert_eq!(game.alternate_score(), 8);
    }

    #[test]
    fn b_x_alternate_score() {
        let game = Game::from("B X").unwrap();
        assert_eq!(game.alternate_score(), 1);
    }

    #[test]
    fn b_y_alternate_score() {
        let game = Game::from("B Y").unwrap();
        assert_eq!(game.alternate_score(), 5);
    }

    #[test]
    fn b_z_alternate_score() {
        let game = Game::from("B Z").unwrap();
        assert_eq!(game.alternate_score(), 9);
    }

    #[test]
    fn c_x_alternate_score() {
        let game = Game::from("C X").unwrap();
        assert_eq!(game.alternate_score(), 2);
    }

    #[test]
    fn c_y_alternate_score() {
        let game = Game::from("C Y").unwrap();
        assert_eq!(game.alternate_score(), 6);
    }

    #[test]
    fn c_z_alternate_score() {
        let game = Game::from("C Z").unwrap();
        assert_eq!(game.alternate_score(), 7);
    }
}
