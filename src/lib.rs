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

mod cards;
mod probability;

pub use cards::card_deck::CardDeck;
pub use cards::card_draw_sequence::CardDrawSequence;
pub use cards::card_draw_tree::CardDrawTree;
pub use probability::Probability;
pub use probability::PROBABILITY_ONE;
pub use probability::PROBABILITY_ZERO;

#[cfg(test)]
mod tests {

    use crate::{CardDeck, Probability, PROBABILITY_ZERO};

    #[test]
    fn coin_toss_works() {
        let coin = CardDeck::from(vec!["Heads", "Tails"]);
        assert_eq!(coin.size(), 2);
        assert_eq!(coin.probability(&"Heads"), Probability::new(1, 2));
        assert_eq!(coin.probability(&"Tails"), Probability::new(1, 2));
        assert_eq!(coin.probability(&"Edge"), PROBABILITY_ZERO);
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
