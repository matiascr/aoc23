use std::{cmp, env, fmt::Display, fs};

struct Schematic {
    lines: Vec<String>,
}

struct Number {
    prev_line: Option<String>,
    line: String,
    next_line: Option<String>,
    number: u32,
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(s) = self.prev_line.clone() {
            writeln!(f, "{}", s)?
        }

        writeln!(f, "{}", self.line)?;

        if let Some(s) = self.next_line.clone() {
            writeln!(f, "{}", s)?
        }

        Ok(())
    }
}

impl Schematic {
    /// Creates a new [`Schematic`] from a [`String`].
    fn new(schematic: String) -> Self {
        Self {
            lines: schematic.trim().split('\n').map(String::from).collect(),
        }
    }

    /// Creates a [`Vec<Number>`] from a [`Schematic`].
    fn to_numbers(&self) -> Vec<Number> {
        let n_lines = self.lines.len();
        let lines_len = self.lines[0].len();

        let mut current: Vec<usize> = Vec::new();
        let mut numbers: Vec<Number> = Vec::new();

        for i in 0..n_lines {
            let line = self.lines[i].clone();
            for j in 0..lines_len {
                let char_vec = line.chars().collect::<Vec<char>>();
                let current_char = char_vec[j];
                if current_char.is_numeric() {
                    match j {
                        // is first
                        0 => current.push(j),
                        // is last
                        j if j == lines_len - 1 => current.push(j),
                        // is single digit
                        j if !char_vec[j - 1].is_numeric() && !char_vec[j + 1].is_numeric() => {
                            current.push(j);
                            current.push(j);
                        }
                        // is is last or first digit in sequence
                        j if !current.contains(&(j - 1)) || !char_vec[j + 1].is_numeric() => {
                            current.push(j)
                        }
                        _ => {}
                    }
                }
            }

            if !current.is_empty() {
                let starts: Vec<&usize> = current.iter().step_by(2).collect::<Vec<&usize>>();
                let ends: Vec<&usize> = current[1..].iter().step_by(2).collect::<Vec<&usize>>();

                let line_indices = starts.iter().zip(ends).collect::<Vec<(&&usize, &usize)>>();

                for (&&s, &e) in line_indices {
                    let line_range =
                        (cmp::max(s as isize - 1, 0) as usize)..=cmp::min(e + 1, lines_len - 1);
                    let current_line_number: String =
                        self.lines[i].as_str()[line_range.clone()].to_string();
                    match i {
                        0 => numbers.push(Number::new(
                            None,
                            current_line_number,
                            Some((self.lines[i + 1].as_str())[line_range.clone()].to_string()),
                        )),
                        ii if ii == n_lines - 1 => numbers.push(Number::new(
                            Some((self.lines[i - 1].as_str())[line_range.clone()].to_string()),
                            current_line_number,
                            None,
                        )),

                        _ => numbers.push(Number::new(
                            Some((self.lines[i - 1].as_str())[line_range.clone()].to_string()),
                            current_line_number,
                            Some((self.lines[i + 1].as_str())[line_range.clone()].to_string()),
                        )),
                    }
                }
                // number_indices.push((i, current));
                current = Vec::new();
            }
        }

        numbers
    }
}

impl Number {
    /// Creates a new [`Number`].
    fn new(prev_line: Option<String>, line: String, next_line: Option<String>) -> Self {
        Self {
            prev_line,
            line: line.clone(),
            next_line,
            number: line
                .split(|c: char| c.is_ascii_punctuation())
                .collect::<String>()
                .parse::<u32>()
                .unwrap(),
        }
    }

    /// Returns if this [`Number`] is part number.
    fn part_number(&self) -> Option<u32> {
        // has if there is no punctuation
        let prev_has = matches!(&self.prev_line, Some(s) if !s.replace('.', "").is_empty());
        let next_has = matches!(&self.next_line, Some(s) if !s.replace('.', "").is_empty());

        let line_has = !&self
            .line
            .chars()
            .filter(|c| !c.is_ascii_digit() && *c != '.')
            .collect::<String>()
            .is_empty();

        if !prev_has && !line_has && !next_has {
            return None;
        }

        Some(self.number)
    }
}

fn part1(schematic_string: String) -> u32 {
    Schematic::new(schematic_string)
        .to_numbers()
        .iter()
        .filter_map(|n| n.part_number())
        .sum()
}

fn main() {
    let schematic = fs::read_to_string(&env::args().collect::<Vec<String>>()[1])
        .expect("should be able to read file.");

    let result = part1(schematic);
    println!("Result of Part 1: {}", result);
}

#[cfg(test)]
pub mod tests {
    use std::fs;

    static DATA_FILE: &str = "data/data.txt";
    static EXAMPLE: &str = "data/example.txt";

    #[test]
    fn example() {
        let schematic = fs::read_to_string(EXAMPLE).expect("should be able to read file.");

        let result = super::part1(schematic);

        println!("\n==> Example result: {}\n", result);
        assert_eq!(result, 4361);
    }

    #[test]
    fn part_1() {
        let schematic = fs::read_to_string(DATA_FILE).expect("Should be able to read file.");

        let result = super::part1(schematic);

        println!("\n==> Part 1 result: {}\n", result);
        assert_eq!(result, 509115);
    }
}
