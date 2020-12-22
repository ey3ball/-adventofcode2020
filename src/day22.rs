use std::collections::VecDeque;

#[aoc_generator(day22)]
pub fn generator(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut parsed =
        input.split("\n\n")
            .map(|player| {
                player.lines()
                    .skip(1)
                    .map(|card| card.parse().unwrap())
            });

    (parsed.next().unwrap().collect(),
     parsed.next().unwrap().collect())
}

#[aoc(day22, part1)]
pub fn part1(input: &(Vec<i64>, Vec<i64>)) -> i64 {
    let mut p1: VecDeque<i64> = input.0.iter().copied().collect();
    let mut p2: VecDeque<i64> = input.1.iter().copied().collect();

    while !p1.is_empty() && !p2.is_empty() {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    let iter_winner = if p1.is_empty() {
        p2.iter()
    } else {
        p1.iter()
    };

    iter_winner.rev()
        .enumerate()
        .map(|(i,v)| (i as i64 + 1) * v)
        .sum()
}
