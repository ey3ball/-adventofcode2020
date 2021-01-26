use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Active,
    Inactive,
}
type Coords = (i64, i64, i64, i64);
type PocketSpace = HashMap<Coords, State>;

#[derive(Debug, Clone)]
pub struct Pocket {
    space: PocketSpace,
    xs: (i64, i64),
    ys: (i64, i64),
    zs: (i64, i64),
    ws: (i64, i64),
}

impl Pocket {
    fn get(&self, coords: &Coords) -> char {
        match self.space.get(coords) {
            Some(State::Active) => '#',
            Some(State::Inactive) => '.',
            None => '.',
        }
    }

    fn show(&self, z: i64) {
        println!("z={}", z);
        for y in self.ys.0..=self.ys.1 {
            for x in self.xs.0..=self.xs.1 {
                print!("{}", self.get(&(x, y, z, 0)))
            }
            print!("\n");
        }
    }

    fn all_cubes(&self) -> Vec<Coords> {
        (self.xs.0 - 1..=self.xs.1 + 1)
            .flat_map(move |x| {
                (self.ys.0 - 1..=self.ys.1 + 1).flat_map(move |y| {
                    (self.zs.0 - 1..=self.zs.1 + 1).flat_map(move |z| {
                        (self.ws.0 - 1..=self.ws.1 + 1).map(move |q| (x, y, z, q))
                    })
                })
            })
            .collect()
    }

    fn cycle(&mut self) {
        let state = self.space.clone();

        for c in self.all_cubes().iter() {
            match state.get(c) {
                Some(State::Active) => {
                    let act_count = active(c, &state);
                    if act_count != 2 && act_count != 3 {
                        self.space.insert(*c, State::Inactive);
                    }
                }
                Some(State::Inactive) | None => {
                    let act_count = active(c, &state);
                    if act_count == 3 {
                        self.space.insert(*c, State::Active);
                    }
                }
            }
        }
        self.xs.0 -= 1;
        self.xs.1 += 1;
        self.ys.0 -= 1;
        self.ys.1 += 1;
        self.zs.0 -= 1;
        self.zs.1 += 1;
        self.ws.0 -= 1;
        self.ws.1 += 1;
    }
}

pub fn active(c: &Coords, space: &PocketSpace) -> usize {
    neigh(c)
        .iter()
        .map(|c| space.get(c).or(Some(&State::Inactive)).unwrap())
        .filter(|s| **s == State::Active)
        .count()
}

pub fn neigh(coords: &Coords) -> Vec<Coords> {
    let deltas = (-1..=1)
        .flat_map(|dx| {
            (-1..=1).flat_map(move |dy| {
                (-1..=1).flat_map(move |dz| (-1..=1).map(move |dw| (dx, dy, dz, dw)))
            })
        })
        .filter(|(dx, dy, dz, dw)| !(*dx == 0 && *dy == 0 && *dz == 0 && *dw == 0));
    deltas
        .map(|d| {
            (
                coords.0 + d.0,
                coords.1 + d.1,
                coords.2 + d.2,
                coords.3 + d.3,
            )
        })
        .collect()
}

#[aoc_generator(day17)]
pub fn generator(input: &str) -> Pocket {
    let mut space: PocketSpace = HashMap::new();
    input
        .lines()
        .enumerate()
        .map(|(y, cs)| (y as i64, cs))
        .for_each(|(y, cubes)| {
            cubes
                .chars()
                .enumerate()
                .map(|(x, c)| (x as i64, c))
                .for_each(|(x, c)| {
                    if c == '.' {
                        space.insert((x, y, 0, 0), State::Inactive)
                    } else {
                        space.insert((x, y, 0, 0), State::Active)
                    };
                });
        });
    Pocket {
        space,
        xs: (
            0,
            (input.lines().next().unwrap().chars().count() - 1) as i64,
        ),
        ys: (0, (input.lines().count() - 1) as i64),
        zs: (0, 0),
        ws: (0, 0),
    }
}

#[aoc(day17, part2)]
fn part2(input: &Pocket) -> usize {
    let mut pocket = input.clone();

    pocket.cycle();
    pocket.cycle();
    pocket.cycle();
    pocket.cycle();
    pocket.cycle();
    pocket.cycle();
    pocket
        .space
        .values()
        .filter(|s| **s == State::Active)
        .count()
}
