use rand::Rng;
use std::iter::repeat;
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
            result: 0,
        }
    }
}

pub trait Processable {
    fn from_vec(input: Vec<String>) -> Self;
}

impl Processable for DieStack {
    /// Process the strings provided by the user
    fn from_vec(input: Vec<String>) -> Self {
        let mut ret_stack = DieStack::new();

        for str in input {
            let mut die = Die::new();
            let upper_string = str.to_uppercase();
            let dice_string: Vec<&str> = upper_string.split("D").collect();

            let count: usize = if dice_string[0].is_empty() {
                1
            } else {
                dice_string[0]
                    .parse()
                    .expect("Count provided wasn't a number")
            };

            let sides: u32 = dice_string[1].parse().expect("Sides provided not a number");
            let name = format!("D{sides}");

            die.sides = sides;
            die.name = name;

            ret_stack.extend(repeat(die).take(count)); // Push die onto stack count times
        }
        return ret_stack;
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
