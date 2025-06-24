mod helper;

use std::cmp::Ordering;
use helper::*;

const CARD_VALS: [&[u8]; 13] = [ &[13], &[1], &[2], &[3], &[4], &[5], &[6], &[7], &[8], &[9], &[10], &[11], &[12] ];

const PLAYER: u8 = 1;
const OPPONENT: u8 = 2;
const NONE: u8 = 3;


fn main() {
    // Initalize everything
    let mut deck = Deck::new();
    deck.shuffle();
    let mut player;
    let mut opponent;
    { // Reduce namespace pollution
        let len = deck.cards.len() as u16 / 2;
        player = deck.draw(len);
        opponent = deck.draw(len);
        println!("Gave both players \x1b[33m{len}\x1b[m cards.");
    }
    
    let mut winner = NONE;
    while winner == NONE {
        let mut player_heap = player.draw(1);
        let mut opponent_heap = opponent.draw(1);

        let pl_val = player_heap.cards[0].get_values(CARD_VALS)[0];
        let op_val = opponent_heap.cards[0].get_values(CARD_VALS)[0];

        match pl_val.cmp(&op_val) {
            // pl < op. Pl wins
            Ordering::Less => {
                println!("Your card: {}. Their card: {}", player_heap.cards[0].to_short_string(), opponent_heap.cards[0].to_short_string());
                println!("You win!");
                player.extend(player_heap);
                player.extend(opponent_heap);
                println!("New deck sizes: You: {}, Them: {}", player.cards.len(), opponent.cards.len());
            },
            // pl > op. Op wins
            Ordering::Greater => {
                println!("Your card: {}. Their card: {}", player_heap.cards[0].to_short_string(), opponent_heap.cards[0].to_short_string());
                println!("Opponent wins!");
                opponent.extend(player_heap);
                opponent.extend(opponent_heap);
                println!("New deck sizes: You: {}, Them: {}", player.cards.len(), opponent.cards.len());
            },
            // pl = op. War
            Ordering::Equal => {
                println!("Your card: {}. Their card: {}", player_heap.cards[0].to_short_string(), opponent_heap.cards[0].to_short_string());
                println!("Both values are equal. WAR!");
                
                let mut tie = true;
                // let mut pl_sum: u16;
                // let mut op_sum: u16;
                
                while tie {
                    if player.cards.len() < 3 {
                        println!("You didn't have enough cards.");
                        winner = OPPONENT;
                        tie = false;
                    } else if opponent.cards.len() < 3 {
                        println!("They didn't have enough cards.");
                        winner = PLAYER;
                        tie = false;
                    } else {
                        player_heap.extend(player.draw(3));
                        opponent_heap.extend(opponent.draw(3));
                        let pl_sum: u16 = player_heap.cards.iter().rev().take(3)
                        .map(|c| c.get_values(CARD_VALS)[0] as u16).sum();

                    let op_sum: u16 = opponent_heap.cards.iter().rev().take(3)
                    .map(|c| c.get_values(CARD_VALS)[0] as u16).sum();
                match pl_sum.cmp(&op_sum) {
                    Ordering::Less => {
                        // Pl wins
                        tie = false; // Break the while loop
                        player.extend(player_heap.clone());
                        player.extend(opponent_heap.clone());
                        println!("New deck sizes: You: {}, Them: {}", player.cards.len(), opponent.cards.len());
                    },
                    Ordering::Greater => {
                        // Op wins
                        tie = false; // Break the while loop
                        opponent.extend(player_heap.clone());
                        opponent.extend(opponent_heap.clone());
                        println!("New deck sizes: You: {}, Them: {}", player.cards.len(), opponent.cards.len());
                    },
                    Ordering::Equal => {
                        // Do nothing, continue
                        tie = true;
                    },
                }
            }
        }
    },
}

if player.cards.len() == 0 {
    winner = OPPONENT;
} else if opponent.cards.len() == 0 {
            winner = PLAYER;
        }
    }

    match winner {
        PLAYER => println!("You won!"),
        OPPONENT => println!("You lost"),
        _ => unreachable!()
    }

}