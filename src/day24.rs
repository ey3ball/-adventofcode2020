use std::collections::HashMap;

use regex::Regex;

type Coords = (i64, i64);
// true == white, false == black
type Map = HashMap<Coords, bool>;

#[derive(Clone, Copy)]
pub enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE
}

lazy_static! {
    static ref DIRECTIONS: Regex = Regex::new("(e|se|sw|w|nw|ne)").unwrap();
}

pub fn parse(input: &str) -> Vec<Vec<Dir>> {
    input.lines()
        .map(|l| {
            DIRECTIONS.captures_iter(l)
                .map(|c| {
                    match c.get(0).unwrap().as_str() {
                        "e" => Dir::E,
                        "se" => Dir::SE,
                        "sw" => Dir::SW,
                        "w" => Dir::W,
                        "nw" => Dir::NW,
                        "ne" => Dir::NE,
                        _ => panic!()
                    }
                })
                .collect::<Vec<Dir>>()
        })
        .collect()
}

fn flip(coords: Coords, tiles: &mut Map) {
    match tiles.get(&coords).cloned() {
        Some(b) => tiles.insert(coords, !b),
        None => tiles.insert(coords, false)
    };
    println!("Flipped {:#?} to {}", coords, tiles.get(&coords).unwrap());
}

fn where_to(dir: Dir) -> Coords {
    match dir {
        Dir::E => (2, 0),
        Dir::W => (-2, 0),
        Dir::SE => (1, -1),
        Dir::SW => (-1, -1),
        Dir::NE => (1, 1),
        Dir::NW => (-1, 1),
    }
}

#[aoc(day24, part1)]
fn part1(input: &str) -> usize {
    let mut tiles: HashMap<(i64, i64), bool> = HashMap::new();
    let paths = parse(input);

    paths.iter()
        .for_each(|p| {
            let identified = p.iter().fold((0,0), |acc, d| {
                let delta = where_to(*d);
                let new_pos = (acc.0 + delta.0, acc.1 + delta.1);
                new_pos
            });
            flip(identified, &mut tiles);
        });
    println!("{:#?}", tiles);
    tiles.values().filter(|x| **x == false).count()
}
