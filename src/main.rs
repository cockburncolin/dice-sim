#[allow(dead_code)]
mod dice;

use clap::Parser;
use dice::{Die, DieStack, Rollable};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {

}

fn main() {
    let _args = Args::parse();
    let mut stack = DieStack::new();

    // "library" of dice
    let dice = [
        Die::new(String::from("D4"),   4),
        Die::new(String::from("D6"),   6),
        Die::new(String::from("D20"),  20),
        Die::new(String::from("D100"), 100),
    ];

    stack.push(dice[1].clone());
    stack.push(dice[3].clone());
    stack.roll();

    println!("{:?}", stack);
}
