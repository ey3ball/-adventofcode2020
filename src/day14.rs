use regex::Regex;
use std::collections::HashMap;

type Inst = (char, u64, u64);
type Inst2 = (String, u64, u64);

#[aoc_generator(day14, part1)]
pub fn generator(input: &str) -> Vec<Inst> {
    let re_bitmask = Regex::new(r"^mask = (?P<bitmask>[X0-1]*)$").unwrap();
    let re_memory = Regex::new(r"^mem.(?P<addr>[0-9]*). = (?P<value>[0-9]*)$").unwrap();

    input.lines().map(|x|
        if re_bitmask.is_match(x) {
            let mask = re_bitmask.captures(x).unwrap();
            let and = u64::from_str_radix(
                &mask.name("bitmask").unwrap().as_str().replace("X", "1"),
                2
            ).unwrap();
            let or = u64::from_str_radix(
                &mask.name("bitmask").unwrap().as_str().replace("X", "0"),
                2
            ).unwrap();
            ('X', and, or)
        } else {
            let mem = re_memory.captures(x).unwrap();
            ('M', mem.name("addr").unwrap().as_str().parse().unwrap(),
                  mem.name("value").unwrap().as_str().parse().unwrap()
            )
        }
    ).collect()
}

#[aoc_generator(day14, part2)]
pub fn generator2(input: &str) -> Vec<Inst2> {
    let re_bitmask = Regex::new(r"^mask = (?P<bitmask>[X0-1]*)$").unwrap();
    let re_memory = Regex::new(r"^mem.(?P<addr>[0-9]*). = (?P<value>[0-9]*)$").unwrap();

    input.lines().map(|x|
        if re_bitmask.is_match(x) {
            let mask = re_bitmask.captures(x).unwrap().name("bitmask").unwrap().as_str().to_owned();
            (mask, 0, 0)
        } else {
            let mem = re_memory.captures(x).unwrap();
            ("M".to_owned(), mem.name("addr").unwrap().as_str().parse().unwrap(),
                  mem.name("value").unwrap().as_str().parse().unwrap()
            )
        }
    ).collect()
}

#[aoc(day14, part1)]
fn part1(input: &Vec<Inst>) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();

    let (_, and, or) = input.iter().next().unwrap();

    let res = input.iter().skip(1)
         .fold((and, or), |(and, or), (t, a1, a2)| {
        if *t == 'X' {
            (a1, a2)
        } else {
            mem.insert(*a1, (a2 & and) | or);
            (and, or)
        }
    });
    mem.iter().map(|(_k,&v)| v).sum()
}

#[aoc(day14, part2)]
fn part2(input: &Vec<Inst2>) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();

    let (mask, _, _) = input.iter().next().unwrap();

    let res = input.iter().skip(1)
         .fold(mask, |mask, (t, a1, a2)| {
            if t == "M" {
                let positions: Vec<u64> = mask.bytes().enumerate().filter_map(|(i, x)| {
                    if x == 'X' as u8 {
                        Some(i as u64)
                    } else {
                        None
                    }
                }).collect();
                let count = positions.len();
                let addresses = (0..1 << count).map(
                    |i| {
                        let mut bin: Vec<u8> = mask.bytes().collect();
                        for (n, p) in positions.iter().enumerate() {
                            bin[*p as usize] = ((i >> n) & 0x01) as u8 + '0' as u8
                        }
                        u64::from_str_radix(String::from_utf8(bin).unwrap().as_str(), 2).unwrap()
                    }
                );
                let bitmask = !addresses.clone().last().unwrap();
                addresses.for_each(|x| {mem.insert(a1 & bitmask | x, *a2);});
                mask
            } else {
                t
            }
    });
    mem.iter().map(|(_k,&v)| v).sum()
}
