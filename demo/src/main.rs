mod helper;
use helper::*;

fn main() {
    println!("Hello, world! This is a test demo for trying out some basic ideas!");
    println!("For example, we can initialize a card!");
    let card = Card::new("Jack", "Diamonds");
    println!("I just initialized the card \x1b[33m{card}\x1b[m!")
}