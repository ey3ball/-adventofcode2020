use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<Vec<HashSet<char>>> {
    input.split("\n\n").map(
        |group| {
            group.lines().map(|i| {
                i.chars().to_owned().collect::<HashSet<char>>()
            }).collect()
        }
    ).collect()
}

#[aoc(day6, part1)]
fn part1(input: &Vec<Vec<HashSet<char>>>) -> usize {
    input.iter().map(
        |group| group.iter()
                     .fold(HashSet::new(),
                           |acc, i| i.union(&acc).copied().collect())
                     .iter().count()
    ).sum()
}

#[aoc(day6, part2)]
fn part2(input: &Vec<Vec<HashSet<char>>>) -> usize {
    input.iter().map(
        |group| group.iter()
                     .fold(('a'..='z').collect(),
                           |acc, i| i.intersection(&acc).copied().collect())
                     .iter().count()
    ).sum()
}
