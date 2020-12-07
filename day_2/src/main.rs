use once_cell::sync::Lazy;
use regex::Regex;

static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?P<min>\d+)\-(?P<max>\d+)\s(?P<char>\w):\s(?P<password>.*)$").unwrap()
});

const INPUT: &str = include_str!("input.txt");

fn main() {
    let lines = INPUT.lines().collect::<Vec<&str>>();

    let valid = lines
        .iter()
        .filter(|line| {
            let (min, max, char, password) = parse(line);
            let policy = SledRentalPasswordPolicy::new(min, max, char);
            policy.valid(&password)
        })
        .count();

    println!("sled rental policy {} valid out of {}", valid, lines.len());

    let valid = lines
        .iter()
        .filter(|line| {
            let (first_pos, second_pos, char, password) = parse(line);
            let policy = TobogganPasswordPolicy::new(first_pos, second_pos, char);
            policy.valid(&password)
        })
        .count();

    println!("toboggan policy {} valid out of {}", valid, lines.len());
}

fn parse(line: &str) -> (usize, usize, char, String) {
    let caps = REGEX.captures(line).unwrap();
    let min: usize = caps["min"].parse::<usize>().unwrap();
    let max: usize = caps["max"].parse::<usize>().unwrap();
    let char = caps["char"].chars().next().unwrap();
    (min, max, char, caps["password"].to_string())
}

struct SledRentalPasswordPolicy {
    min: usize,
    max: usize,
    char: char,
}

impl SledRentalPasswordPolicy {
    pub fn new(min: usize, max: usize, char: char) -> Self {
        SledRentalPasswordPolicy { min, max, char }
    }

    pub fn valid(&self, password: &str) -> bool {
        let count = password.chars().filter(|c| c == &self.char).count();

        count >= self.min && count <= self.max
    }
}

struct TobogganPasswordPolicy {
    first_index: usize,
    second_index: usize,
    char: char,
}

impl TobogganPasswordPolicy {
    pub fn new(first_pos: usize, second_pos: usize, char: char) -> Self {
        TobogganPasswordPolicy {
            first_index: first_pos - 1,
            second_index: second_pos - 1,
            char,
        }
    }

    pub fn valid(&self, password: &str) -> bool {
        let first_pos = password.chars().nth(self.first_index).unwrap();
        let second_pos = password.chars().nth(self.second_index).unwrap();
        match (first_pos == self.char, second_pos == self.char) {
            (true, false) => true,
            (false, true) => true,
            _ => false,
        }
    }
}
