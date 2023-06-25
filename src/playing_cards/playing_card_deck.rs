use std::fmt::Display;

use enumset::EnumSet;
use itertools::Itertools;

use crate::CardDeck;

use super::{
    playing_card_suit::ALL_SUITS, playing_card_value::ALL_VALUES, PlayingCard, PlayingCardSuit,
    PlayingCardValue,
};

/// A builder for quickly creating decks of playing cards.
///
/// # Example
///
/// ```
/// use stochasta::playing_cards::{PlayingCard, PlayingCardDeck, PlayingCardSuit, PlayingCardValue};
///
/// let deck = PlayingCardDeck::new()
///     .value_range(PlayingCardValue::Seven, PlayingCardValue::Ace)
///     .all_suits()
///     .set_count(2)
///     .to_deck();
/// println!("{:?}", deck);
/// assert_eq!(deck.size(), 64);
/// ```
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlayingCardDeck {
    values: EnumSet<PlayingCardValue>,
    suits: EnumSet<PlayingCardSuit>,
    count: u64,
}

impl PlayingCardDeck {
    /// Constructs a new empty deck.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::playing_cards::PlayingCardDeck;
    ///
    /// let deck = PlayingCardDeck::new();
    /// assert!(deck.is_empty())
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            values: EnumSet::new(),
            suits: EnumSet::new(),
            count: 1,
        }
    }

    /// Sets the values.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::playing_cards::{PlayingCard, PlayingCardDeck, PlayingCardSuit, PlayingCardValue};
    ///
    /// let deck = PlayingCardDeck::new()
    ///     .set_values([PlayingCardValue::Ten, PlayingCardValue::Ace])
    ///     .set_suits([PlayingCardSuit::Hearts])
    ///     .to_deck();
    ///
    /// assert_eq!(deck.size(), 2);
    /// assert!(deck.contains(&PlayingCard::new(PlayingCardValue::Ten, PlayingCardSuit::Hearts)));
    /// assert!(deck.contains(&PlayingCard::new(PlayingCardValue::Ace, PlayingCardSuit::Hearts)));
    /// ```
    #[must_use]
    pub fn set_values<I>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = PlayingCardValue>,
    {
        self.values.clear();
        self.values.extend(values);
        self
    }

    /// Sets the suits.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::playing_cards::{PlayingCard, PlayingCardDeck, PlayingCardSuit, PlayingCardValue};
    ///
    /// let deck = PlayingCardDeck::new()
    ///     .set_values([PlayingCardValue::Ace])
    ///     .set_suits([PlayingCardSuit::Hearts, PlayingCardSuit::Clubs])
    ///     .to_deck();
    ///
    /// assert_eq!(deck.size(), 2);
    /// assert!(deck.contains(&PlayingCard::new(PlayingCardValue::Ace, PlayingCardSuit::Hearts)));
    /// assert!(deck.contains(&PlayingCard::new(PlayingCardValue::Ace, PlayingCardSuit::Clubs)));
    /// ```
    #[must_use]
    pub fn set_suits<I>(mut self, suits: I) -> Self
    where
        I: IntoIterator<Item = PlayingCardSuit>,
    {
        self.suits.clear();
        self.suits.extend(suits);
        self
    }

    /// Sets the value range. (both inclusive)
    ///    
    /// # Example
    ///
    /// ```
    /// use stochasta::playing_cards::{PlayingCard, PlayingCardDeck, PlayingCardSuit, PlayingCardValue};
    ///
    /// let deck = PlayingCardDeck::new()
    ///     .value_range(PlayingCardValue::Jack, PlayingCardValue::Ace)
    ///     .set_suits([PlayingCardSuit::Hearts])
    ///     .to_deck();
    ///
    /// assert_eq!(deck.size(), 4);
    /// assert!(deck.contains(&PlayingCard::new(PlayingCardValue::Jack, PlayingCardSuit::Hearts)));
    /// assert!(deck.contains(&PlayingCard::new(PlayingCardValue::Queen, PlayingCardSuit::Hearts)));
    /// assert!(deck.contains(&PlayingCard::new(PlayingCardValue::King, PlayingCardSuit::Hearts)));
    /// assert!(deck.contains(&PlayingCard::new(PlayingCardValue::Ace, PlayingCardSuit::Hearts)));
    /// ```
    #[must_use]
    pub fn value_range(mut self, from: PlayingCardValue, to: PlayingCardValue) -> Self {
        self.values.clear();
        self.values.extend(arr_from_to(&ALL_VALUES, &from, &to));
        self
    }

    /// Sets all `PlayingCardValue`s to be included.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::playing_cards::{PlayingCard, PlayingCardDeck, PlayingCardSuit, PlayingCardValue};
    ///
    /// let deck = PlayingCardDeck::new()
    ///     .all_values()
    ///     .set_suits([PlayingCardSuit::Hearts])
    ///     .to_deck();
    ///
    /// assert_eq!(deck.size(), 13);
    /// assert!(deck.contains(&PlayingCard::new(PlayingCardValue::Two, PlayingCardSuit::Hearts)));
    /// assert!(deck.contains(&PlayingCard::new(PlayingCardValue::Ten, PlayingCardSuit::Hearts)));
    /// assert!(deck.contains(&PlayingCard::new(PlayingCardValue::Ace, PlayingCardSuit::Hearts)));
    /// ```
    #[must_use]
    pub fn all_values(mut self) -> Self {
        self.values.extend(ALL_VALUES);
        self
    }

    /// Sets all `PlayingCardSuit`s inclusive.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::playing_cards::{PlayingCard, PlayingCardDeck, PlayingCardSuit, PlayingCardValue};
    ///
    /// let deck = PlayingCardDeck::new()
    ///     .set_values([PlayingCardValue::Two])
    ///     .all_suits()
    ///     .to_deck();
    ///
    /// assert_eq!(deck.size(), 4);
    /// assert!(deck.contains(&PlayingCard::new(PlayingCardValue::Two, PlayingCardSuit::Diamonds)));
    /// assert!(deck.contains(&PlayingCard::new(PlayingCardValue::Two, PlayingCardSuit::Hearts)));
    /// assert!(deck.contains(&PlayingCard::new(PlayingCardValue::Two, PlayingCardSuit::Clubs)));
    /// assert!(deck.contains(&PlayingCard::new(PlayingCardValue::Two, PlayingCardSuit::Spades)));
    /// ```
    #[must_use]
    pub fn all_suits(mut self) -> Self {
        self.suits.extend(ALL_SUITS);
        self
    }

    /// Sets the count of each individual card.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::playing_cards::{PlayingCard, PlayingCardDeck, PlayingCardSuit, PlayingCardValue};
    ///
    /// let deck = PlayingCardDeck::new()
    ///     .set_values([PlayingCardValue::Two])
    ///     .set_suits([PlayingCardSuit::Hearts])
    ///     .set_count(4)
    ///     .to_deck();
    ///
    /// assert_eq!(deck.size(), 4);
    /// assert!(deck.contains(&PlayingCard::new(PlayingCardValue::Two, PlayingCardSuit::Hearts)));
    /// ```
    #[must_use]
    pub fn set_count(mut self, count: u64) -> Self {
        self.count = count;
        self
    }

    /// Converts this to a [`CardDeck`](crate::CardDeck).
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::playing_cards::{PlayingCard, PlayingCardDeck, PlayingCardSuit, PlayingCardValue};
    ///
    /// let deck = PlayingCardDeck::new()
    ///     .all_values()
    ///     .all_suits()
    ///     .set_count(2)
    ///     .to_deck();
    ///
    /// assert_eq!(deck.size(), 13 * 4 * 2);
    /// ```
    #[must_use]
    pub fn to_deck(&self) -> CardDeck<PlayingCard> {
        let mut deck = CardDeck::new();
        for value in self.values.iter() {
            for suit in self.suits.iter() {
                deck.add_times(PlayingCard::new(value, suit), self.count);
            }
        }
        deck
    }

    /// Returns `true` if deck contains no cards.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::playing_cards::{PlayingCard, PlayingCardDeck, PlayingCardSuit, PlayingCardValue};
    ///
    /// assert!(PlayingCardDeck::new().is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.count == 0 || self.suits.is_empty() || self.values.is_empty()
    }
}

impl Default for PlayingCardDeck {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for PlayingCardDeck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] ({}x)",
            self.suits
                .iter()
                .map(|s| self.values.iter().map(|v| format!("{v}{s}")).join(" "))
                .join(" "),
            self.count
        )
    }
}

fn arr_from_to<T>(arr: &[T], from: &T, to: &T) -> Vec<T>
where
    T: Eq + Clone,
{
    let from_idx = arr.iter().position(|x| x == from).unwrap();
    let to_idx = arr.iter().position(|x| x == to).unwrap();
    arr[from_idx..=to_idx].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_check() {
        let deck = PlayingCardDeck::new()
            .set_suits([PlayingCardSuit::Clubs, PlayingCardSuit::Hearts])
            .set_values([
                PlayingCardValue::Jack,
                PlayingCardValue::Queen,
                PlayingCardValue::King,
            ])
            .set_count(2);

        assert_eq!("[J♣ Q♣ K♣ J♥ Q♥ K♥] (2x)", deck.to_string());
    }
}
