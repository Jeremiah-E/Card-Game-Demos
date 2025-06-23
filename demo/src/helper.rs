pub struct Card {
    face: Face,
    suite: Suite
}
pub enum Suite {
    Diamonds(), Spades(), Hearts(), Clubs()
}
pub enum Face {
    Ace(), Two(), Three(), Four(), Five(), Six(), Seven(), Eight(), Nine(), Ten(), Jack(), Queen(), King()
}

impl Card {
    pub fn new(face: Face, suite: Suite) -> Card {
        return Card {face, suite};
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Card::new(Face::Queen, Suite::Clubs) -> "Queen of Clubs"
        write!(f, "{} of {}", self.face, self.suite)
    }
}
impl std::fmt::Display for Suite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Suite::Diamonds() => "Diamonds",
            Suite::Spades() => "Spades",
            Suite::Hearts() => "Hearts",
            Suite::Clubs() => "Clubs",
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