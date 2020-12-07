#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(
        |x| x.chars().collect()
    ).collect()
}

fn risky_slope(grid: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
    let (t, _) = grid
        .iter()
        .step_by(down)
        .fold((0,0), |(trees, y), x| {
            if x[y % x.len()] == '#' {
                (trees + 1, y + right)
            } else {
                (trees, y + right)
            }
        });
    t
}

#[aoc(day3, part1)] 
fn part1(grid: &Vec<Vec<char>>) -> usize {
    risky_slope(&grid, 3, 1)
}

#[aoc(day3, part2)] 
fn part2(grid: &Vec<Vec<char>>) -> usize {
    [(1,1),(3,1),(5,1),(7,1),(1,2)]
        .iter()
        .map(|(x,y)| risky_slope(grid, *x, *y))
        .product()
}
