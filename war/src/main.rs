// Warning, code below is AI-generated. Serves as a proof of concept that the helper code works and abstracts away deck manipulation

// Remove comments to slow down the game

mod helper;

use helper::*;
// use std::io::stdin;

const CARD_VALS: [&[u8]; 13] = [ &[13], &[1], &[2], &[3], &[4], &[5], &[6], &[7], &[8], &[9], &[10], &[11], &[12] ];

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    let mut opponent = deck.draw(52 / 2);
    let mut player = deck.draw(52 / 2);
    println!("Given both players {} cards", player.clone().cards.len());
    // println!("Press enter to pass one round");

    let mut round = 1;

    while !player.cards.is_empty() && !opponent.cards.is_empty() {
        // stdin().read_line(&mut String::new()).unwrap();
        println!("\n--- Round {round} ---");

        let player_card = player.draw(1).cards.pop().unwrap();
        let opponent_card = opponent.draw(1).cards.pop().unwrap();

        println!("You play: {}", player_card);
        println!("Opponent plays: {}", opponent_card);

        let player_value = player_card.clone().get_values(CARD_VALS)[0];
        let opponent_value = opponent_card.clone().get_values(CARD_VALS)[0];

        if player_value > opponent_value {
            println!("You win the round!");
            player.cards.insert(0, player_card);
            player.cards.insert(0, opponent_card);
        } else if player_value < opponent_value {
            println!("Opponent wins the round!");
            opponent.cards.insert(0, player_card);
            opponent.cards.insert(0, opponent_card);
        } else {
            println!("WAR!");

            let mut player_war_pile = vec![player_card];
            let mut opponent_war_pile = vec![opponent_card];

            // Check that both players have enough cards for war (3 down, 1 up)
            if player.cards.len() < 4 || opponent.cards.len() < 4 {
                println!("A player doesn't have enough cards for war!");

                // Whoever can't continue loses
                if player.cards.len() < 4 {
                    opponent.cards.append(&mut player_war_pile);
                    opponent.cards.append(&mut opponent_war_pile);
                    opponent.cards.append(&mut player.cards);
                    player.cards.clear();
                } else {
                    player.cards.append(&mut player_war_pile);
                    player.cards.append(&mut opponent_war_pile);
                    player.cards.append(&mut opponent.cards);
                    opponent.cards.clear();
                }
                break;
            }

            // Burn 3 cards each
            player_war_pile.extend(player.draw(3).cards);
            opponent_war_pile.extend(opponent.draw(3).cards);

            // Final face-up war card
            let player_final = player.draw(1).cards.pop().unwrap();
            let opponent_final = opponent.draw(1).cards.pop().unwrap();

            println!(
                "War card - You: {}, Opponent: {}",
                player_final, opponent_final
            );

            player_war_pile.push(player_final.clone());
            opponent_war_pile.push(opponent_final.clone());

            let player_final_val = player_final.get_values(CARD_VALS)[0];
            let opponent_final_val = opponent_final.get_values(CARD_VALS)[0];

            if player_final_val > opponent_final_val {
                println!("You win the war!");
                player.extend(Deck {cards: opponent_war_pile.clone()});
                player.extend(Deck {cards: player_war_pile.clone()});
            } else {
                println!("Opponent wins the war!");
                opponent.extend(Deck {cards: player_war_pile.clone()});
                opponent.extend(Deck {cards: opponent_war_pile.clone()});
            }
        }

        println!(
            "You: {} cards, Opponent: {} cards",
            player.cards.len(),
            opponent.cards.len()
        );

        round += 1;
    }

    println!("\n=== Final Result ===");
    if player.cards.is_empty() {
        println!("You lose!");
    } else {
        println!("You win!");
    }
}
