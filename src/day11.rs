use std::collections::HashMap;

#[derive(Clone)]
pub struct Floor {
    plan: HashMap<(usize, usize), char>,
    width: usize,
    height: usize
}

impl Floor {
    fn new(input: &str) -> Floor {
        let mut plan: HashMap<(usize, usize), char> = HashMap::new();
        let height = input.lines().count();
        let width = input.lines().next().unwrap().chars().count();

        input.lines()
             .enumerate()
             .for_each(|(y, line)|
                line.chars().enumerate().for_each(|(x, c)| {plan.insert((x,y), c); ()})
             );
        Floor {
            plan,
            width,
            height
        }
    }

    fn debug(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.plan[&(x,y)]);
            }
            print!("\n");
        }
    }

    fn neighbours(&self, (x, y): (usize, usize)) -> impl Iterator<Item=(usize, usize)> {
        let width = self.width;
        let height = self.height;
        (-1..=1).flat_map(move |dx|
                    (-1..=1).map(move |dy| (x as isize + dx, y as isize + dy))
               )
               .filter(move |(c1, c2)| {
                    (0..width as isize).contains(c1)
                    && (0..height as isize).contains(c2)
                    && (*c1, *c2) != (x as isize,y as isize)
               })
               .map(|(c1, c2)| (c1 as usize, c2 as usize))
               //.map(|c| {println!("{:#?}", c); c})
    }

    /* Could this return an iterator ? (borrows self ?) */
    fn neighours_values(&self, coords: (usize, usize)) -> Vec<char> {
        self.neighbours(coords).map(|xy| self.plan[&xy]).collect()
    }
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Floor {
    Floor::new(input)
}

#[aoc(day11, part1)]
fn part1(floor: &Floor) -> usize {
    let mut prev_floor = floor.clone();
    let mut count = 1;

    //prev_floor.debug();
    loop {
        // println!();
        let mut new_floor = prev_floor.clone();
        new_floor.plan = prev_floor.plan.iter().map(|(&k,&v)| {
            if v == 'L' && prev_floor.neighours_values(k)
                                     .iter()
                                     .all(|&x| x == 'L' || x == '.') {
                (k, '#')
            } else if v == '#' && prev_floor.neighours_values(k)
                                            .iter()
                                            .filter(|&x| *x == '#')
                                            .count() >= 4 {
                (k, 'L')
            } else {
                (k, v)
            }
        }).collect();

        //new_floor.debug();

        let next_count = prev_floor.plan.values().filter(|&c| *c == '#').count();
        if next_count == count {
            break
        }
        prev_floor = new_floor;
        count = next_count
    }
    count
}
