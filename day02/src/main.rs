use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(PartialEq, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    pub fn new(play: &str) -> Play {
        match play {
            "A" | "X" => Play::Rock,
            "B" | "Y" => Play::Paper,
            "C" | "Z" => Play::Scissors,
            &_ => panic!("Not valid move!"),
        }
    }

    //Part 1
    pub fn beats(self, other: Play) -> Outcome {
        match (self, other) {
            (s, o) if s == Play::Rock && o == Play::Scissors => Outcome::Win,
            (s, o) if s == Play::Paper && o == Play::Rock => Outcome::Win,
            (s, o) if s == Play::Scissors && o == Play::Paper => Outcome::Win,
            (s, o) if s == o => Outcome::Draw,
            (_, _) => Outcome::Lose
        }
    }

    // Part 2
    pub fn  get_play_by_outcome(self, result: Outcome) -> Play {
        match (self, result) {
            (s, r) if s == Play::Rock && r == Outcome::Win => Play::Paper,
            (s, r) if s == Play::Rock && r == Outcome::Lose => Play::Scissors,
            (s, r) if s == Play::Paper && r == Outcome::Win => Play::Scissors,
            (s, r) if s == Play::Paper && r == Outcome::Lose => Play::Rock,
            (s, r) if s == Play::Scissors && r == Outcome::Win => Play::Rock,
            (s, r) if s == Play::Scissors && r == Outcome::Lose => Play::Paper,
            (_s, r) if r == Outcome::Draw => self.clone(),
            (_, _) => panic!("Undefined play/outcome pairing")
         }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Outcome {
    Win,
    Lose,
    Draw,
    None,
}

impl Outcome {
    pub fn new(outcome: &str) -> Outcome {
        match outcome  {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Incorrect result format")
        }
    }
}

struct Game {
    opponent_move: Play,
    my_move: Play,
    outcome: Outcome,
}

impl Game {
    // Part1
    pub fn new(opponent_play: &str, my_play: &str) -> Game {
        let mut game = Game {
            opponent_move: Play::new(opponent_play),
            my_move: Play::new(my_play),
            outcome: Outcome::None,
        };
        game.outcome = game.my_move.beats(game.opponent_move);
        game
    }

    // Part2
    pub fn new2(opponent_play: &str, outcome: &str) -> Game {
        let opponent_move = Play::new(opponent_play);
        let result = Outcome::new(outcome);
        Game {
            opponent_move: opponent_move,
            my_move: opponent_move.get_play_by_outcome(result),
            outcome: result,
        }
    }


    pub fn get_score(self) -> u32 {
        self.get_outcome_score() + self.get_play_score()
    }

    fn get_outcome_score(&self) -> u32 {
        match self.outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
            Outcome::None => panic!("Outcome was never set!"),
        }
    }

    fn get_play_score(&self) -> u32 {
        match self.my_move {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
}

fn main() {
    let mut score: u32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let moves: Vec<&str> = l.split(' ').collect();
                // part 1 - fmt "opponent move, my move"
                //let round = Game::new(moves[0], moves[1]);

                // part 2 - fmt "Opponent move, desired outcome"
                let round = Game::new2(moves[0], moves[1]);
                
                score += round.get_score()
            }
        }
    }
    println!("Total Score: {}", score);
}
