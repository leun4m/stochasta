//! Stochasta aims to be a simple stochastic analysis library.
//!
//! # Example
//!
//! ```
//! use stochasta::cards::CardDeck;
//! use stochasta::Probability;
//!
//! let coin = CardDeck::from(vec!["head", "tails"]);
//!
//! assert_eq!(coin.size(), 2);
//! assert_eq!(coin.probability(&"head"), Probability::new(1, 2));
//! assert_eq!(coin.probability(&"tails"), Probability::new(1, 2));
//! ```

pub mod cards;

use num_rational::Ratio;

/// The probability of a certain event
pub type Probability = Ratio<u64>;

#[cfg(test)]
mod tests {

    use crate::cards;
    use crate::Probability;

    #[test]
    fn coin_toss_works() {
        let coin = cards::CardDeck::from(vec!["Head", "Tails"]);
        assert_eq!(coin.size(), 2);
        assert_eq!(coin.probability(&"Head"), Probability::new(1, 2));
        assert_eq!(coin.probability(&"Tails"), Probability::new(1, 2));
        assert_eq!(coin.probability(&"Edge"), Probability::from(0));
    }

    #[test]
    fn uneven_dice_works() {
        let coin = cards::CardDeck::from(vec!["1", "2", "3", "4", "5", "1"]);

        assert_eq!(coin.size(), 6);
        assert_eq!(coin.probability(&"1"), Probability::new(1, 3));
        assert_eq!(coin.probability(&"2"), Probability::new(1, 6));
        assert_eq!(coin.probability(&"5"), Probability::new(1, 6));
        assert_eq!(coin.probability(&"6"), Probability::new(0, 6));
    }
}
