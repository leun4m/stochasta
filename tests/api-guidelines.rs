//! Checks the api against the [Rust API Guideline]
//!
//! [Rust API Guideline]: https://rust-lang.github.io/api-guidelines

use stochasta::{CardDeck, CardDrawSequence, CardDrawTree, Probability};

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
    /// - Default
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
            assert!(impls!($input: Default));
        };
    }

    #[test]
    fn check_copy() {
        assert!(impls!(Probability: Copy));
        // assert!(impls!(CardDeck<String>: Copy));
        // assert!(impls!(CardDrawSequence<String>: Copy));
        // assert!(impls!(CardDrawTree<String>: Copy));
    }

    #[test]
    fn check_basics() {
        assert_impls_basics!(Probability);
        assert_impls_basics!(CardDeck<String>);
        assert_impls_basics!(CardDrawSequence<String>);
        assert_impls_basics!(CardDrawTree<String>);
    }
}

/// Checks whether the types conform to [C-SERDE]
///
/// [C-SERDE]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-serde
#[cfg(feature = "serde")]
mod c_serde {
    use super::*;

    #[test]
    fn check_serde() {
        use serde::{Deserialize, Serialize};

        assert!(impls!(Probability: Serialize & Deserialize<'static>));
        assert!(impls!(CardDeck<String>: Serialize & Deserialize<'static>));
        assert!(impls!(CardDrawSequence<String>: Serialize & Deserialize<'static>));
        assert!(impls!(CardDrawTree<String>: Serialize & Deserialize<'static>));
    }
}
