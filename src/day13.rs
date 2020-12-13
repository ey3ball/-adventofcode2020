use itertools::sorted;

#[aoc_generator(day13, part1)]
pub fn generator(input: &str) -> (i64, Vec<i64>) {
    let mut iter = input.lines();
    let departure = iter.next().unwrap().parse::<u32>().unwrap() as i64;
    let buses = iter.next().unwrap()
                    .split(",")
                    .filter(|&x| x != "x")
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect();
    (departure, buses)
}

#[aoc_generator(day13, part2)]
pub fn generator2(input: &str) -> Vec<(i64, i64)> {
    let mut iter = input.lines();
    iter.skip(1).next().unwrap()
        .split(",")
        .enumerate()
        .filter(|(i, x)| *x != "x")
        .map(|(i, x)| (i as i64, x.parse::<i64>().unwrap()))
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(input: &(i64, Vec<i64>)) -> i64 {
    println!("{:#?}", input);
    let (departure, buses) = input;
    let (delta, bus) = sorted(buses.iter().map(|x| {
        let loop_count = (departure / x) + 1;
        let timedelta = (loop_count * x) - departure;
        (timedelta, x)
    })).next().unwrap();
    delta * bus
}

// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

#[aoc(day13, part2)]
pub fn part2(input: &Vec<(i64, i64)>) -> i64 {
    println!("{:#?}", input);
    let remainders = input.iter().map(|(i, x)| {
        let mut r : i64 = *i;
        while r > 0i64 {
            r -= x
        }
        -r
    }).collect::<Vec<i64>>();
    let modulii = input.iter().map(|(_, x)| *x).collect::<Vec<i64>>();
    chinese_remainder(&remainders[..], &modulii[..]).unwrap()
}
