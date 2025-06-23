pub struct Card {
    face: String,
    suite: String
}

impl Card {
    pub fn new(face: &str, suite: &str) -> Card {
        return Card {face: face.to_string(), suite: suite.to_string()};
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} of {}", self.face, self.suite)
    }
}