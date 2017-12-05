use std::str::FromStr;
use std::mem;

mod direction;
use direction::Direction;

mod rooms;
use rooms::Room;

mod cat;
use cat::Cat;

mod state;
use state::State;

mod js;
pub use js::{alloc, free};

fn join_space<S: AsRef<str>, I: Iterator<Item=S>>(words: I) -> String {
    words.fold(String::new(), |acc, word| {
        if acc.is_empty() {
            word.as_ref().into()
        } else {
            format!("{} {}", acc, word.as_ref())
        }
    })
}

enum Action {
    Go(Direction),
    Open(Direction),
    Close(Direction),
    Look,
    Help,
    None,
}

impl FromStr for Action {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<String> = s.split_whitespace()
            .map(|word| word.to_lowercase())
            .filter(|word| match word.as_str() {
                "a" | "an" => false,
                "the" | "my" | "your" => false,
                "to" | "from" => false,
                _ => true,
            })
            .collect();
        match words.get(0) {
            Some(word) => match word.as_str() {
                "go" | "walk" | "climb" | "travel" => Ok(Action::Go(join_space(words.iter().skip(1)).parse()?)),
                "open" => Ok(Action::Open(join_space(words.iter().skip(1).filter(|word| match word.as_str() {
                    "door" => false,
                    _ => true,
                })).parse()?)),
                "close" => Ok(Action::Close(join_space(words.iter().skip(1).filter(|word| match word.as_str() {
                    "door" => false,
                    _ => true,
                })).parse()?)),
                "look" => {
                    Ok(Action::Look)
                }
                "help" => Ok(Action::Help),
                _ => Err(())
            },
            None => Ok(Action::None),
        }
    }
}

#[no_mangle]
pub fn start() {
    State::reset();
    let mut state = State::get();
    let mut cat: Cat = Cat::new();
    cat.set_name(Cat::get_name().into());
    state.add_cat(cat, &rooms::LOUNGE);
    let room = state.room;
    js::display(format!("{}\n", room.describe(&mut *state)));
    (*state).visited();
    js::timeout_set((js::math_random() * 2000.0).floor() as u32);
}

#[no_mangle]
pub fn on_input(input: *mut u8, input_len: u32) {
    let input = {
        let input = unsafe{String::from_raw_parts(input, input_len as usize, input_len as usize)};
        let input2 = input.clone();
        mem::forget(input);
        input2
    };
    let mut state = State::get();
    let room = state.room;
    js::display(format!("{}\n",
        match input.parse::<Action>() {
            Ok(action) => match action {
                Action::Go(direction) => state.go(direction),
                Action::Open(direction) => state.open(direction),
                Action::Close(direction) => state.close(direction),
                Action::Look => room.describe(&mut *state),
                Action::Help => help().to_string(),
                Action::None => "Time passes.".to_string(),
            },
            Err(_) => format!("Sorry, I didn't understand that.\nAsk for help if you need it!"),
        }));
    js::timeout_set((js::math_random() * 2000.0).floor() as u32);
}

fn help() -> &'static str {
    "I understand: \"north\", \"south\", \"west\", \"east\", \"open\", \"close\", \"look\" and \"help\"."
}

#[no_mangle]
pub fn on_timeout() {
    let mut state = State::get();
    let room: &Room = rooms::ROOMS.get((js::math_random() * rooms::ROOMS.len() as f32).floor() as usize).unwrap();
    let here = state.room;
    let have_cats = match state.cats.get(&room) {
        Some(cats) => !cats.is_empty(),
        None => false,
    };
    if have_cats {
        let direction = direction::DIRECTIONS.get((js::math_random() * direction::DIRECTIONS.len() as f32).floor() as usize).unwrap();
        let there = room.get_room(direction);
        match there {
            Some(there) => {
                if state.is_door_open_between(room, there) {
                    let cat = {
                        let cats = state.cats.get_mut(&room).unwrap();
                        let num_cats = cats.len();
                        cats.swap_remove((js::math_random() * num_cats as f32).floor() as usize)
                    };
                    if room == here {
                        js::display(format!("{} leaves through the door to the {}.\n", cat.describe(), direction));
                    } else if there == here {
                        js::display(format!("{} enters through the door to the {}.\n", cat.describe(), direction.opposite()));
                    }
                    state.add_cat(cat, there);
                } else {
                    if room == here {
                        let cats = state.cats.get(&room).unwrap();
                        js::display(format!("{} scratches at the door to the {}.\n", cats.get((js::math_random() * cats.len() as f32).floor() as usize).unwrap().describe(), direction));
                    } else if there == here {
                        js::display(format!("You hear scratching at the door to the {}.\n", direction.opposite()));
                    }
                }
            },
            None => if room == here {
                let cats = state.cats.get(&room).unwrap();
                js::display(format!("{} walks around aimlessly.\n", cats.get((js::math_random() * cats.len() as f32).floor() as usize).unwrap().describe()));
            },
        }
    }
    for room in &[&rooms::PORCH, &rooms::GARDEN] {
        let have_cats = match state.cats.get(room) {
            Some(cats) => !cats.is_empty(),
            _ => false,
        };
        if !have_cats {
            if js::math_random() > 0.95 {
                let cat = Cat::new();
                if room == &here {
                    js::display(format!("{} comes out of the bushes.", cat.describe()));
                }
                state.add_cat(Cat::new(), room);
            }
        }
    }
    js::timeout_set((js::math_random() * 2000.0).floor() as u32);
}

