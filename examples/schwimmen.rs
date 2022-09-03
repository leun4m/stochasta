use num_rational::Ratio;
use stochasta::{
    playing_cards::{PlayingCardDeck, PlayingCardValue},
    CardDrawTree, Probability,
};

fn main() {
    let deck = PlayingCardDeck::new()
        .value_range(PlayingCardValue::Seven, PlayingCardValue::Ace)
        .all_suits()
        .to_deck();
    let shrinking = CardDrawTree::shrinking(&deck, 3);

    let p: Probability = shrinking
        .paths()
        .iter()
        .filter(|p| {
            p.cards().iter().any(|x| x.value().is_ace())
                && p.cards()
                    .iter()
                    .filter(|x| x.value().is_picture() || x.value() == PlayingCardValue::Ten)
                    .count()
                    == 2
        })
        .map(|p| *p.probability().ratio())
        .sum::<Ratio<_>>()
        .into();

    println!("{}", p.to_f64());
}
