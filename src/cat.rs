use super::js;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Cat {
    pub name: Option<Cow<'static, str>>,
    pub breed: &'static str,
}

impl Cat {
    pub fn new() -> Self {
        Self{
            name: None,
            breed: Cat::get_breed(),
        }
    }
    
    pub fn describe(&self) -> String {
        self.name.clone().map(|s| s.into_owned()).unwrap_or_else(|| format!("A {} cat", self.breed))
    }

    pub fn set_name(&mut self, name: Cow<'static, str>) {
        self.name = Some(name);
    }

    pub fn get_name() -> &'static str {
        NAMES.get((js::math_random() * NAMES.len() as f32).floor() as usize).unwrap()
    }

    pub fn get_breed() -> &'static str {
        BREEDS.get((js::math_random() * BREEDS.len() as f32).floor() as usize).unwrap()
    }
}

// Adapted from https://github.com/sindresorhus/cat-names
static NAMES: &[&'static str] = &[
    "Abby",
    "Angel",
    "Annie",
    "Baby",
    "Bailey",
    "Bandit",
    "Bear",
    "Bella",
    "Bob",
    "Boo",
    "Boots",
    "Bubba",
    "Buddy",
    "Buster",
    "Cali",
    "Callie",
    "Casper",
    "Charlie",
    "Chester",
    "Chloe",
    "Cleo",
    "Coco",
    "Cookie",
    "Cuddles",
    "Daisy",
    "Dusty",
    "Felix",
    "Fluffy",
    "Garfield",
    "George",
    "Ginger",
    "Gizmo",
    "Gracie",
    "Harley",
    "Jack",
    "Jasmine",
    "Jasper",
    "Kiki",
    "Kitty",
    "Leo",
    "Lilly",
    "Lily",
    "Loki",
    "Lola",
    "Lucky",
    "Lucy",
    "Luna",
    "Maggie",
    "Max",
    "Mia",
    "Midnight",
    "Milo",
    "Mimi",
    "Miss kitty",
    "Missy",
    "Misty",
    "Mittens",
    "Molly",
    "Muffin",
    "Nala",
    "Oliver",
    "Oreo",
    "Oscar",
    "Patches",
    "Peanut",
    "Pepper",
    "Precious",
    "Princess",
    "Pumpkin",
    "Rascal",
    "Rocky",
    "Sadie",
    "Salem",
    "Sam",
    "Samantha",
    "Sammy",
    "Sasha",
    "Sassy",
    "Scooter",
    "Shadow",
    "Sheba",
    "Simba",
    "Simon",
    "Smokey",
    "Snickers",
    "Snowball",
    "Snuggles",
    "Socks",
    "Sophie",
    "Spooky",
    "Sugar",
    "Tiger",
    "Tigger",
    "Tinkerbell",
    "Toby",
    "Trouble",
    "Whiskers",
    "Willow",
    "Zoe",
    "Zoey",
];

static BREEDS: &[&'static str] = &[
    "Bengal",
    "Burmese",
    "Javanese",
    "Siamese",
];
