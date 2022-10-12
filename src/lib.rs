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
//!
//! More examples may be found under the [examples] directory.
//!
//! [examples]: https://github.com/leun4m/stochasta/tree/develop/examples

#![deny(missing_docs, unused_imports)]
#![deny(clippy::pedantic)]
#![allow(
    clippy::implicit_return,
    clippy::module_name_repetitions,
    clippy::non_ascii_literal
)]

mod cards;
mod probability;

pub use cards::card_deck::CardDeck;
pub use cards::card_draw_sequence::CardDrawSequence;
pub use cards::card_draw_tree::CardDrawTree;
pub use probability::Probability;
pub use probability::ProbabilityRatioError;
pub use probability::PROBABILITY_ONE;
pub use probability::PROBABILITY_ZERO;

#[cfg(test)]
mod tests {
    use crate::{CardDeck, CardDrawTree, Probability, PROBABILITY_ZERO};
    use num_rational::Ratio;

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

    fn create_card_deck() -> CardDeck<(&'static str, &'static str)> {
        let mut cards = Vec::new();
        for suit in ["♦", "♥", "♠", "♣"] {
            for value in [
                "2", "3", "4", "5", "6", "7", "8", "9", "10", "B", "D", "K", "A",
            ] {
                cards.push((suit, value));
            }
        }
        CardDeck::from(cards)
    }

    #[test]
    fn poker_hand_specific_order() {
        let deck = create_card_deck();
        let tree = CardDrawTree::shrinking(&deck, 2);
        let probability = tree.probability_of(&[("♦", "A"), ("♥", "A")]);
        assert_eq!(probability, Probability::new(1, 2652));
    }

    #[test]
    fn poker_hand_just_aces() {
        let deck = create_card_deck();
        let tree = CardDrawTree::shrinking(&deck, 2);
        let probability: Probability = tree
            .paths()
            .iter()
            .filter(|seq| seq.cards().iter().all(|card| matches!(card, (_, "A"))))
            .map(|seq| seq.probability().ratio())
            .sum::<Ratio<u64>>()
            .into();
        assert_eq!(probability, Probability::new(6, 1326));
    }
}
