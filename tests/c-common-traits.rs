//! Checks whether the types conform to [C-COMMON-TRAITS]
//!
//! [C-COMMON-TRAITS]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-common-traits

use std::fmt::{Debug, Display};
use std::hash::Hash;

use stochasta::{CardDeck, CardDrawSequence, CardDrawTree, Probability};

#[macro_use]
extern crate impls;

/// Checks whether the type implements:
/// - Clone
/// - Eq
/// - PartialEq
/// - Hash
/// - Debug
/// - Display
/// - Default
macro_rules! assert_impls_basics {
    ($input:ty) => {
        assert!(impls!($input: Clone));
        assert!(impls!($input: Eq));
        assert!(impls!($input: PartialEq));
        assert!(impls!($input: Hash));
        assert!(impls!($input: Debug));
        assert!(impls!($input: Display));
        assert!(impls!($input: Default));
    };
}

#[test]
fn check_clone() {
    assert!(impls!(Probability: Clone));
    assert!(impls!(CardDeck<String>: Clone));
    assert!(impls!(CardDrawSequence<String>: Clone));
    assert!(impls!(CardDrawTree<String>: Clone));
}

#[test]
fn check_basics() {
    assert_impls_basics!(Probability);
    assert_impls_basics!(CardDeck<String>);
    assert_impls_basics!(CardDrawSequence<String>);
    assert_impls_basics!(CardDrawTree<String>);
}

#[test]
fn check_copy() {
    assert!(impls!(Probability: Copy));
    // assert!(impls!(CardDeck<String>: Copy));
    // assert!(impls!(CardDrawSequence<String>: Copy));
    // assert!(impls!(CardDrawTree<String>: Copy));
}

#[test]
fn check_ord() {
    assert!(impls!(Probability: Ord & PartialOrd));
}
