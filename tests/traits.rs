//! Checks whether the types provide a default.

use stochasta::{CardDeck, CardDrawSequence, CardDrawTree, Probability};

#[test]
fn default_implemented() {
    let _: Probability = Default::default();
    let _: CardDeck<String> = Default::default();
    let _: CardDrawSequence<String> = Default::default();
    let _: CardDrawTree<String> = Default::default();
}
