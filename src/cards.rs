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
//! use stochasta::cards;
//! use stochasta::Probability;
//!
//! let d1 = cards::Card::new("1");
//! let d2 = cards::Card::new("2");
//! let d3 = cards::Card::new("3");
//! let d4 = cards::Card::new("4");
//! let d5 = cards::Card::new("5");
//! let d6 = cards::Card::new("6");
//! 
//! let dice = cards::CardDeck::from(vec![&d1, &d2, &d3, &d4, &d5, &d1]);
//! 
//! assert_eq!(dice.size(), 6);
//! 
//! assert_eq!(dice.probability(&d1), Probability::new(1, 3));
//! assert_eq!(dice.probability(&d2), Probability::new(1, 6));
//! assert_eq!(dice.probability(&d5), Probability::new(1, 6));
//! assert_eq!(dice.probability(&d6), Probability::new(0, 6));
//! ```

mod card;
mod card_deck;

pub use card::Card;
pub use card_deck::CardDeck;
