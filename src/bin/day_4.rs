use regex::Regex;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../input/day_4.txt");

fn main() {
    let passports = parse_passports(INPUT);
    let required_fields_validator = RequiredFieldsValidator::new();

    let valid_passports = passports
        .iter()
        .filter(|p| required_fields_validator.valid(p))
        .collect::<Vec<_>>();

    println!(
        "valid passports according to required fields: {}",
        valid_passports.len()
    );

    let rules_validator = RulesValidator::new();

    let valid = valid_passports
        .iter()
        .filter(|p| rules_validator.valid(p))
        .count();

    println!("valid passports according to rules: {}", valid);
}

fn parse_passports(input: &str) -> Vec<Passport> {
    let mut passports = Vec::new();
    let mut passport = Passport::new();

    for line in input.lines() {
        if line.is_empty() {
            passports.push(passport);
            passport = Passport::new();
            continue;
        }

        let kvs = line.split(' ').map(|l| {
            let mut kv = l.split(':');
            (kv.next().unwrap(), kv.next().unwrap())
        });

        for (k, v) in kvs {
            passport.insert(k, v);
        }
    }

    passports.push(passport);
    passports
}

pub struct Passport<'a>(HashMap<&'a str, &'a str>);

impl<'a> Passport<'a> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, k: &'a str, v: &'a str) {
        self.0.insert(k, v);
    }
}

impl<'a> Default for Passport<'a> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Validator {
    fn valid(&self, passport: &Passport) -> bool;
}

struct RequiredFieldsValidator<'a> {
    required_fields: HashSet<&'a str>,
    all_fields: HashSet<&'a str>,
}

impl<'a> RequiredFieldsValidator<'a> {
    pub fn new() -> Self {
        let mut required_fields = HashSet::new();
        required_fields.insert("byr");
        required_fields.insert("iyr");
        required_fields.insert("eyr");
        required_fields.insert("hgt");
        required_fields.insert("hcl");
        required_fields.insert("ecl");
        required_fields.insert("pid");

        let mut all_fields = required_fields.clone();
        all_fields.insert("cid");

        Self {
            required_fields,
            all_fields,
        }
    }
}

impl<'a> Validator for RequiredFieldsValidator<'a> {
    fn valid(&self, passport: &Passport) -> bool {
        let required_fields = self
            .required_fields
            .difference(&passport.0.keys().cloned().collect())
            .next()
            .is_none();
        let all_fields = passport
            .0
            .keys()
            .cloned()
            .collect::<HashSet<&str>>()
            .difference(&self.all_fields)
            .next()
            .is_none();
        required_fields && all_fields
    }
}

struct RulesValidator {
    hgt_regex: Regex,
    hcl_regex: Regex,
    ecl_regex: Regex,
    pid_regex: Regex,
}

impl RulesValidator {
    pub fn new() -> Self {
        Self {
            hgt_regex: Regex::new(r"^(?P<value>[0-9]+)(?P<unit>cm|in)$").unwrap(),
            hcl_regex: Regex::new(r"^#[0-9a-f]{6}$").unwrap(),
            ecl_regex: Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap(),
            pid_regex: Regex::new(r"^[0-9]{9}$").unwrap(),
        }
    }
}

impl Validator for RulesValidator {
    fn valid(&self, passport: &Passport) -> bool {
        for (key, value) in passport.0.iter() {
            let valid = match *key {
                "byr" => value
                    .parse::<u32>()
                    .map(|i| i >= 1920 && i <= 2002)
                    .unwrap_or(false),
                "iyr" => value
                    .parse::<u32>()
                    .map(|i| i >= 2010 && i <= 2020)
                    .unwrap_or(false),
                "eyr" => value
                    .parse::<u32>()
                    .map(|i| i >= 2020 && i <= 2030)
                    .unwrap_or(false),
                "hgt" => match self.hgt_regex.captures(value) {
                    Some(caps) => {
                        let v: i32 = caps["value"].parse().unwrap();
                        match &caps["unit"] {
                            "cm" => v >= 150 && v <= 193,
                            "in" => v >= 59 && v <= 76,
                            _ => false,
                        }
                    }
                    _ => false,
                },
                "hcl" => self.hcl_regex.is_match(value),
                "ecl" => self.ecl_regex.is_match(value),
                "pid" => self.pid_regex.is_match(value),
                _ => true,
            };

            if !valid {
                return false;
            }
        }

        true
    }
}
