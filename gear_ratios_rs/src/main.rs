use std::cmp;
use std::env;
use std::fmt::Display;
use std::fs;
use std::process::exit;
use std::usize;

struct Number {
    prev_line: Option<String>,
    line: String,
    next_line: Option<String>,
}

impl Number {
    fn part_number(&self) -> Option<u32> {
        let prev_has = match &self.prev_line {
            Some(s) => !s.replace('.', "").is_empty(),
            None => false,
        };

        let next_has = match &self.next_line {
            Some(s) => !s.replace('.', "").is_empty(),
            None => false,
        };

        let line_has = !&self
            .line
            .chars()
            .filter(|c| !c.is_ascii_digit() && *c != '.')
            .collect::<String>()
            .is_empty();

        if !prev_has && !line_has && !next_has {
            return None;
        }

        let res = Some(
            self.line
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse::<u32>()
                .unwrap(),
        );

        res
    }
}

impl Display for Number {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.prev_line.clone() {
            Some(s) => println!("{:}", s),
            None => {}
        }

        println!("{:}", self.line);

        match self.next_line.clone() {
            Some(s) => println!("{:}", s),
            None => {}
        }
        Ok(())
    }
}

trait Schematic {
    fn to_numbers(&self) -> Vec<Number>;
}

impl Schematic for str {
    fn to_numbers(&self) -> Vec<Number> {
        let mut numbers = Vec::new();

        let lines: Vec<&str> = self.split('\n').collect::<Vec<_>>();

        let end = lines.len();

        for i in 0..end {
            let line: &str = lines[i];

            let mut line_numbers = Vec::new();

            let present_numbers: Vec<u32> = line
                .split(|c: char| c.is_ascii_punctuation())
                .filter_map(|s| s.parse().ok())
                .collect();

            let indices: Vec<(usize, usize)> = present_numbers
                .into_iter()
                .flat_map(|pn| {
                    let pn_str = pn.to_string();
                    line.match_indices(pn_str.as_str())
                        .map(|(i, _)| (i, i + pn_str.len()))
                        .collect::<Vec<(usize, usize)>>()
                })
                .collect();

            for (start_char, end_char) in indices {
                let start_char = if start_char == 0 { 0 } else { start_char - 1 };
                let range = cmp::max(start_char, 0)..cmp::min(end_char + 1, end);

                match i {
                    // first row
                    0 => line_numbers.push(Number {
                        line: line[range.clone()].to_string(),
                        prev_line: None,
                        next_line: Some(String::from(lines[i + 1])[range].to_string()),
                    }),
                    // last row
                    i if i == end - 1 => line_numbers.push(Number {
                        line: line[range.clone()].to_string(),
                        prev_line: Some(String::from(lines[i - 1])[range].to_string()),
                        next_line: None,
                    }),
                    // rest of rows
                    _ => line_numbers.push(Number {
                        line: line[range.clone()].to_string(),
                        prev_line: Some(String::from(lines[i - 1])[range.clone()].to_string()),
                        next_line: Some(String::from(lines[i + 1])[range].to_string()),
                    }),
                }
            }

            numbers.append(&mut line_numbers);
        }

        numbers
    }
}

fn part_1(path: &str) -> u32 {
    let schematic: String = fs::read_to_string(path).expect("Should be able to read file.");

    schematic
        .trim()
        .to_numbers()
        .into_iter()
        .map(|number| match number.part_number() {
            Some(number) => number,
            None => 0,
        })
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("ERROR: No valid arguments provided. Please provide the data file path.");
        exit(1);
    }

    part_1(&args[1]);
}

#[cfg(test)]
mod tests {

    static DATA_FILE: &str = "data/data.txt";
    static EXAMPLE: &str = "data/example.txt";

    #[test]
    fn example() {
        let result = super::part_1(EXAMPLE);

        assert_eq!(result, 4361);
    }

    #[test]
    fn part_1() {
        let result: u32 = super::part_1(DATA_FILE);

        println!("Part 1 result: {}", result);
        assert!(result < 524360);
    }
}
