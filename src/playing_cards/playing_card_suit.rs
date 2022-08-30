use std::fmt::Display;

/// The suit of a playing card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayingCardSuit {
    /// ♦ (diamonds)
    Diamonds,
    /// ♣ (clubs)
    Clubs,
    /// ♥ (hearts)
    Hearts,
    /// ♠ (spades)
    Spades,
}

impl PlayingCardSuit {
    /// Returns `true`, if the suit is red.
    ///
    /// (Hearts or Diamonds)
    pub fn is_red(&self) -> bool {
        *self == PlayingCardSuit::Hearts || *self == PlayingCardSuit::Diamonds
    }

    /// Returns `true`, if the suit is black.
    ///
    /// (Clubs or Spades)
    pub fn is_black(&self) -> bool {
        !self.is_red()
    }

    fn as_char(&self) -> char {
        match self {
            PlayingCardSuit::Diamonds => '♦',
            PlayingCardSuit::Clubs => '♣',
            PlayingCardSuit::Hearts => '♥',
            PlayingCardSuit::Spades => '♠',
        }
    }
}

impl Display for PlayingCardSuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}
