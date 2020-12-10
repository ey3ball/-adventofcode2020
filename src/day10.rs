use std::collections::HashMap;
use std::collections::VecDeque;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<i64> {
    let mut parsed: Vec<i64> =
        input.lines().map(|x| x.parse().unwrap()).collect();
    parsed.sort();
    let device = parsed.iter().last().unwrap() + 3;
    parsed.push(device);
    parsed
}

#[aoc(day10, part1)]
pub fn part1(input: &Vec<i64>) -> u64 {
    let jolts = input.iter().scan(0, |acc, &x| {
        let prev = *acc;
        *acc = x;
        Some(x - prev)
    });
    let one = jolts.clone().filter(|&x| x == 1).count();
    let three = jolts.clone().filter(|&x| x == 3).count();
    (one * three) as u64
}


#[aoc(day10, part2)]
pub fn part2(input: &Vec<i64>) -> i64 {
    let mut previous = VecDeque::<i64>::new();
    previous.push_back(-4);
    previous.push_back(-4);
    previous.push_back(0);

    // Count the number of ways to reach consecutive power levels
    let mut paths_counter = HashMap::<i64, i64>::new();
    paths_counter.insert(-4, 1);
    paths_counter.insert(0, 1);

    input.iter().map(|&x| {
        let paths = if x - previous[2] == 3 {
            paths_counter[&previous[2]]
        } else if x - previous[1] >= 3 {
            paths_counter[&previous[2]]
        } else if previous.iter().all(|&p| x - p <= 3) {
            previous.iter().map(|x| paths_counter[x]).sum()
        } else if (2*x - previous[1] - previous[2]) == 3 {
            previous.iter().skip(1).map(|x| paths_counter[x]).sum()
        } else {
            panic!("No !");
        };
        previous.push_back(x);
        previous.pop_front();
        paths_counter.insert(x, paths);
        paths
    }).last().unwrap()
}
