use std::fs;
use std::collections::HashMap;

//fn read_input(mut filedata: &String) -> () {
//    let input = "input/input.txt";
//    filedata = fs::read_to_string(input).unwrap();
//}

fn parse_input(contents: &str) -> Vec<HashMap<&str, &str>> {
    let mut dict: HashMap<&str, &str> = HashMap::new();
    let mut list: Vec<HashMap<&str, &str>> = Vec::new();
    for line in contents.lines() {
        println!("{}", line);
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
    println!("{:#?}", list);

    list
}

fn day04_1() {
    let input = "input/input.txt";
    let contents = fs::read_to_string(input).unwrap();
    let parsed = parse_input(&contents);

    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let valid = parsed.iter().filter(|x| {
        let is_valid = (x.keys().count() == 8)
            || ((x.keys().count() == 7) && x.keys().find(|y| **y == "cid").is_none());
        println!("{}", is_valid);
        is_valid
    }).count();

    println!("Valid passports {}", valid);
}

fn main() {
    day04_1();
    println!("Hello, world!");
}
