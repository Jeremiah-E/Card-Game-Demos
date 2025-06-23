mod helper;
use helper::*;

fn main() {
    println!("Hello, world! This is a test demo for trying out some basic ideas!");

    println!("For example, we can initialize a card!");
    let card = Card::new(Face::Jack(), Suit::Diamonds());
    println!("I just initialized the card \x1b[33m{card}\x1b[m!");

    println!("Individual cards are quite a chore to deal with. Let's introduce a Deck!");
    let deck = Deck::new();
    println!("We initialized a deck with \x1b[33m{}\x1b[m cards!", deck.cards.len());
    let mut deck_str = String::new();
    for card in deck.cards {
        deck_str.push_str(card.to_string().as_str());
        deck_str.push_str("\x1b[m, \x1b[33m");
    }
    deck_str = deck_str.strip_suffix("\x1b[m, \x1b[33m").unwrap().to_string();
    println!("Here's a full list of cards:\n\x1b[33m{}\x1b[m", deck_str)
}