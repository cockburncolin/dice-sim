use rand::Rng;
use std::vec::Vec;

pub type DieStack = Vec<Die>;

#[derive(Clone, Debug)]
pub struct Die {
    pub name: String,
    pub sides: u8,
    pub result: u8
}

impl Die {
    pub fn new(name: String, sides: u8) -> Self {
        Self {
            name,
            sides,
            result: 0,
        }
    }
}

pub trait Rollable {
    fn roll(&mut self);
}

impl Rollable for Die {
    fn roll(&mut self) {
        let mut rng = rand::rng();
        let max_size = self.sides + 1;
        self.result = rng.random_range(1..max_size);
    }
}

impl Rollable for DieStack {
    fn roll(&mut self) {
        self.iter_mut().for_each(|d| d.roll());
    }
}

