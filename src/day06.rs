use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<HashSet<char>> {
    input.split("\n\n").map(
        |group| {
            group.lines().fold(HashSet::new(), |acc, i| {
                i.chars().to_owned().collect::<HashSet<char>>().union(&acc).copied().collect()
            })
        }
    ).collect()
}

#[aoc(day6, part1)] 
fn part1(input: &Vec<HashSet<char>>) -> usize {
    input.iter().map(|h| h.iter().count()).sum()
}
