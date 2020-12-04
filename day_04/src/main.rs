use std::fs;
use std::collections::HashMap;
use regex::Regex;

//fn read_input(mut filedata: &String) -> () {
//    let input = "input/input.txt";
//    filedata = fs::read_to_string(input).unwrap();
//}

fn parse_input(contents: &str) -> Vec<HashMap<&str, &str>> {
    let mut dict: HashMap<&str, &str> = HashMap::new();
    let mut list: Vec<HashMap<&str, &str>> = Vec::new();
    for line in contents.lines() {
        if line.trim().is_empty() {
            list.push(dict);
            dict = HashMap::new();
            continue;
        }

        line.split(" ").map(|x| {
            let mut split2 = x.splitn(2, ":");
            (split2.next().unwrap(), split2.next().unwrap())
        }).for_each(|(k,v)| {dict.insert(k, v); ()});
    }
    list.push(dict);
    //println!("{:#?}", list);

    list
}

fn has_required_fields(passport: &&HashMap<&str, &str>) -> bool {
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


fn day04_1() {
    let input = "input/input.txt";
    let contents = fs::read_to_string(input).unwrap();
    let parsed = parse_input(&contents);

    let valid = parsed.iter().filter(has_required_fields).count();

    println!("Valid passports {}", valid);
}

fn day04_2() {
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

    let input = "input/input.txt";
    let contents = fs::read_to_string(input).unwrap();
    let parsed = parse_input(&contents);

    let valid = parsed.iter()
        .filter(has_required_fields)
        .filter(|x| check_digits(x["byr"], 1920, 2002))
        .filter(|x| check_digits(x["iyr"], 2010, 2020))
        .filter(|x| check_digits(x["eyr"], 2020, 2030))
        .filter(|x| check_height(x["hgt"]))
        .filter(|x| check_hair(x["hcl"]))
        .filter(|x| check_eyes(x["ecl"]))
        .filter(|x| check_pid(x["pid"]))
        .count();

    println!("Valid passports {}", valid);
}

fn main() {
    day04_1();
    day04_2();
}
