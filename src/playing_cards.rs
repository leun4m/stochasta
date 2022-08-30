//! Optional module for providing standard playing cards.
//!
//! This is an optional feature which must be activated: `playing_cards`

mod playing_card_suit;
mod playing_card_value;

pub use playing_card_suit::PlayingCardSuit;
pub use playing_card_value::PlayingCardValue;
