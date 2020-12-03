use std::fs;

fn day_03_1() {
    let input = "input/input.txt";
    let contents = fs::read_to_string(input).unwrap();

    let grid: Vec<Vec<char>> = contents.lines().map(
        |x| x.chars().collect()
    ).collect();

    let mut y: usize = 0;
    let mut trees: usize = 0;

    grid.iter().for_each(|x| {
        //println!("{}", x[y % x.len()]);
        if x[y % x.len()] == '#' {
            trees = trees + 1
        }
        y += 3
    });

    //println!("{:#?}", grid);
    println!("{:#?}", trees);
}

fn main() {
    day_03_1();
    println!("Hello, world!");
}
