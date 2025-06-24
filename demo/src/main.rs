mod helper;
use helper::*;

// Blackjack values. Not ideal, will improve upon later
const CARD_VALS: [&[u8]; 13] = [&[1, 10], &[2], &[3], &[4], &[5], &[6], &[7], &[8], &[9], &[10], &[11], &[12], &[13]];

fn main() {
    println!("Hello, world! This is a test demo for trying out some basic ideas!");

    println!("For example, we can initialize a card!");
    let card1 = Card::new(Face::Jack(), Suit::Diamonds());
    println!("I just initialized the card \x1b[33m{}\x1b[m!", card1.to_string());

    println!("Individual cards are quite a chore to deal with. Let's introduce a Deck!");
    let mut deck: Deck = Deck::new();
    deck.shuffle();
    println!("We initialized a deck with \x1b[33m{}\x1b[m cards!", deck.cards.len());
    println!("Here's a full list of cards:\n\x1b[33m{}\x1b[m", deck.to_short_string("\x1b[m, \x1b[33m"));

    let card2 = Card::new(Face::Ace(), Suit::Clubs());
    // Not a very pretty line. Will make a .to_string(delimiter) for Vec<u8> later
    println!("Say you pulled the card \x1b[33m{}\x1b[m and we're going with Blackjack rules. The value(s) would be \x1b[33m[{}]\x1b[m", card2.clone().to_short_string(), card2.get_values(CARD_VALS).iter().map(|n| n.to_string()).collect::<Vec<String>>().join("\x1b[m, \x1b[33m"));
}