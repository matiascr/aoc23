use std::env;
use std::fs;
use std::process::exit;
use std::str::FromStr;

#[derive(Debug)]
struct ParseGameError;
#[derive(Debug)]
struct ParseRoundError;

struct Game {
    index: u16,
    rounds: Vec<Round>,
}

struct Round {
    red: u16,
    green: u16,
    blue: u16,
}

impl Game {
    fn is_valid(&self) -> bool {
        self.rounds
            .iter()
            .fold(true, |acc, round| acc && round.is_valid())
    }
}

impl Round {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect::<Vec<_>>();

        let index: u16 = parts[0].split(' ').collect::<Vec<_>>()[1]
            .parse::<u16>()
            .unwrap();

        let rounds: Vec<Round> = parts[1]
            .split(';')
            .map(|round| Round::from_str(round).unwrap())
            .collect();

        Ok(Game {
            index: index,
            rounds: rounds,
        })
    }
}

impl FromStr for Round {
    type Err = ParseRoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut round = Round {
            red: 0,
            green: 0,
            blue: 0,
        };

        let colors = s.trim().split(',').map(|color| {
            let color_split: Vec<_> = color.trim().split(' ').collect();
            (color_split[0].parse::<u16>().unwrap(), color_split[1])
        });

        for (value, name) in colors.into_iter() {
            round = match name {
                "red" => Round {
                    red: value,
                    ..round
                },
                "green" => Round {
                    green: value,
                    ..round
                },
                "blue" => Round {
                    blue: value,
                    ..round
                },
                _ => round,
            }
        }

        Ok(round)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("ERROR: No valid arguments provided");
        exit(1);
    }

    let content = fs::read_to_string(&args[1]).expect("Should be able to read the file.");

    let result = content
        .trim()
        .split(&['\r', '\n'][..])
        .map(|line| Game::from_str(line).unwrap())
        .filter(Game::is_valid)
        .fold(0, |acc, game| acc + game.index);

    println!("{}", result);
}
