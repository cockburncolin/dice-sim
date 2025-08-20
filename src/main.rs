mod dice;

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

fn process_string(s: &String) -> DieStack {
    let mut ret_stack = DieStack::new();
    let dice_string = s.to_uppercase();
    let str_vec: Vec<&str> = dice_string.split('D').collect();

    let count: u32 = if str_vec[0].is_empty() {
        1
    } else {
        str_vec[0].parse().expect("Count provided wasn't a number")
    };

    let sides: u32 = str_vec[1].parse().expect("Sides provided not a number");
    let name: String = String::from("D") + &sides.to_string();

    for _ in 0..count {
        ret_stack.push(Die::new(&name, &sides));
    }

    return ret_stack;      
}

fn main() {
    let args: Args = Args::parse();
    let mut stack = DieStack::new();

    for s in args.dice_str {
        stack.extend(process_string(&s));
    }

    stack.roll();
    stack.print_results();
}
