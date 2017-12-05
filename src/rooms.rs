use super::{Direction, State};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Room {
    pub name: &'static str,
    pub long_desc: &'static str,
    pub short_desc: &'static str,
    pub north: Option<&'static Room>,
    pub south: Option<&'static Room>,
    pub west: Option<&'static Room>,
    pub east: Option<&'static Room>,
    pub up: Option<&'static Room>,
    pub down: Option<&'static Room>,
    private_marker: (),
}

impl Room {
    pub fn get_room(&self, direction: &Direction) -> Option<&'static Room> {
        match direction {
            &Direction::North => self.north.clone(),
            &Direction::South => self.south.clone(),
            &Direction::West => self.west.clone(),
            &Direction::East => self.east.clone(),
            &Direction::Up => self.up.clone(),
            &Direction::Down => self.down.clone(),
        }
    }

    pub fn describe(&'static self, state: &State) -> String {
        let mut desc = if state.has_visited(self) {
            self.short_desc
        } else {
            self.long_desc
        }.to_string();
        for direction in &[Direction::North, Direction::South, Direction::West, Direction::East, Direction::Up, Direction::Down] {
            match self.get_room(direction) {
                Some(there) => desc = format!("{}\nThe door to {} is {}.",
                    desc,
                    if state.has_visited(&there) {
                        there.name.to_string()
                    } else {
                        format!("the {}", direction)
                    },
                    if state.is_door_open(there) { "open" } else { "closed" }),
                None => {},
            };
        }
        match state.cats.get(&self) {
            Some(ref cats) => {
                for cat in cats.iter() {
                    desc = format!("{}\n{} is here.", desc, cat.describe());
                }
            },
            None => {},
        }
        desc
    }
}

impl PartialEq for &'static Room {
    fn eq(&self, other: &&'static Room) -> bool {
        (*self as *const Room) == (*other as *const Room)
    }
}

impl Eq for &'static Room {}

impl Hash for &'static Room {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        (*self as *const Room).hash(state)
    }
}

pub static ROOMS: &[&'static Room] = &[
    &KITCHEN,
    &HALL,
    &LOUNGE,
    &GARDEN,
    &PORCH,
];

pub static KITCHEN: Room = Room{
    name: "the kitchen",
    long_desc: "You enter a kitchen.\nThere's a large table in the center of the room, a sink and hob against the easst wall and a large fridge to the north.",
    short_desc: "You're in the kitchen.",
    north: None,
    south: Some(&HALL),
    west: Some(&GARDEN),
    east: None,
    up: None,
    down: None,
    private_marker: (),
};

pub static HALL: Room = Room{
    name: "the hall",
    long_desc: "You're standing in the hallway of a small house.\nIt is dominated by a large mirror on the east wall.\nThere's a doormat in front of the door to the south.",
    short_desc: "You're in the hall.",
    north: Some(&KITCHEN),
    south: Some(&PORCH),
    west: Some(&LOUNGE),
    east: None,
    up: None,
    down: None,
    private_marker: (),
};

pub static LOUNGE: Room = Room{
    name: "the lounge",
    long_desc: "You're in a lounge.\nTwo slightly delapidated sofas run along the south and west walls, facing a TV that sites in the north-east corner.",
    short_desc: "You're in the lounge.",
    north: Some(&GARDEN),
    south: None,
    west: None,
    east: Some(&HALL),
    up: None,
    down: None,
    private_marker: (),
};

pub static GARDEN: Room = Room{
    name: "the garden",
    long_desc: "You step out into a garden.\nA wooden table and bench sit on a stone patio, and a lawn is bordered by well-maintained flower beds.",
    short_desc: "You're in the garden.",
    north: None,
    south: Some(&LOUNGE),
    west: Some(&KITCHEN),
    east: None,
    up: None,
    down: None,
    private_marker: (),
};

pub static PORCH: Room = Room{
    name: "the porch",
    long_desc: "Stepping through the front door, you find yourself on the porch of the house.\nA recently-mown lawn runs to a white picket fence.",
    short_desc: "You're on the porch.",
    north: Some(&HALL),
    south: None,
    west: None,
    east: None,
    up: None,
    down: None,
    private_marker: (),
};

