//! Checks whether the types provide a default.

use stochasta::{Probability, CardDeck, CardDrawSequence, CardDrawTree};

#[test]
fn default_implemented() {
    let _: Probability = Default::default();
    let _: CardDeck<String> = Default::default();
    let _: CardDrawSequence<String> = Default::default();
    let _: CardDrawTree<String> = Default::default();
}
