use rand::Rng;
use std::vec::Vec;

pub type DieStack = Vec<Die>;

#[derive(Clone, Debug, Default)]
pub struct Die {
    pub name: String,
    pub sides: u32,
    pub result: u32,
}

impl Die {
    pub fn new() -> Die {
        Die {
            name: String::from(""),
            sides: 0,
            result: 0
        }
    }
}

pub trait Outputable {
    fn print_results(&self);
}

impl Outputable for DieStack {
    fn print_results(&self) {
        let mut total: u32 = 0;
        println!("{:<8} {:<8}", "Die", "Result");
        println!("-----------------");

        self.iter().for_each(|d| {
            println!("{:<8} {:<8}", d.name, d.result);
            total += d.result;
        });

        println!("=================");
        println!("{:<8} {:<8}", "Total", total);
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
