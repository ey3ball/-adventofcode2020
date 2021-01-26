use regex::Regex;
use std::collections::HashMap;

pub enum Op {
    Masks(u64, u64, u64),
    Assign(u64, u64),
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Vec<Op> {
    let re_bitmask = Regex::new(r"^mask = (?P<bitmask>[X0-1]*)$").unwrap();
    let re_memory = Regex::new(r"^mem.(?P<addr>[0-9]*). = (?P<value>[0-9]*)$").unwrap();

    input
        .lines()
        .map(|x| {
            if re_bitmask.is_match(x) {
                let weird_mask = re_bitmask
                    .captures(x)
                    .and_then(|c| c.name("bitmask").map(|b| b.as_str()))
                    .unwrap();
                let and = u64::from_str_radix(&weird_mask.replace("X", "1"), 2).unwrap();
                let or = u64::from_str_radix(&weird_mask.replace("X", "0"), 2).unwrap();
                let flip = u64::from_str_radix(&weird_mask.replace("1", "0").replace("X", "1"), 2)
                    .unwrap();
                Op::Masks(and, or, flip)
            } else {
                let mem = re_memory.captures(x).unwrap();
                Op::Assign(
                    mem.name("addr")
                        .and_then(|a| a.as_str().parse().ok())
                        .unwrap(),
                    mem.name("value")
                        .and_then(|v| v.as_str().parse().ok())
                        .unwrap(),
                )
            }
        })
        .collect()
}

#[aoc(day14, part1)]
fn part1(input: &Vec<Op>) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();

    let mut mask = input.iter().next().unwrap();

    input.iter().skip(1).for_each(|mem_op| {
        if let Op::Masks(and_mask, or_mask, _) = mask {
            match mem_op {
                Op::Masks(_, _, _) => {
                    mask = mem_op;
                }
                Op::Assign(addr, value) => {
                    mem.insert(*addr, (value & and_mask) | or_mask);
                }
            }
        } else {
            panic!()
        }
    });
    mem.iter().map(|(_k, &v)| v).sum()
}

#[aoc(day14, part2)]
fn part2(input: &Vec<Op>) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();

    let mut mask = input.iter().next().unwrap();

    input.iter().skip(1).for_each(|mem_op| {
        if let Op::Masks(_, or_mask, flip_mask) = mask {
            match mem_op {
                Op::Masks(_, _, _) => {
                    mask = mem_op;
                }
                Op::Assign(addr, value) => {
                    let base_addr = (addr | or_mask) & !flip_mask;
                    let decoded = decode(*flip_mask);

                    decoded.iter().for_each(|dec| {
                        mem.insert(base_addr | dec, *value);
                    });
                }
            }
        } else {
            panic!()
        }
    });
    mem.iter().map(|(_k, &v)| v).sum()
}

fn decode(mut flip_mask: u64) -> Vec<u64> {
    let mut addresses: Vec<u64> = vec![flip_mask];

    while flip_mask != 0 {
        let lowest = flip_mask & (!flip_mask + 1);

        addresses.append(&mut addresses.iter().map(|a| a & !lowest).collect());
        flip_mask &= !lowest;
    }
    addresses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_center() {
        let flip_mask = 0b0110u64;
        let addresses = decode(flip_mask);
        assert_eq!(addresses, vec![0b0110u64, 0b0100u64, 0b0010u64, 0b0000u64]);
    }

    #[test]
    fn test_decode_edge() {
        let flip_mask = 0b1001u64;
        let addresses = decode(flip_mask);
        assert_eq!(addresses, vec![0b1001u64, 0b1000u64, 0b0001u64, 0b0000u64]);
    }
}
