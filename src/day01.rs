use itertools::Itertools;

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<u32> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(expenses: &Vec<u32>) -> u32 {
    for (i, n1) in expenses.iter().enumerate() {
        for n2 in expenses.iter().skip(i + 1) {
            if n1 + n2 == 2020 {
                return n1 * n2;
            }
        }
    }
    return 0
}

#[aoc(day1, part2)]
pub fn part2(expenses: &Vec<u32>) -> u32 {
    for (i, n1) in expenses.iter().enumerate() {
        for (j, n2) in expenses.iter().enumerate().skip(i + 1) {
            for n3 in expenses.iter().skip(j) {
                if n1 + n2 + n3 == 2020 {
                    return n1 * n2 * n3
                }
            }
        }
    }
    return 0
}

#[aoc(day1, part1, itertools)]
pub fn part1_iter(expenses: &Vec<u32>) -> u32 {
    expenses.iter().tuple_combinations()
                   .filter(|(&n1,&n2)| n1 + n2 == 2020)
                   .map(|(n1, n2)| n1 * n2)
                   .next()
                   .unwrap()
}

#[aoc(day1, part2, itertools)]
pub fn part2_iter(expenses: &Vec<u32>) -> u32 {
    expenses.iter().tuple_combinations()
                   .filter(|(&n1,&n2,&n3)| n1 + n2 + n3 == 2020)
                   .map(|(&n1,&n2,&n3)| n1 * n2 * n3)
                   .next()
                   .unwrap()
}
