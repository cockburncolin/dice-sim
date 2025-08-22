mod dice;

use std::iter::repeat;

use clap::Parser;
use dice::{Die, DieStack, Outputable, Rollable};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Strings to operate on.
    /// This should be in the format [optional_qty]d[num_sides]
    #[arg(required = true)]
    dice_str: Vec<String>,
}

/// Process the strings provided by the user
fn process_string(input: Vec<String>) -> DieStack {
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

fn main() {
    let args: Args = Args::parse();
    let mut stack = DieStack::new();

    stack.extend(process_string(args.dice_str));

    stack.roll();
    stack.print_results();
}
