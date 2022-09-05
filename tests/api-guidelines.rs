//! Checks the api against the [Rust API Guideline]
//!
//! [Rust API Guideline]: https://rust-lang.github.io/api-guidelines

use stochasta::{CardDeck, CardDrawSequence, CardDrawTree, Probability, ProbabilityRatioError};

#[macro_use]
extern crate impls;

/// Checks whether the types conform to [C-COMMON-TRAITS]
///
/// [C-COMMON-TRAITS]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-common-traits
mod c_common_traits {
    use std::fmt::{Debug, Display};
    use std::hash::Hash;

    use super::*;

    /// Checks whether the type implements:
    /// - Clone
    /// - Eq
    /// - PartialEq    
    /// - Ord
    /// - PartialOrd
    /// - Hash
    /// - Debug
    /// - Display
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
    }

    #[test]
    fn check_copy() {
        assert!(impls!(Probability: Copy));
        assert!(impls!(ProbabilityRatioError: Copy));
        // assert!(impls!(CardDeck<String>: Copy));
        // assert!(impls!(CardDrawSequence<String>: Copy));
        // assert!(impls!(CardDrawTree<String>: Copy));
    }

    #[test]
    fn check_default() {
        assert!(impls!(Probability: Default));
        // assert!(impls!(ProbabilityRatioError: Default));
        assert!(impls!(CardDeck<String>: Default));
        assert!(impls!(CardDrawSequence<String>: Default));
        assert!(impls!(CardDrawTree<String>: Default));
    }
}

/// Checks whether the types conform to [C-SERDE]
///
/// [C-SERDE]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-serde
#[cfg(feature = "serde")]
mod c_serde {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[test]
    fn check_serialize() {
        assert!(impls!(Probability: Serialize));
        assert!(impls!(ProbabilityRatioError: Serialize));
        assert!(impls!(CardDeck<String>: Serialize));
        assert!(impls!(CardDrawSequence<String>: Serialize));
        assert!(impls!(CardDrawTree<String>: Serialize));
    }

    #[test]
    fn check_deserialize() {
        assert!(impls!(Probability: Deserialize<'static>));
        assert!(impls!(ProbabilityRatioError: Deserialize<'static>));
        assert!(impls!(CardDeck<String>: Deserialize<'static>));
        assert!(impls!(CardDrawSequence<String>: Deserialize<'static>));
        assert!(impls!(CardDrawTree<String>: Deserialize<'static>));
    }
}

/// Checks whether the types conform to [C-SEND-SYNC]
///
/// [C-SEND-SYNC]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-send-sync
mod c_send_sync {
    use super::*;

    #[test]
    fn check_send() {
        assert!(impls!(Probability: Send));
        assert!(impls!(ProbabilityRatioError: Send));
        assert!(impls!(CardDeck<String>: Send));
        assert!(impls!(CardDrawSequence<String>: Send));
        assert!(impls!(CardDrawTree<String>: Send));
    }

    #[test]
    fn check_sync() {
        assert!(impls!(Probability: Send));
        assert!(impls!(ProbabilityRatioError: Sync));
        assert!(impls!(CardDeck<String>: Sync));
        assert!(impls!(CardDrawSequence<String>: Sync));
        assert!(impls!(CardDrawTree<String>: Sync));
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
