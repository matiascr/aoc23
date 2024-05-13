use std::cmp;
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

    fn get_max(&self) -> Round {
        self.rounds
            .iter()
            .fold(Round::new(), |acc, round| round.get_min_required(acc))
    }
}

impl Round {
    fn new() -> Round {
        Round {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn get_min_required(&self, other: Round) -> Round {
        Round {
            red: cmp::max(self.red, other.red),
            green: cmp::max(self.green, other.green),
            blue: cmp::max(self.blue, other.blue),
        }
    }

    fn power(&self) -> u16 {
        self.red * self.green * self.blue
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
        let mut round = Round::new();

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

fn part_1(path: &str) -> u16 {
    let content = fs::read_to_string(path).expect("Should be able to read the file.");

    content
        .trim()
        .split(&['\r', '\n'][..])
        .map(|line| Game::from_str(line).unwrap())
        .filter(Game::is_valid)
        .fold(0, |acc, game| acc + game.index)
}

fn part_2(path: &str) -> u16 {
    let content = fs::read_to_string(path).expect("Should be able to read the file.");

    content
        .trim()
        .split(&['\r', '\n'][..])
        .map(|line| Game::from_str(line).unwrap())
        .map(|game| Game::get_max(&game))
        .fold(0, |acc, round| acc + Round::power(&round))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("ERROR: No valid arguments provided");
        exit(1);
    }

    part_1(&args[1]);
}

#[cfg(test)]
mod tests {

    static DATA_FILE: &str = "data/data.txt";

    #[test]
    fn part_1() {
        let result = super::part_1(DATA_FILE);

        println!("Part 1 result: {}", result);
        assert_eq!(result, 2101);
    }

    #[test]
    fn part_2() {
        let result = super::part_2(DATA_FILE);

        println!("Part 2 result: {}", result);
        assert_eq!(result, 58269);
    }
}
