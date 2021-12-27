use std::fmt::Display;

/// A card as it may be included in a [CardDeck].
/// 
/// [CardDeck]: crate::cards::CardDeck
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Card {
    text: String,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "({})", self.text)
    }
}

impl Card {
    /// Creates a new Card with the given text
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::cards::Card;
    ///
    /// let card = Card::new("Head");
    /// assert_eq!(card.text(), "Head");
    /// ```
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_owned(),
        }
    }

    /// Returns the text of the Card.
    pub fn text(&self) -> &str {
        &self.text
    }
}
