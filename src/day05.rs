use std::collections::HashSet;

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|x| {
            x.replace("R", "1")
                .replace("L", "0")
                .replace("F", "0")
                .replace("B", "1")
        })
        .map(|x| usize::from_str_radix(&x, 2).unwrap())
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &Vec<usize>) -> usize {
    input.iter().max().unwrap().clone()
}

#[aoc(day5, part2)]
fn part2(input: &Vec<usize>) -> usize {
    let mut seats = (0..864).collect::<HashSet<usize>>();

    input.iter().for_each(|x| {
        seats.remove(x);
        ()
    });
    seats
        .iter()
        .filter(|x| !seats.contains(&(**x + 1)) && !seats.contains(&(**x - 1)))
        .next()
        .unwrap()
        .clone()
}
