use itertools::sorted;

#[aoc_generator(day13)]
pub fn generator(input: &str) -> (i32, Vec<i32>) {
    let mut iter = input.lines();
    let departure = iter.next().unwrap().parse::<u32>().unwrap() as i32;
    let buses = iter.next().unwrap()
                    .split(",")
                    .filter(|&x| x != "x")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();
    (departure, buses)
}

#[aoc(day13, part1)]
pub fn part1(input: &(i32, Vec<i32>)) -> i32 {
    println!("{:#?}", input);
    let (departure, buses) = input;
    let (delta, bus) = sorted(buses.iter().map(|x| {
        let loop_count = (departure / x) + 1;
        let timedelta = (loop_count * x) - departure;
        (timedelta, x)
    })).next().unwrap();
    delta * bus
}
