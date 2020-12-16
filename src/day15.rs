use std::collections::HashMap;

#[aoc(day15, part1)]
fn part1(raw: &str) -> isize {
    let input = vec![0,1,5,10,3,12];
    //let input = vec![0,3];
    let mut seen: HashMap<isize, isize> = HashMap::new();

    input.iter()
         .enumerate()
         .for_each(|(i, x)| {seen.insert(*x, (i + 1) as isize); ()});

    let mut i = input.iter().count() as isize + 1;
    //let mut prev = 6;
    let mut prev = 19;

    loop {
        let say = if ! seen.contains_key(&prev) {
            0
        } else {
            i - seen.get(&prev).unwrap()
        };
        //println!("{} {}", i, say);
        seen.insert(prev, i);
        prev = say;
        i = i + 1;

        if i == 2020 {
            break;
        }
    }
    prev
}

#[aoc(day15, part2)]
fn part2(raw: &str) -> isize {
    let input = vec![0,1,5,10,3,12];
    //let input = vec![0,3];
    let mut seen: HashMap<isize, isize> = HashMap::new();

    input.iter()
         .enumerate()
         .for_each(|(i, x)| {seen.insert(*x, (i + 1) as isize); ()});

    let mut i = input.iter().count() as isize + 1;
    //let mut prev = 6;
    let mut prev = 19;

    loop {
        let say = if ! seen.contains_key(&prev) {
            0
        } else {
            i - seen.get(&prev).unwrap()
        };
        //println!("{} {}", i, say);
        seen.insert(prev, i);
        prev = say;
        i = i + 1;

        if i == 30000000 {
            break;
        }
    }
    prev
}
