use std::fs;
use std::collections::HashSet;

fn parse_input() -> Vec<usize> {
    let input = "input/input.txt";
    let contents = fs::read_to_string(input).unwrap();

    contents.lines()
            .map(
        |x| x.replace("R", "1").replace("L", "0").replace("F", "0").replace("B", "1"))
            .map(|x| usize::from_str_radix(&x, 2).unwrap())
            .collect()
}

fn day_05_1() {
    let data = parse_input();
    println!("{:#?}", data.iter().max());
}

fn day_05_2() {
    let data = parse_input();
    // 864 = part1 answer
    let mut seats = (0..864).collect::<HashSet<usize>>();

    data.iter().for_each(|x| {seats.remove(x); ()});
    let result = seats.iter().filter(
        |x| !seats.contains(&(**x + 1)) && !seats.contains(&(**x - 1))
    ).next();

    println!("{:#?}", result);
}



fn main() {
    day_05_1();
    day_05_2();
    println!("Hello, world!");
}
