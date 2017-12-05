use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub enum Direction {
    North,
    South,
    West,
    East,
    Up,
    Down
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            &Direction::North => Direction::South,
            &Direction::South => Direction::North,
            &Direction::West => Direction::East,
            &Direction::East => Direction::West,
            &Direction::Up => Direction::Down,
            &Direction::Down => Direction::Up,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &Direction::North => "north",
            &Direction::South => "south",
            &Direction::West => "west",
            &Direction::East => "east",
            &Direction::Up => "up",
            &Direction::Down => "down",
        })
    }
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "north" | "n" => Ok(Direction::North),
            "south" | "s" => Ok(Direction::South),
            "west" | "w" => Ok(Direction::West),
            "east" | "e" => Ok(Direction::East),
            "up" | "u" => Ok(Direction::Up),
            "down" | "d" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

pub static DIRECTIONS: &[&'static Direction] = &[
    &Direction::North,
    &Direction::South,
    &Direction::West,
    &Direction::East,
];
