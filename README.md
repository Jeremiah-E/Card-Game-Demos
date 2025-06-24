# Rust Card Games
The goal of this project is twofold:

1. Make a card game engine
2. Learn how Github works

I've already made a Github project [here (Wikipedia Scraper - Jeremiah-E)](https://github.com/Jeremiah-E/Wikipedia-Scraper), but Github was an afterthought meant to make it easier to share what I made there. Here, I want to get more familiar with version management and source control by diving headfirst into something I've only done barebones examples of.

As for the card game engine, it serves as a way to create cool stuff and to build on my knowledge of coding.

# Principles For the Engine

As of now, most decisions for how I want this to go are undecided. For now, I want a .rs file I can paste in any Cargo project that I can import functons from. The engine will revolve around `Card` structs and `Vec<Card>` structs called `Deck`s. As there's many things you can do with a card deck, I want to have a fairly extensive set of impls for ease-of-use, but I also want the code to be game-agnostic, where I have to specify as little about the game as possible to use the functions provided.

Eventually, I want to be able to make a new game solely by writing code in `main()`, no modifications to the engine needed to support the new game. However, this does mean I have to establish limits on what I support with the engine. Games with custom cards, like Uno, Dos, Crazy Eights, etc. probably won't be supported, although I will look into non-standard decks to see what I can implement without compromising other parts of the project. That said, non-standard cards will be looked into a decent ways down the line, so I don't need to worry about it quite yet. As of now, the engine supports the standard 52-card deck [(Standard 52-card deck - Wikipedia)](https://en.wikipedia.org/wiki/Standard_52-card_deck) (Ace-King, 4 suits \[♣♦♥♠\]). The most common alternative is a 54-card deck with 2 jokers, however I will need to find a coherent way to represent the deck without compromising ease-of-use or game agnosticism. I chose to use the French deck as the only supported one for the time being because 1, it's the only one I'm familiar with, and 2, it's the most popular one globally. I hope to expand to other decks in the future, but that would complicate certain algorithms and mean I have to convey more information to `helper.rs` with each function.

# Games made in the engine

1. `Demo` - Not a game, but the first prototype for how I want the project to go
2. `War` - Not particularely skill-based, but serves as a decent starter to a catalogue of games
3. `Blackjack` (WIP) - A game with user input! While still a fairly high chance game, it allows for a bit of strategy
