use rand::{rng, seq::SliceRandom};
pub struct Deck {
    pub cards: Vec<Card>
}
#[derive(Clone)]
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
    pub fn get_values(self, face_values: [&[u8]; 13]) -> Vec<u8> {
        return self.face.get_values(face_values);
    }
    pub fn new(face: Face, suit: Suit) -> Card {
        return Card {face, suit};
    }
    pub fn to_short_string(self) -> String {
        return format!("{}{}", self.face.to_short_string(), self.suit.to_short_string());
    }
}

impl Face {
    pub fn get_values(self, face_values: [&[u8]; 13]) -> Vec<u8> {
        return match self {
            Face::Ace() => {face_values[0]},
            Face::Two() => {face_values[1]},
            Face::Three() => {face_values[2]},
            Face::Four() => {face_values[3]},
            Face::Five() => {face_values[4]},
            Face::Six() => {face_values[5]},
            Face::Seven() => {face_values[6]},
            Face::Eight() => {face_values[7]},
            Face::Nine() => {face_values[8]},
            Face::Ten() => {face_values[9]},
            Face::Jack() => {face_values[10]},
            Face::Queen() => {face_values[11]},
            Face::King() => {face_values[12]},
        }.to_vec();
    }
    pub fn to_short_string(self) -> String {
        return match self {
            Face::Ace() => "A",
            Face::Two() => "2",
            Face::Three() => "3",
            Face::Four() => "4",
            Face::Five() => "5",
            Face::Six() => "6",
            Face::Seven() => "7",
            Face::Eight() => "8",
            Face::Nine() => "9",
            Face::Ten() => "10",
            Face::Jack() => "J",
            Face::Queen() => "Q",
            Face::King() => "K",
        }.to_string();
    }
}

impl Suit {
    pub fn to_short_string(self) -> String {
        return match self {
            Suit::Diamonds() => "♦",
            Suit::Spades() => "♠",
            Suit::Hearts() => "♥",
            Suit::Clubs() => "♣",
        }.to_string();
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
    pub fn to_short_string(self, delimiter: &str) -> String {
        let mut string = String::new();
        for card in self.cards {
            string.push_str(format!("{}{delimiter}", card.to_short_string()).as_str());
        }
        return string;
    }
    pub fn to_string(self, delimiter: &str) -> String {
        let mut string = String::new();
        for card in self.cards {
            string.push_str(format!("{}{delimiter}", card.to_string()).as_str());
        }
        return string;
    }
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
    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }
}