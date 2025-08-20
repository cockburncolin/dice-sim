# Dice-sim
This is a command line utility to simulate rolling a number of dice.  Made for
the purpose of learning Rust 🦀.

Takes any amount of objects as command line positional arguments for dice that
should be rolled.  The format for the arguments are `[optional qty]d[num
sides]`, eg `dice-sim d6 2d20` to roll one D6 and 2 D20s.  Calculates the totals
for all the dice rolled and displays all the results in a table.
