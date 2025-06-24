mod helper;
use helper::*;

fn main() {
    println!("Hello, world! This is a test demo for trying out some basic ideas!");

    println!("For example, we can initialize a card!");
    let card = Card::new(Face::Jack(), Suit::Diamonds());
    println!("I just initialized the card \x1b[33m{}\x1b[m!", card.to_string());

    println!("Individual cards are quite a chore to deal with. Let's introduce a Deck!");
    let mut deck: Deck = Deck::new();
    deck.shuffle();
    println!("We initialized a deck with \x1b[33m{}\x1b[m cards!", deck.cards.len());
    println!("Here's a full list of cards:\n\x1b[33m{}\x1b[m", deck.to_short_string("\x1b[m, \x1b[33m"));
}