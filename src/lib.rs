//! Stochasta aims to be a simple stochastic analysis library.
//!
//! # Example
//!
//! ```
//! use stochasta::CardDeck;
//! use stochasta::Probability;
//!
//! let coin = CardDeck::from(vec!["heads", "tails"]);
//!
//! assert_eq!(coin.size(), 2);
//! assert_eq!(coin.probability(&"heads"), Probability::new(1, 2));
//! assert_eq!(coin.probability(&"tails"), Probability::new(1, 2));
//! ```

#![deny(missing_docs, unused_imports)]

mod cards;

pub use cards::card_deck::CardDeck;
pub use cards::card_draw_sequence::CardDrawSequence;
pub use cards::card_draw_tree::CardDrawTree;

use num_rational::Ratio;

/// The probability of a certain event
pub type Probability = Ratio<u64>;

/// A probability of 0%.
///
/// An event with the same probability must never occur.
pub const PROBABILITY_ZERO: Probability = Probability::new_raw(0, 1);

/// A probability of 100%.
///
/// An event with the same probability must occur.
pub const PROBABILITY_ONE: Probability = Probability::new_raw(1, 1);

#[cfg(test)]
mod tests {

    use crate::CardDeck;
    use crate::Probability;
    use crate::PROBABILITY_ZERO;

    #[test]
    fn coin_toss_works() {
        let coin = CardDeck::from(vec!["Heads", "Tails"]);
        assert_eq!(coin.size(), 2);
        assert_eq!(coin.probability(&"Heads"), Probability::new(1, 2));
        assert_eq!(coin.probability(&"Tails"), Probability::new(1, 2));
        assert_eq!(coin.probability(&"Edge"), Probability::from(0));
    }

    #[test]
    fn uneven_dice_works() {
        let coin = CardDeck::from(vec!["1", "2", "3", "4", "5", "1"]);

        assert_eq!(coin.size(), 6);
        assert_eq!(coin.probability(&"1"), Probability::new(1, 3));
        assert_eq!(coin.probability(&"2"), Probability::new(1, 6));
        assert_eq!(coin.probability(&"5"), Probability::new(1, 6));
        assert_eq!(coin.probability(&"6"), PROBABILITY_ZERO);
    }
}
