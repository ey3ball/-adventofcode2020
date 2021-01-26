use std::ops::{Add, Mul, Sub};

type Move = (char, i64);

struct Ship {
    position: Point,
    orientation: usize,
    waypoint: Point,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}
impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
    fn rotate(&self, angle: i64) -> Point {
        let by_quadrant = ((angle / 90) % 4 + 4) % 4;
        if by_quadrant == 0 {
            *self
        } else if by_quadrant == 1 {
            Point {
                x: self.y,
                y: -self.x,
            }
        } else if by_quadrant == 2 {
            Point {
                x: -self.x,
                y: -self.y,
            }
        } else {
            Point {
                x: -self.y,
                y: self.x,
            }
        }
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Mul<i64> for Point {
    type Output = Self;

    fn mul(self, other: i64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

const NORTH: Point = Point { x: 0, y: 1 };
const SOUTH: Point = Point { x: 0, y: -1 };
const EAST: Point = Point { x: 1, y: 0 };
const WEST: Point = Point { x: -1, y: 0 };
const STANDBY: Point = Point { x: 0, y: 0 };
const DIRECTIONS: [Point; 4] = [NORTH, EAST, SOUTH, WEST];

impl Ship {
    fn new() -> Ship {
        Ship {
            position: Point::new(0, 0),
            orientation: 1,
            waypoint: Point::new(10, 1),
        }
    }

    fn act(&mut self, (action, value): Move) {
        let displacement = match action {
            'N' => NORTH * value,
            'W' => WEST * value,
            'E' => EAST * value,
            'S' => SOUTH * value,
            'F' => DIRECTIONS[self.orientation] * value,
            'L' => STANDBY,
            'R' => STANDBY,
            _ => panic!("Unknown direction requested"),
        };
        let orientation = match action {
            'L' => ((((self.orientation + 4) as i64) - value / 90) % 4) as usize,
            'R' => ((self.orientation as i64 + value / 90) % 4) as usize,
            _ => self.orientation as usize,
        };
        let position = self.position + displacement;
        self.position = position;
        self.orientation = orientation;
    }

    fn act2(&mut self, (action, value): Move) {
        self.waypoint = match action {
            'N' => self.waypoint + (NORTH * value),
            'W' => self.waypoint + (WEST * value),
            'E' => self.waypoint + (EAST * value),
            'S' => self.waypoint + (SOUTH * value),
            'F' => self.waypoint,
            'L' => self.waypoint.rotate(-value),
            'R' => self.waypoint.rotate(value),
            _ => panic!("Unknown direction requested"),
        };
        if action == 'F' {
            self.position = self.position + (self.waypoint * value)
        }
    }
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|l| {
            let direction = l.chars().next().unwrap();
            let value = l[1..].parse::<i64>().unwrap();
            (direction, value)
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn part1(input: &Vec<Move>) -> i64 {
    let mut ship = Ship::new();
    input.iter().for_each(|&action| ship.act(action));
    ship.position.x.abs() + ship.position.y.abs()
}

#[aoc(day12, part2)]
pub fn part2(input: &Vec<Move>) -> i64 {
    let mut ship = Ship::new();
    input.iter().for_each(|&action| ship.act2(action));
    ship.position.x.abs() + ship.position.y.abs()
}
