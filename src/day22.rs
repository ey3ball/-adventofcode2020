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

#[aoc(day22, part2)]
pub fn part2(input: &(Vec<i64>, Vec<i64>)) -> i64 {
    let mut p1: VecDeque<i64> = input.0.iter().copied().collect();
    let mut p2: VecDeque<i64> = input.1.iter().copied().collect();

    let (_, winner_deck) = combat(1, p1, p2);

    winner_deck.iter()
        .rev()
        .enumerate()
        .map(|(i,v)| (i as i64 + 1) * v)
        .sum()
}

pub fn combat(j: i64, mut p1: VecDeque<i64>, mut p2: VecDeque<i64>) -> (bool, VecDeque<i64>) {
    let mut p1_decks: Vec<VecDeque<i64>> = Vec::new();
    let mut p2_decks: Vec<VecDeque<i64>> = Vec::new();

    let mut i = 0;
    while !p1.is_empty() && !p2.is_empty() {
        if p1_decks.contains(&p1) || p2_decks.contains(&p2) {
            return (true, p1)
        } else {
            p1_decks.push(p1.clone());
            p2_decks.push(p2.clone());
        }

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        let p1_wins = if c1 <= p1.len() as i64 && c2 <= p2.len() as i64 {
            let (p1_wins, _) = combat(
                    j + 1,
                    p1.iter().take(c1 as usize).copied().collect(),
                    p2.iter().take(c2 as usize).copied().collect()
            );
            p1_wins
        } else {
            c1 > c2
        };


        if p1_wins {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
        i = i + 1
    }

    if p1.is_empty() {
        (false, p2)
    } else {
        (true, p1)
    }
}
