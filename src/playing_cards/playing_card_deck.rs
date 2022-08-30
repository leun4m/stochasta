use std::ops::RangeInclusive;

use crate::CardDeck;

use super::{PlayingCard, PlayingCardSuit, PlayingCardValue};

/// A builder for quickly creating decks of playing cards.
///
/// # Example
///
/// ```
/// use stochasta::playing_cards::{PlayingCard, PlayingCardDeck, PlayingCardSuit, PlayingCardValue};
///
/// let deck = PlayingCardDeck::new()
///     .values(PlayingCardValue::Seven..=PlayingCardValue::Ace)
///     .suits(PlayingCardSuit::Diamonds..=PlayingCardSuit::Spades)
///     .count(2)
///     .to_deck();
/// println!("{:?}", deck);
/// assert_eq!(deck.size(), 64);
/// ```
pub struct PlayingCardDeck {
    values: RangeInclusive<PlayingCardValue>,
    suits: RangeInclusive<PlayingCardSuit>,
    count: u64,
}

impl PlayingCardDeck {
    /// Constructs a new empty deck.
    pub fn new() -> Self {
        Self {
            values: PlayingCardValue::default()..=PlayingCardValue::default(),
            suits: PlayingCardSuit::default()..=PlayingCardSuit::default(),
            count: 1,
        }
    }

    /// Sets the value range.
    pub fn values(mut self, values: RangeInclusive<PlayingCardValue>) -> Self {
        self.values = values;
        self
    }

    /// Sets the suit range.
    pub fn suits(mut self, suits: RangeInclusive<PlayingCardSuit>) -> Self {
        self.suits = suits;
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
        for value in values_from_to(self.values.start(), self.values.end()) {
            for suit in suits_from_to(self.suits.start(), self.suits.end()) {
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

const ALL_SUITS: [PlayingCardSuit; 4] = [
    PlayingCardSuit::Diamonds,
    PlayingCardSuit::Clubs,
    PlayingCardSuit::Hearts,
    PlayingCardSuit::Spades,
];

const ALL_VALUES: [PlayingCardValue; 13] = [
    PlayingCardValue::Two,
    PlayingCardValue::Three,
    PlayingCardValue::Four,
    PlayingCardValue::Five,
    PlayingCardValue::Six,
    PlayingCardValue::Seven,
    PlayingCardValue::Eight,
    PlayingCardValue::Nine,
    PlayingCardValue::Ten,
    PlayingCardValue::Jack,
    PlayingCardValue::Queen,
    PlayingCardValue::King,
    PlayingCardValue::Ace,
];

fn suits_from_to(from: &PlayingCardSuit, to: &PlayingCardSuit) -> Vec<PlayingCardSuit> {
    arr_from_to(&ALL_SUITS, from, to)
}

fn values_from_to(from: &PlayingCardValue, to: &PlayingCardValue) -> Vec<PlayingCardValue> {
    arr_from_to(&ALL_VALUES, from, to)
}

fn arr_from_to<T>(arr: &[T], from: &T, to: &T) -> Vec<T>
where
    T: Eq + Clone,
{
    let from_idx = arr.iter().position(|x| x == from).unwrap();
    let to_idx = arr.iter().position(|x| x == to).unwrap();
    arr[from_idx..=to_idx].to_vec()
}
