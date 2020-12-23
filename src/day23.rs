#[aoc_generator(day23)]
pub fn generator(input: &str) -> Vec<i64> {
    input.chars()
        .map(|digit| {
            digit.to_digit(10).unwrap() as i64
        })
        .collect()
}

pub fn crabby_move(moves: u64, input: &Vec<i64>) -> Vec<i64> {
    let n: i64 = input.len() as i64;
    let mut cups = input.clone();
    let mut cur_idx = 0;

    let mut mv = 1;
    while mv <= moves {
        println!("\nmove: {}", mv);
        println!("cups: {}", format!("{:#?}", cups).replace("\n", ""));
        let cur_label = cups[cur_idx];

        // Immediately lookup what the next label will be, this lets us easily
        // center the solution back on the expected next label once we're done
        let next_idx = (cur_idx + 4) % input.len();
        let next_label = cups[next_idx];

        let picked_up: Vec<i64> = cups.iter()
            .cycle()
            .skip(cur_idx + 1)
            .take(3)
            .copied().collect();
        let mut next: Vec<i64> = cups.iter()
            .filter(|x| !picked_up.contains(x))
            .copied().collect();

        println!("picked up: {}", format!("{:#?}", picked_up).replace("\n", ""));

        let mut dest = cur_label + n;
        dest = loop {
            dest = dest - 1;
            let value = if dest == n { dest } else { dest % n };
            if !picked_up.contains(&value) {
                break value
            }
        };
        println!("destination: {}", dest);
        /* Find out where to insert picked up digits and insert them back */
        let dest_idx = next.iter()
            .enumerate()
            .find(|(_i,v)| **v == dest)
            .unwrap().0;
        picked_up.iter()
            .rev()
            .for_each(|v| next.insert(dest_idx + 1, *v));

        /* Find out where next label is and reorder sequence accordingly */
        let new_idx = next.iter()
            .enumerate()
            .find(|(_i, v)| **v == next_label)
            .unwrap().0;

        cups = next.iter()
                .cycle()
                .skip(new_idx - cur_idx - 1 + input.len())
                .take(input.len())
                .copied().collect();
        mv += 1;
        cur_idx = (cur_idx + 1) % input.len();
    };

    cups
}

#[aoc(day23, part1)]
pub fn part1(input: &Vec<i64>) -> i64 {
    let cups = crabby_move(100, input);

    let final_idx = cups.iter()
        .enumerate()
        .find(|(_i, v)| **v == 1)
        .unwrap().0;

    cups.iter()
        .cycle()
        .skip(final_idx + 1)
        .take(input.len() - 1)
        .map(|&d| std::char::from_digit(d as u32, 10).unwrap())
        .collect::<String>()
        .parse().unwrap()
}

#[aoc(day23, part2)]
pub fn part2(input: &Vec<i64>) -> i64 {
    let mut cups = [
        input.clone(),
        (input.len()+1..1000000).map(|x| x as i64).collect()
    ].concat();

    cups = crabby_move(10000000, input);

    let final_idx = cups.iter()
        .enumerate()
        .find(|(_i, v)| **v == 1)
        .unwrap().0;

    let clockwise: Vec<i64> = cups.iter()
        .cycle()
        .skip(final_idx + 1)
        .take(2)
        .copied()
        .collect();

    println!("{} {}", clockwise[0], clockwise[1]);
    0
}
