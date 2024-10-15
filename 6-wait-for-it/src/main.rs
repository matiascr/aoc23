use std::fs;

pub const DATA_FILE: &str = "data/data.txt";
pub const EXAMPLE_FILE: &str = "data/example.txt";

struct Race {
    duration: usize,
    record_distance: usize,
}
type Races = Vec<Race>;

fn get_races(file_content: String) -> Races {
    let lines: Vec<Vec<&str>> = file_content
        .lines()
        .into_iter()
        .map(str::split_whitespace)
        .map(Iterator::collect)
        .collect();

    (1..lines[0].len())
        .map(|item| Race {
            duration: lines[0][item].parse().unwrap(),
            record_distance: lines[1][item].parse().unwrap(),
        })
        .collect()
}

fn beat(race: &Race) -> usize {
    let mut amount = 0;
    for d in 0..=race.duration {
        if d * (race.duration - d) > race.record_distance {
            amount += 1;
        }
    }
    amount
}

fn part_1(runner: &str, file_path: &str) -> usize {
    let file_content: String = fs::read_to_string(file_path).expect("should be able to read file.");
    let races: Races = get_races(file_content);
    let results: Vec<usize> = races.iter().map(beat).collect();
    let result = results.iter().fold(1, std::ops::Mul::mul);
    println!("\n==> {} result: {:?}", runner, result);
    result
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn part_1() {
        let result = super::part_1("Part 1", super::DATA_FILE);
        assert_eq!(result, 170000);
    }

    #[test]
    fn example() {
        let result = super::part_1("Example", super::EXAMPLE_FILE);
        assert_eq!(result, 288);
    }
}

fn main() {
    let mut result: usize;
    result = part_1("Part 1", DATA_FILE);
    assert_eq!(result, 170000);

    result = part_1("Example", EXAMPLE_FILE);
    assert_eq!(result, 288);
}
