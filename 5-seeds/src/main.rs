use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;

pub const DATA_FILE: &str = "data/data.txt";
pub const EXAMPLE_FILE: &str = "data/example.txt";

type Seed = isize;
type Location = Seed;
type Seeds = Vec<Seed>;
type Locations = Vec<Location>;
type Map_ = HashMap<RangeInclusive<Seed>, Seed>;
type Maps = Vec<Map_>;

struct Almanac {
    seeds: Seeds,
    maps: Maps,
}

trait Traversable {
    fn traverse(&self) -> Locations;
}

impl Traversable for Almanac {
    fn traverse(&self) -> Locations {
        assert!(!self.maps.is_empty());
        assert!(!self.seeds.is_empty());

        let mut soils = self.seeds.clone();

        for map in self.maps.clone() {
            let mut new_soils: Locations = Vec::new();

            soils
                .iter()
                .for_each(|&soil| new_soils.push(get(map.clone(), soil)));

            assert_eq!(
                new_soils.len(),
                soils.len(),
                "soils: {:?} map: {:?}",
                soils,
                map
            );
            soils = new_soils;
        }
        assert!(!soils.is_empty(), "soils: {:?}", soils);
        soils
    }
}

fn get(map: Map_, seed: Seed) -> Seed {
    seed + if let Some(range) = map.keys().filter(|&key| key.contains(&seed)).next() {
        map.get(range).cloned().unwrap()
    } else {
        0
    }
}

impl From<String> for Almanac {
    fn from(almanac: String) -> Self {
        Almanac {
            seeds: get_seeds(&almanac),
            maps: get_maps(&almanac),
        }
    }
}

fn get_seeds(almanac_lines: &str) -> Seeds {
    let seeds_line = almanac_lines.lines().collect::<Vec<&str>>()[0];
    let trailing_text = seeds_line.trim_start_matches("seeds: ");

    assert!(
        trailing_text.replace(" ", "").chars().all(char::is_numeric),
        "trailing_text = {}",
        trailing_text
    );

    trailing_text
        .split_whitespace()
        .map(|number_str| number_str.parse::<Seed>().unwrap())
        .collect::<Seeds>()
}

fn get_maps(almanac_lines: &str) -> Maps {
    let seeds_lines_removed = almanac_lines.lines().collect::<Vec<&str>>()[2..].to_vec();
    assert!(!seeds_lines_removed.is_empty());

    seeds_lines_removed
        .join("\n")
        .split("\n\n")
        .map(|map_lines| create_map(map_lines))
        .collect::<Maps>()
}

fn create_map(map_lines: &str) -> Map_ {
    let map_lines_vec = map_lines.lines().collect::<Vec<&str>>();
    let title_line = map_lines_vec[0];
    let number_lines = map_lines_vec[1..].to_vec();

    assert!(
        title_line.replace(" ", "").chars().all(|c| !c.is_numeric()),
        "title_line: {}",
        title_line
    );
    assert!(
        number_lines
            .concat()
            .replace(" ", "")
            .chars()
            .all(char::is_numeric),
        "number_lines: {:?}",
        number_lines
    );

    let mut maps: Map_ = HashMap::new();

    number_lines.iter().for_each(|&line| {
        let map_line = match line
            .split_whitespace()
            .map(|n| n.parse::<Seed>().unwrap())
            .collect::<Vec<Seed>>()[..]
        {
            [dest, source, range] => Some((source..=(source + range - 1), dest - source)),
            _ => None,
        };

        match map_line.unwrap() {
            (range, jump) => maps.insert(range, jump),
        };
    });
    maps
}

pub fn part_1(runner: &str, file_path: &str) -> Location {
    let file_content = fs::read_to_string(file_path).expect("should be able to read file.");

    let almanac = Almanac::from(file_content);
    let soils = almanac.traverse();
    assert!(!soils.is_empty(), "soils: {:?}", soils);

    let lowest_seed = soils.into_iter().reduce(isize::min).unwrap();
    println!("\n==> {} result: {:?}", runner, lowest_seed);
    lowest_seed
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn part_1() {
        let lowest_seed = super::part_1("Part 1", super::DATA_FILE);
        assert_eq!(lowest_seed, 318728750);
    }

    #[test]
    fn example() {
        let lowest_seed = super::part_1("Example", super::EXAMPLE_FILE);
        assert_eq!(lowest_seed, 35);
    }
}

pub fn main() {
    let mut lowest_seed: Seed;
    lowest_seed = part_1("Part 1", DATA_FILE);
    assert_eq!(lowest_seed, 318728750);

    lowest_seed = part_1("Example", EXAMPLE_FILE);
    assert_eq!(lowest_seed, 35);
}
