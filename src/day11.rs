use std::collections::HashMap;

type Pt = (isize, isize);

#[derive(Clone)]
pub struct Floor {
    plan: HashMap<Pt, char>,
    width: isize,
    height: isize
}

impl Floor {
    fn new(input: &str) -> Floor {
        let mut plan: HashMap<Pt, char> = HashMap::new();
        let height = input.lines().count() as isize;
        let width = input.lines().next().unwrap().chars().count() as isize;

        input.lines()
             .enumerate()
             .for_each(|(y, line)|
                line.chars().enumerate().for_each(|(x, c)| {plan.insert((x as isize,y as isize), c); ()})
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

    fn neighbours<'a>(&'a self, (x, y): Pt) -> impl Iterator<Item=Pt> + 'a {
        (-1..=1).flat_map(move |dx|
                    (-1..=1).map(move |dy| (x + dx, y + dy))
               )
               .filter(move |(c1, c2)| {
                    (0..self.width).contains(c1)
                    && (0..self.height).contains(c2)
                    && (*c1, *c2) != (x,y)
               })
               .map(|(c1, c2)| (c1, c2))
               //.map(|c| {println!("{:#?}", c); c})
    }

    fn contains(&self, (x,y): Pt) -> bool {
        (0..self.width as isize).contains(&x)
            && (0..self.height as isize).contains(&y)
    }

    fn add((x,y): Pt, (dx, dy): Pt) -> Pt {
        (x + dx, y + dy)
    }

    /* Could this return an iterator ? (borrows self ?) */
    fn neighours_values<'a>(&'a self, coords: Pt) -> impl Iterator<Item=char> + 'a {
        self.neighbours(coords).map(move |xy| self.plan[&xy])
    }

    fn seats<'a>(&'a self, point: Pt) -> impl Iterator<Item=char> + 'a {
        let patterns = (-1..=1).flat_map(|x| (-1..=1).map(move |y| (x,y)));

        patterns.filter(|pt| *pt != (0, 0))
                .map(move |direction| {
            let mut seen: char = '.';
            let mut cursor = point;
            loop {
                cursor = Floor::add(cursor, direction);
                //println!("{:#?} {:#?}", cursor, direction);
                if !self.contains(cursor) {
                    break
                }
                seen = self.plan[&cursor];
                if seen != '.' {
                    break
                }
            }
            seen
        })
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
                                     .all(|x| x == 'L' || x == '.') {
                (k, '#')
            } else if v == '#' && prev_floor.neighours_values(k)
                                            .filter(|&x| x == '#')
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

#[aoc(day11, part2)]
fn part2(floor: &Floor) -> usize {
    let mut prev_floor = floor.clone();
    let mut count = 1;

    loop {
        let mut new_floor = prev_floor.clone();
        new_floor.plan = prev_floor.plan.iter().map(|(&k,&v)| {
            if v == 'L' && prev_floor.seats(k)
                                     .all(|x| x == 'L' || x == '.') {
                (k, '#')
            } else if v == '#' && prev_floor.seats(k)
                                            .filter(|x| *x == '#')
                                            .count() >= 5 {
                (k, 'L')
            } else {
                (k, v)
            }
        }).collect();

        let next_count = prev_floor.plan.values().filter(|&c| *c == '#').count();
        if next_count == count {
            break
        }
        prev_floor = new_floor;
        count = next_count
    }
    count
}
