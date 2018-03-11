use std::io;
use std::cmp::Ordering;

// TODO:
// need to abstract the looping mechanism into a function which gets called in either arm of the if/else.

fn main() {
    println!("Print a table of n!");
    println!("Enter the (numeric) value of n:");

    let mut choice = String::new();

    io::stdin().read_line(&mut choice)
        .expect("Failed to read line.");

    let choice: u32 = choice.trim().parse()
        .expect("Failed to read line.");

    println!("Enter upper bound (not to be exceeded):");

    let mut boundary = String::new();

    io::stdin().read_line(&mut boundary)
        .expect("Failed to read line.");

    let boundary: u32 = boundary.trim().parse()
        .expect("Failed to read line.");

    println!("Whole numbers between between 0 and {}, incremented by {}:", boundary, choice);

    let mut increment = choice;

    let remainder = boundary % choice;

    println!("{}", choice);

    if remainder > 0 {
        let boundary: u32 = boundary - remainder;
        loop {
            increment += choice;
            println!("{}", increment);

            match increment.cmp(&boundary) {
                Ordering::Less    => {
                    continue;
                }
                Ordering::Greater => {
                    break;
                }
                Ordering::Equal   => {
                    break;
                }
            }
        }
    } else {
        loop {
            increment += choice;
            println!("{}", increment);

            match increment.cmp(&boundary) {
                Ordering::Less    => {
                    continue;
                }
                Ordering::Greater => {
                    break;
                }
                Ordering::Equal   => {
                    break;
                }
            }
        }
    }
}
