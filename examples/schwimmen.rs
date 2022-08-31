use num_rational::Ratio;
use stochasta::{
    playing_cards::{PlayingCardDeck, PlayingCardValue},
    CardDrawTree, Probability, PROBABILITY_ZERO,
};

fn main() {
    let deck = PlayingCardDeck::new()
        .value_range(PlayingCardValue::Seven, PlayingCardValue::Ace)
        .all_suits()
        .to_deck();
    let shrinking = CardDrawTree::shrinking(&deck, 3);
    // println!("{}", shrinking.to_graphviz());
    let p: Vec<_> = shrinking
        .paths()
        .iter()
        .map(|p| *p.probability().ratio())
        .collect();

    // .filter(|p| {
    //     p.cards().iter().any(|x| x.value().is_ace())
    //         // && p.cards()
    //         //     .iter()
    //         //     .filter(|x| x.value().is_picture() || x.value() == PlayingCardValue::Ten)
    //         //     .count()
    //         //     == 2
    // })
    // .map(|p| *p.probability().ratio())
    // .collect();
    // .sum();

    // .reduce(|acc, item| Probability::from(acc.ratio() + item.ratio()))
    // .unwrap_or(PROBABILITY_ZERO);
    println!("{:?}", p);
}
