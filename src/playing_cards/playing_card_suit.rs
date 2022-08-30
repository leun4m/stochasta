use std::fmt::Display;

/// The suit of a playing card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum PlayingCardSuit {
    /// ♦ (diamonds)
    #[default]
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
    /// # Example
    ///
    /// ```
    /// use stochasta::playing_cards::PlayingCardSuit;
    ///
    /// assert_eq!(PlayingCardSuit::Clubs.is_red(), false);
    /// assert_eq!(PlayingCardSuit::Hearts.is_red(), true);
    /// ```
    #[must_use]
    pub fn is_red(&self) -> bool {
        *self == PlayingCardSuit::Hearts || *self == PlayingCardSuit::Diamonds
    }

    /// Returns `true`, if the suit is black.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::playing_cards::PlayingCardSuit;
    ///
    /// assert_eq!(PlayingCardSuit::Clubs.is_black(), true);
    /// assert_eq!(PlayingCardSuit::Hearts.is_black(), false);
    /// ```
    #[must_use]
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

#[cfg(test)]
mod tests {
    use super::PlayingCardSuit;

    #[test]
    fn display_check() {
        assert_eq!(format!("{}", PlayingCardSuit::Clubs), "♣");
    }
}
