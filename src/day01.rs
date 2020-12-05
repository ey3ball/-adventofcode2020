#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<u32> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1)] 
pub fn part1(expenses: &Vec<u32>) -> u32 {
    let mut solution: u32 = 0;
    for n1 in expenses {
        for n2 in expenses {
            if n1 + n2 == 2020 {
                solution = n1 * n2;
            }
        }
    }
    solution
}

#[aoc(day1, part2)] 
pub fn part2(expenses: &Vec<u32>) -> u32 {
    let mut solution: u32 = 0;
    for n1 in expenses {
        for n2 in expenses {
            for n3 in expenses {
                if n1 + n2 + n3 == 2020 {
                    solution = n1 * n2 * n3
                }
            }
        }
    }
    solution
}
