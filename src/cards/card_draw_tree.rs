use std::{collections::HashMap, hash::Hash};

use crate::{CardDeck, Probability, PROBABILITY_ONE, PROBABILITY_ZERO};

pub struct CardDrawTree<C>
where
    C: Eq + Hash,
{
    probability: Probability,
    nodes: Box<HashMap<C, CardDrawTree<C>>>,
}

impl<C> Default for CardDrawTree<C>
where
    C: Eq + Hash,
{
    fn default() -> Self {
        Self {
            probability: PROBABILITY_ONE,
            nodes: Box::default(),
        }
    }
}

impl<C> CardDrawTree<C>
where
    C: Eq + Hash + Clone,
{
    /// Creates a new empty tree.
    #[must_use]
    pub fn new(probability: Probability) -> Self {
        Self {
            probability,
            nodes: Box::new(HashMap::<C, CardDrawTree<C>>::new()),
        }
    }

    /// Creates a tree with one level from the given card deck.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::{CardDeck, CardDrawTree};
    ///
    /// let deck = CardDeck::from(vec!["head", "tails"]);
    /// let tree = CardDrawTree::create_from(deck);
    /// ```
    #[must_use]
    pub fn create_from(card_deck: &CardDeck<C>) -> Self {
        let mut tree = Self::new(PROBABILITY_ONE);
        for (card, probability) in card_deck.probabilities() {
            tree.nodes.insert(card.clone(), Self::new(probability));
        }
        tree
    }

    /// Returns the probability of a certain sequence in the tree.
    #[must_use]
    pub fn probability_of(&self, sequence: &[C]) -> Probability {
        if sequence.is_empty() {
            PROBABILITY_ONE
        } else if let Some(node) = self.nodes.get(&sequence[0]) {
            node.probability * node.probability_of(&sequence[1..])
        } else {
            PROBABILITY_ZERO
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probability_of_empty() {
        let tree: CardDrawTree<i32> = CardDrawTree::default();
        assert_eq!(tree.probability_of(&[]), PROBABILITY_ONE);
    }

    #[test]
    fn probability_of_coin() {
        let coin = CardDeck::from(vec!["head", "tails"]);
        let tree = CardDrawTree::create_from(&coin);

        assert_eq!(tree.probability_of(&["head"]), Probability::new(1, 2));
        assert_eq!(tree.probability_of(&["tails"]), Probability::new(1, 2));
        assert_eq!(tree.probability_of(&["side"]), PROBABILITY_ZERO);
        assert_eq!(tree.probability_of(&["head", "tails"]), PROBABILITY_ZERO);
    }
}
