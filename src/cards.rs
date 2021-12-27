//! Includes stuff for card-based stochastics.
//!
//! The idea is to have a card deck including unique or multiples of a certain card
//! and to draw cards from the deck.
//!
//! # Example: Uneven dice
//!
//! The following shows how to construct an uneven dice with a second one instead of a six.
//!
//! ```
//! use stochasta::cards::CardDeck;
//! use stochasta::Probability;
//!
//! let dice = CardDeck::from(vec!["1", "2", "3", "4", "5", "1"]);
//!
//! assert_eq!(dice.size(), 6);
//! assert_eq!(dice.probability(&"1"), Probability::new(1, 3));
//! assert_eq!(dice.probability(&"2"), Probability::new(1, 6));
//! assert_eq!(dice.probability(&"5"), Probability::new(1, 6));
//! assert_eq!(dice.probability(&"6"), Probability::new(0, 6));
//! ```

mod card_deck;

pub use card_deck::CardDeck;
