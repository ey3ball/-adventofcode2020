use std::fs;

fn parse_input() -> Vec<Vec<char>> {
    let input = "input/input.txt";
    let contents = fs::read_to_string(input).unwrap();

    let grid: Vec<Vec<char>> = contents.lines().map(
        |x| x.chars().collect()
    ).collect();
    grid
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

fn day_03_1() {
    let grid = parse_input();
    let trees = risky_slope(&grid, 3, 1);

    println!("part1: {:#?}", trees);
}

fn day_03_2() {
    let grid = parse_input();

    let mut trees = risky_slope(&grid, 1, 1);
    trees *= risky_slope(&grid, 3, 1);
    trees *= risky_slope(&grid, 5, 1);
    trees *= risky_slope(&grid, 7, 1);
    trees *= risky_slope(&grid, 1, 2);

    println!("part2: {:#?}", trees);
}

fn main() {
    day_03_1();
    day_03_2();
}
