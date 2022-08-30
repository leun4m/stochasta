use stochasta::playing_cards::{PlayingCardDeck, PlayingCardSuit, PlayingCardValue};

fn main() {
    // let deck = PlayingCardDeck::new()
    //     .values(PlayingCardValue::Seven..=PlayingCardValue::Ace)
    //     .suits(PlayingCardSuit::Diamonds..=PlayingCardSuit::Spades)
    //     .to_deck();
    let deck = PlayingCardDeck::new()
        .value_range(PlayingCardValue::Seven, PlayingCardValue::Ace)
        .all_suits()
        .to_deck();
}
