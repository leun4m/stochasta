use crate::{Probability, PROBABILITY_ZERO};
use std::hash::Hash;

/// A representation of a sequence of drawn cards.
#[derive(Clone, Eq, Debug)]
pub struct CardDrawSequence<C>
where
    C: Eq + Hash,
{
    cards: Vec<C>,
    probability: Probability,
}

impl<C> PartialEq for CardDrawSequence<C>
where
    C: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.probability == other.probability
    }
}

impl<C> CardDrawSequence<C>
where
    C: Eq + Hash,
{
    pub fn empty() -> Self {
        Self {cards: Vec::new(), probability: PROBABILITY_ZERO}
    }

    pub fn new(cards: Vec<C>, probability: Probability) -> Self {
        Self { cards, probability }
    }

    pub fn append_card(&mut self, card: C) {
        self.cards.push(card);
    }

    pub fn set_probability(&mut self, probability: Probability) {
        self.probability = probability;
    }
}
