//! Checks the api against the [Rust API Guideline]
//!
//! [Rust API Guideline]: https://rust-lang.github.io/api-guidelines

use stochasta::{CardDeck, CardDrawSequence, CardDrawTree, Probability, ProbabilityRatioError};

#[cfg(playing_cards)]
use stochasta::playing_cards::{PlayingCard, PlayingCardSuit, PlayingCardValue};

#[macro_use]
extern crate impls;

/// Checks whether the types conform to [C-COMMON-TRAITS]
///
/// [C-COMMON-TRAITS]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-common-traits
mod c_common_traits {
    use std::fmt::{Debug, Display};
    use std::hash::Hash;

    use super::{CardDeck, CardDrawSequence, CardDrawTree, Probability, ProbabilityRatioError};

    /// Checks whether the type implements:
    /// - `Clone`
    /// - `Eq`
    /// - `PartialEq`    
    /// - `Ord`
    /// - `PartialOrd`
    /// - `Hash`
    /// - `Debug`
    /// - `Display`
    macro_rules! assert_impls_basics {
        ($input:ty) => {
            assert!(impls!($input: Clone));
            assert!(impls!($input: Eq));
            assert!(impls!($input: PartialEq));
            assert!(impls!($input: Ord));
            assert!(impls!($input: PartialOrd));
            assert!(impls!($input: Hash));
            assert!(impls!($input: Debug));
            assert!(impls!($input: Display));
        };
    }

    #[test]
    fn check_basics() {
        assert_impls_basics!(Probability);
        assert_impls_basics!(ProbabilityRatioError);
        assert_impls_basics!(CardDeck<String>);
        assert_impls_basics!(CardDrawSequence<String>);
        assert_impls_basics!(CardDrawTree<String>);

        #[cfg(playing_cards)]
        {
            assert_impls_basics!(PlayingCard);
            assert_impls_basics!(PlayingCardSuit);
            assert_impls_basics!(PlayingCardValue);
        }
    }

    #[test]
    fn check_copy() {
        assert!(impls!(Probability: Copy));
        assert!(impls!(ProbabilityRatioError: Copy));
        // assert!(impls!(CardDeck<String>: Copy));
        // assert!(impls!(CardDrawSequence<String>: Copy));
        // assert!(impls!(CardDrawTree<String>: Copy));

        #[cfg(playing_cards)]
        {
            assert!(impls!(PlayingCard: Copy));
            assert!(impls!(PlayingCardSuit: Copy));
            assert!(impls!(PlayingCardValue: Copy));
        }
    }

    #[test]
    fn check_default() {
        assert!(impls!(Probability: Default));
        // assert!(impls!(ProbabilityRatioError: Default));
        assert!(impls!(CardDeck<String>: Default));
        assert!(impls!(CardDrawSequence<String>: Default));
        assert!(impls!(CardDrawTree<String>: Default));

        #[cfg(playing_cards)]
        {
            assert!(impls!(PlayingCard: Default));
            assert!(impls!(PlayingCardSuit: Default));
            assert!(impls!(PlayingCardValue: Default));
        }
    }
}

/// Checks whether the types conform to [C-SERDE]
///
/// [C-SERDE]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-serde
#[cfg(feature = "serde")]
mod c_serde {
    use serde::{Deserialize, Serialize};

    use super::{CardDeck, CardDrawSequence, CardDrawTree, Probability, ProbabilityRatioError};

    #[test]
    fn check_serialize() {
        assert!(impls!(Probability: Serialize));
        assert!(impls!(ProbabilityRatioError: Serialize));
        assert!(impls!(CardDeck<String>: Serialize));
        assert!(impls!(CardDrawSequence<String>: Serialize));
        assert!(impls!(CardDrawTree<String>: Serialize));

        #[cfg(playing_cards)]
        {
            assert!(impls!(PlayingCard: Serialize));
            assert!(impls!(PlayingCardSuit: Serialize));
            assert!(impls!(PlayingCardValue: Serialize));
        }
    }

    #[test]
    fn check_deserialize() {
        assert!(impls!(Probability: Deserialize<'static>));
        assert!(impls!(ProbabilityRatioError: Deserialize<'static>));
        assert!(impls!(CardDeck<String>: Deserialize<'static>));
        assert!(impls!(CardDrawSequence<String>: Deserialize<'static>));
        assert!(impls!(CardDrawTree<String>: Deserialize<'static>));

        #[cfg(playing_cards)]
        {
            assert!(impls!(PlayingCard: Deserialize<'static>));
            assert!(impls!(PlayingCardSuit: Deserialize<'static>));
            assert!(impls!(PlayingCardValue: Deserialize<'static>));
        }
    }
}

/// Checks whether the types conform to [C-SEND-SYNC]
///
/// [C-SEND-SYNC]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-send-sync
mod c_send_sync {

    use super::{CardDeck, CardDrawSequence, CardDrawTree, Probability, ProbabilityRatioError};

    #[test]
    fn check_send() {
        assert!(impls!(Probability: Send));
        assert!(impls!(ProbabilityRatioError: Send));
        assert!(impls!(CardDeck<String>: Send));
        assert!(impls!(CardDrawSequence<String>: Send));
        assert!(impls!(CardDrawTree<String>: Send));
        
        #[cfg(playing_cards)]
        {
            assert!(impls!(PlayingCard: Send));
            assert!(impls!(PlayingCardSuit: Send));
            assert!(impls!(PlayingCardValue: Send));
        }
    }

    #[test]
    fn check_sync() {
        assert!(impls!(Probability: Send));
        assert!(impls!(ProbabilityRatioError: Sync));
        assert!(impls!(CardDeck<String>: Sync));
        assert!(impls!(CardDrawSequence<String>: Sync));
        assert!(impls!(CardDrawTree<String>: Sync));

        #[cfg(playing_cards)]
        {
            assert!(impls!(PlayingCard: Sync));
            assert!(impls!(PlayingCardSuit: Sync));
            assert!(impls!(PlayingCardValue: Sync));
        }
    }
}

/// Checks whether the types conform to [C-GOOD-ERR]
///
/// [C-GOOD-ERR]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-good-err
mod c_good_err {
    use std::error::Error;

    use stochasta::ProbabilityRatioError;

    #[test]
    fn check_error() {
        assert!(impls!(ProbabilityRatioError: Error));
    }
}
