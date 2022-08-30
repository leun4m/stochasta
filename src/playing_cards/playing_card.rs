use std::fmt::Display;

use super::{PlayingCardSuit, PlayingCardValue};

/// A standard playing card like **10 ♥** consisting of
/// a [`PlayingCardValue`](crate::playing_cards::PlayingCardValue) and
/// a [`PlayingCardSuit`](crate::playing_cards::PlayingCardSuit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlayingCard {
    value: PlayingCardValue,
    suit: PlayingCardSuit,
}

impl PlayingCard {
    /// Creates a new playing card.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::playing_cards::{PlayingCard, PlayingCardSuit, PlayingCardValue};
    ///
    /// let card = PlayingCard::new(PlayingCardValue::King, PlayingCardSuit::Hearts);
    /// assert_eq!(card.value(), PlayingCardValue::King);
    /// assert_eq!(card.suit(), PlayingCardSuit::Hearts);
    /// ```
    #[must_use]
    pub fn new(value: PlayingCardValue, suit: PlayingCardSuit) -> Self {
        Self { value, suit }
    }

    /// Returns the value of the card
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::playing_cards::{PlayingCard, PlayingCardSuit, PlayingCardValue};
    ///
    /// assert_eq!(
    ///     PlayingCard::new(PlayingCardValue::King, PlayingCardSuit::Hearts).value(),
    ///     PlayingCardValue::King
    /// );
    /// ```
    #[must_use]
    pub fn value(&self) -> PlayingCardValue {
        self.value
    }

    /// Returns the suit of the card
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::playing_cards::{PlayingCard, PlayingCardSuit, PlayingCardValue};
    ///
    /// assert_eq!(
    ///     PlayingCard::new(PlayingCardValue::King, PlayingCardSuit::Hearts).suit(),
    ///     PlayingCardSuit::Hearts
    /// );
    /// ```
    #[must_use]
    pub fn suit(&self) -> PlayingCardSuit {
        self.suit
    }
}

impl Display for PlayingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.suit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_check() {
        let king_hearts = PlayingCard::new(PlayingCardValue::King, PlayingCardSuit::Hearts);
        assert_eq!(format!("{}", king_hearts), "K ♥");
    }
}
