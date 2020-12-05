#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(
        |x| x.chars().collect()
    ).collect()
}

fn risky_slope(grid: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
    let mut y: usize = 0;
    let mut trees: usize = 0;

    grid.iter().enumerate().filter_map(|(i,x)| {
        if i % down == 0 {
            Some(x)
        } else {
            None
        }
    }).for_each(|x| {
        if x[y % x.len()] == '#' {
            trees += 1
        }
        y += right
    });
    trees
}

#[aoc(day3, part1)] 
fn part1(grid: &Vec<Vec<char>>) -> usize {
    risky_slope(&grid, 3, 1)
}

#[aoc(day3, part2)] 
fn part2(grid: &Vec<Vec<char>>) -> usize {
    let mut trees = risky_slope(&grid, 1, 1);
    trees *= risky_slope(&grid, 3, 1);
    trees *= risky_slope(&grid, 5, 1);
    trees *= risky_slope(&grid, 7, 1);
    trees * risky_slope(&grid, 1, 2)
}
