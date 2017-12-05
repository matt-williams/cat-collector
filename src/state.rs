use std::sync::{Mutex, MutexGuard};
use std::collections::{HashSet, HashMap};
use super::{rooms, Room, Direction, Cat};

#[derive(Debug)]
pub struct State {
    pub room: &'static Room,
    pub visited_rooms: HashSet<&'static Room>,
    pub open_doors: HashSet<(&'static Room, &'static Room)>,
    pub cats: HashMap<&'static Room, Vec<Cat>>,
}

static mut STATE: Option<Mutex<State>> = None;

impl State {
    fn new() -> Self {
        Self{
            room: &rooms::HALL,
            visited_rooms: HashSet::new(),
            open_doors: HashSet::new(),
            cats: HashMap::new(),
        }
    }

    pub fn get() -> MutexGuard<'static, State> {
        unsafe {
            if STATE.is_none() {
                STATE = Some(Mutex::new(State::new()));
            }
            match STATE {
                Some(ref state) => state.lock().unwrap(),
                None => unreachable!(),
            }
        }
    }

    pub fn reset() {
        *Self::get() = Self::new();
    }

    pub fn visited(&mut self) {
        let here = self.room;
        self.visited_rooms.insert(here as &'static Room);
    }

    pub fn has_visited(&self, room: &'static Room) -> bool {
        self.visited_rooms.contains(&(room as &'static Room))
    }

    pub fn open_door(&mut self, there: &'static Room) -> String {
        let here = self.room;
        self.open_doors.insert((here, there));
        self.open_doors.insert((there, here));
        "The door is now open.".to_string()
    }

    pub fn close_door(&mut self, there: &'static Room) -> String {
        let here = self.room;
        self.open_doors.remove(&(here, there));
        self.open_doors.remove(&(there, here));
        "The door is now closed.".to_string()
    }

    pub fn is_door_open(&self, there: &'static Room) -> bool {
        self.is_door_open_between(self.room, there)
    }

    pub fn is_door_open_between(&self, here: &'static Room, there: &'static Room) -> bool {
        self.open_doors.contains(&(here, there))
    }

    pub fn go(&mut self, direction: Direction) -> String {
        match self.room.get_room(&direction) {
            Some(there) => {
                if self.is_door_open(there) {
                    self.room = there;
                    let description = self.room.describe(self);
                    self.visited();
                    description
                } else {
                    "The door is closed.".to_string()
                }
            },
            None => "You can't go that way.".to_string(),
        }
    }

    pub fn open(&mut self, direction: Direction) -> String {
        match self.room.get_room(&direction) {
            Some(there) => {
                if self.is_door_open(there) {
                    "The door is already open".to_string()
                } else {
                    self.open_door(there)
                }
            },
            None => "There is no door in that direction.".to_string(),
        }
    }

    pub fn close(&mut self, direction: Direction) -> String {
        match self.room.get_room(&direction) {
            Some(there) => {
                if !self.is_door_open(there) {
                    "The door is already closed".to_string()
                } else {
                    self.close_door(there)
                }
            },
            None => "There is no door in that direction.".to_string(),
        }
    }

    pub fn add_cat(&mut self, cat: Cat, room: &'static Room) {
        self.cats.entry(room).or_insert(Vec::new()).push(cat);
    }
}

