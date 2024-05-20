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
            let char_vec = line.chars().collect::<Vec<char>>();
            for j in 0..lines_len {
                if char_vec[j].is_numeric() {
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
                            current.push(j);
                        }
                        _ => {}
                    }
                }
            }

            if !current.is_empty() {
                let (line_indices, _) =
                    current
                        .iter()
                        .fold((Vec::new(), None), |(mut lis, n), c| match n {
                            None => (lis, Some(c)),
                            Some(n_) => {
                                lis.push((*n_, *c));
                                (lis, None)
                            }
                        });

                for (s, e) in line_indices {
                    let range_start = cmp::max(s as isize - 1, 0) as usize;
                    let range_end = cmp::min(e + 1, lines_len - 1);
                    let line_range = range_start..=range_end;
                    let current_line_number =
                        self.lines[i].as_str()[line_range.clone()].to_string();
                    match i {
                        0 => numbers.push(Number::new(
                            None,
                            current_line_number,
                            Some((self.lines[i + 1].as_str())[line_range.clone()].to_string()),
                        )),
                        i if i == n_lines - 1 => numbers.push(Number::new(
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
        if !has_punctuation(self.prev_line.clone())
            && !has_punctuation(self.next_line.clone())
            && !has_punctuation(Some(self.line.clone()))
        {
            return None;
        }

        Some(self.number)
    }
}

/// Determines if a given [`String`] any symbols other than periods (".").
fn has_punctuation(line: Option<String>) -> bool {
    matches!(line, Some(s) if !s.replace('.', "").replace(char::is_numeric, "").is_empty())
}

/// Runs the part 1 implementation.
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
