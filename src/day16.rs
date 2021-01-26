use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Tickets {
    rules: Vec<Rule>,
    mine: Vec<u32>,
    nearby: Vec<Vec<u32>>,
}

type Rule = (String, (u32, u32), (u32, u32));

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Tickets {
    let mut iter = input.lines();
    let re_ranges = Regex::new(" ([0-9]+)[-]([0-9]+) or ([0-9]+)[-]([0-9]+)").unwrap();

    let rules: Vec<Rule> = iter
        .by_ref()
        .take_while(|x| x != &"")
        .map(|x| {
            let captures = re_ranges.captures(x).unwrap();
            let r1: u32 = captures[1].parse().unwrap();
            let r2: u32 = captures[2].parse().unwrap();
            let r3: u32 = captures[3].parse().unwrap();
            let r4: u32 = captures[4].parse().unwrap();
            let name = x.split(":").next().unwrap();
            (name.to_owned(), (r1, r2), (r3, r4))
        })
        .collect();

    let mine: Vec<u32> = iter
        .by_ref()
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(|x| {
            println!("{}", x);
            x.parse().unwrap()
        })
        .collect();

    let nearby: Vec<Vec<u32>> = iter
        .skip(2)
        .map(|line| line.split(",").map(|f| f.parse().unwrap()).collect())
        .collect();

    Tickets {
        rules,
        mine,
        nearby,
    }
}

fn valid_field(v: u32, rules: &Vec<Rule>) -> bool {
    rules
        .iter()
        .any(|r| (r.1 .0..=r.1 .1).contains(&v) || (r.2 .0..=r.2 .1).contains(&v))
}

#[aoc(day16, part1)]
fn part1(input: &Tickets) -> u32 {
    input
        .nearby
        .iter()
        .flat_map(|ticket| ticket.iter().filter(|v| !valid_field(**v, &input.rules)))
        .copied()
        .sum()
}

fn pass(v: u32, r: &Rule) -> bool {
    (r.1 .0..=r.1 .1).contains(&v) || (r.2 .0..=r.2 .1).contains(&v)
}

#[aoc(day16, part2)]
fn part2(input: &Tickets) -> u64 {
    let valid: Vec<Vec<u32>> = input
        .nearby
        .iter()
        .filter(|ticket| !ticket.iter().any(|v| !valid_field(*v, &input.rules)))
        .cloned()
        .collect();

    let mut fields: HashMap<&str, HashSet<usize>> = HashMap::new();
    for (_, r) in input.rules.iter().enumerate() {
        fields.insert(r.0.as_str(), HashSet::new());
    }

    // For all rules
    for r in input.rules.iter() {
        // Try all fields
        for j in 0..input.rules.len() {
            // Find field that passes all tests
            if valid.iter().all(|t| pass(t[j], r)) {
                fields.get_mut(r.0.as_str()).unwrap().insert(j);
            }
        }
    }

    let mut i = 0;
    loop {
        let keys: Vec<&str> = fields.keys().copied().collect();
        for k in keys.iter() {
            if fields[k].len() == 1 {
                let remove = fields[k].iter().next().unwrap().clone();
                fields
                    .iter_mut()
                    .filter(|(l, _)| *l != k)
                    .for_each(|(_, v)| {
                        v.remove(&remove);
                    });
            }
        }
        if (i >= 10) {
            break;
        }
        i = i + 1;
    }
    println!("valid: {:#?}", fields);
    let indexes: HashSet<usize> = fields
        .iter()
        .filter(|(k, v)| k.starts_with("departure"))
        .map(|(_, v)| v.iter().next().unwrap())
        .copied()
        .collect();
    println!("indexes: {:#?}", indexes);

    let values: Vec<u32> = indexes.iter().map(|i| input.mine[*i]).collect();

    println!("values: {:#?}", values);

    input
        .mine
        .iter()
        .enumerate()
        .filter(|(i, x)| indexes.contains(i))
        .map(|(i, x)| *x as u64)
        .product()
}
