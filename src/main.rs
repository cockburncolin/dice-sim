mod dice;

use clap::Parser;
use dice::{DieStack, Outputable, Rollable, Processable};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Strings to operate on.
    /// This should be in the format [optional_qty]d[num_sides]
    #[arg(required = true)]
    dice_str: Vec<String>,
}

fn main() {
    let args: Args = Args::parse();
    let mut stack = DieStack::from_vec(args.dice_str);

    stack.roll();
    stack.print_results();
}
