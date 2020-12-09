use std::collections::VecDeque;
use std::collections::HashSet;

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<u64> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &Vec<u64>) -> u64 {
    let mut preamble: VecDeque<_> = input.iter().take(25).copied().collect();

    *input.iter().skip(25).find(|&cur| {
        let valid: HashSet<_> = preamble.iter().copied().collect();

        let checked = valid.iter().find(|&prev| {
            (&valid - &[*prev].iter().copied().collect::<HashSet<_>>()).contains(&(cur - prev))
        });
        preamble.pop_front();
        preamble.push_back(*cur);
        checked.is_none()
    }).unwrap()
}
