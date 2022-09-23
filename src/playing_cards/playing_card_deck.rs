use std::{collections::BTreeSet, fmt::Display};

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
///     .suits_range(PlayingCardSuit::Diamonds, PlayingCardSuit::Spades)
///     .count(2)
///     .to_deck();
/// println!("{:?}", deck);
/// assert_eq!(deck.size(), 64);
/// ```
///
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PlayingCardDeck {
    values: BTreeSet<PlayingCardValue>,
    suits: BTreeSet<PlayingCardSuit>,
    count: u64,
}

impl PlayingCardDeck {
    /// Constructs a new empty deck.
    pub fn new() -> Self {
        Self {
            values: BTreeSet::new(),
            suits: BTreeSet::new(),
            count: 1,
        }
    }

    /// Sets the value range.
    pub fn values(mut self, values: &[PlayingCardValue]) -> Self {
        self.values.extend(values);
        self
    }

    /// Sets the suit range.
    pub fn suits(mut self, suits: &[PlayingCardSuit]) -> Self {
        self.suits.extend(suits);
        self
    }

    /// Sets the value range. (both inclusive)
    pub fn value_range(mut self, from: PlayingCardValue, to: PlayingCardValue) -> Self {
        self.values.extend(arr_from_to(&ALL_VALUES, &from, &to));
        self
    }

    /// Sets the value range. (both inclusive)
    pub fn suits_range(mut self, from: PlayingCardSuit, to: PlayingCardSuit) -> Self {
        self.suits.extend(arr_from_to(&ALL_SUITS, &from, &to));
        self
    }

    /// Sets all `PlayingCardValue`s inclusive
    pub fn all_values(mut self) -> Self {
        self.values.extend(ALL_VALUES);
        self
    }

    /// Sets all `PlayingCardSuit`s inclusive
    pub fn all_suits(mut self) -> Self {
        self.suits.extend(ALL_SUITS);
        self
    }

    /// Sets the count of cards.
    pub fn count(mut self, count: u64) -> Self {
        self.count = count;
        self
    }

    /// Converts this to a [`CardDeck`](crate::CardDeck).
    pub fn to_deck(&self) -> CardDeck<PlayingCard> {
        let mut deck = CardDeck::new();
        for value in self.values.iter().copied() {
            for suit in self.suits.iter().copied() {
                deck.add_times(PlayingCard::new(value, suit), self.count);
            }
        }
        deck
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
                .map(|s| self.values.iter().map(|v| format!("{}{}", v, s)).join(" "))
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
    fn test_display() {
        let deck = PlayingCardDeck::new()
            .suits(&[PlayingCardSuit::Clubs, PlayingCardSuit::Hearts])
            .values(&[
                PlayingCardValue::Jack,
                PlayingCardValue::Queen,
                PlayingCardValue::King,
            ])
            .count(2);

        assert_eq!("[J♣ Q♣ K♣ J♥ Q♥ K♥] (2x)", deck.to_string());
    }
}
