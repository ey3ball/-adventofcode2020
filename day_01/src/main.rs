use std::fs;

fn main() {
    let input = "input/day01.txt";
    let contents = fs::read_to_string(input).unwrap();

    let expenses: Vec<u32> = contents.lines().map(
        |x| x.parse().unwrap()).collect();

    for n1 in &expenses {
        for n2 in &expenses {
            if n1 + n2 == 2020 {
                println!("Part1: {}", n1 * n2)
            }
            for n3 in &expenses {
                if n1 + n2 + n3 == 2020 {
                    println!("Part2: {}", n1 * n2 * n3)
                }
            }
        }
    }
}
