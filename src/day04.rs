use std::collections::HashMap;
use regex::Regex;

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<HashMap<String, String>> {
    let mut dict: HashMap<String, String> = HashMap::new();
    let mut list: Vec<HashMap<String, String>> = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            list.push(dict);
            dict = HashMap::new();
            continue;
        }

        line.split(" ").map(|x| {
            let mut split2 = x.splitn(2, ":");
            (split2.next().unwrap().to_owned(),
             split2.next().unwrap().to_owned())
        }).for_each(|(k,v)| {dict.insert(k, v); ()});
    }
    list.push(dict);
    list
}

fn has_required_fields(passport: &&HashMap<String, String>) -> bool {
    (passport.keys().count() == 8)
    || ((passport.keys().count() == 7)
            && passport.keys().find(|y| **y == "cid").is_none()
    )
}

fn check_digits(input: &str, min: usize, max: usize) -> bool {
    let re = Regex::new(r"^\d{4}$").unwrap();
    let found = re.find(input);
    found.is_some()
        && found.unwrap().as_str().parse::<usize>().unwrap() >= min
        && found.unwrap().as_str().parse::<usize>().unwrap() <= max
}

fn check_height(input: &str) -> bool {
    let re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    let captures = re.captures(input);
    if captures.is_some() {
        let captures = captures.unwrap();
        let height = captures[1].parse::<usize>().unwrap();
        match captures[2].as_ref() {
            "cm" => height >= 150 && height <= 193,
            "in" => height >= 59 && height <= 76,
            _ => false
        }
    } else {
        false
    }
}

fn check_hair(input: &str) -> bool {
    let re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    re.find(input).is_some()
}

fn check_eyes(input: &str) -> bool {
    vec!["amb", "blu", "brn", "gry", "grn", "hzl" ,"oth"].contains(&input)
}

fn check_pid(input: &str) -> bool {
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    re.find(input).is_some()
}

#[aoc(day4, part1)]
pub fn part1(input: &Vec<HashMap<String, String>>) -> usize {
    input.iter().filter(has_required_fields).count()
}

#[aoc(day4, part2)]
pub fn part2(input: &Vec<HashMap<String, String>>) -> usize {
    assert!(check_digits("2020", 2010, 2030));
    assert!(!check_digits("2031", 2010, 2030));
    assert!(!check_digits("2009", 2010, 2030));
    assert!(check_height("60in"));
    assert!(check_height("190cm"));
    assert!(!check_height("190in"));
    assert!(!check_height("190"));
    assert!(check_eyes("gry"));
    assert!(!check_eyes("gray"));
    assert!(check_hair("#abcd67"));
    assert!(!check_hair("abcd67"));
    assert!(!check_hair("abcd6k"));
    assert!(!check_pid("1234567810"));
    assert!(!check_pid("12345678"));
    assert!(check_pid("123456789"));

    input.iter()
        .filter(has_required_fields)
        .filter(|x| check_digits(&x["byr"], 1920, 2002))
        .filter(|x| check_digits(&x["iyr"], 2010, 2020))
        .filter(|x| check_digits(&x["eyr"], 2020, 2030))
        .filter(|x| check_height(&x["hgt"]))
        .filter(|x| check_hair(&x["hcl"]))
        .filter(|x| check_eyes(&x["ecl"]))
        .filter(|x| check_pid(&x["pid"]))
        .count()
}
