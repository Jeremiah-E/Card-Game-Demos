pub struct Deck {
    pub cards: Vec<Card>
}
pub struct Card {
    face: Face,
    suit: Suit
}
#[derive(Clone)]
pub enum Suit {
    Diamonds(), Spades(), Hearts(), Clubs()
}
#[derive(Clone)]
pub enum Face {
    Ace(), Two(), Three(), Four(), Five(), Six(), Seven(), Eight(), Nine(), Ten(), Jack(), Queen(), King()
}

impl Card {
    pub fn new(face: Face, suit: Suit) -> Card {
        return Card {face, suit};
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Card::new(Face::Queen, Suite::Clubs) -> "Queen of Clubs"
        write!(f, "{} of {}", self.face, self.suit)
    }
}
impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Suit::Diamonds() => "Diamonds",
            Suit::Spades() => "Spades",
            Suit::Hearts() => "Hearts",
            Suit::Clubs() => "Clubs",
        };
        write!(f, "{str}")
    }
}
impl std::fmt::Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Face::Ace() => "Ace",
            Face::Two() => "2",
            Face::Three() => "3",
            Face::Four() => "4",
            Face::Five() => "5",
            Face::Six() => "6",
            Face::Seven() => "7",
            Face::Eight() => "8",
            Face::Nine() => "9",
            Face::Ten() => "10",
            Face::Jack() => "Jack",
            Face::Queen() => "Queen",
            Face::King() => "King",
        };
        write!(f, "{str}")
    }
}
impl Deck {
    pub fn new() -> Deck {
        let suits = [Suit::Clubs(), Suit::Diamonds(), Suit::Hearts(), Suit::Spades()];
        let faces = [Face::Ace(), Face::Two(), Face::Three(), Face::Four(), Face::Five(), Face::Six(), Face::Seven(), Face::Eight(), Face::Nine(), Face::Ten(), Face::Jack(), Face::Queen(), Face::King()];
        let mut cards = Vec::new();
        for suit in suits {
            for face in faces.clone() {
                cards.push(Card::new(face, suit.clone()));
            }
        }
        return Deck {cards};
    }
}