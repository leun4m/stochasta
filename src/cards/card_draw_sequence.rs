use crate::{Probability, PROBABILITY_ZERO};
use std::hash::Hash;

/// A representation of a sequence of drawn cards.
///
/// # Type Parameters
/// - `C`: The type of a single card
///
/// # See also
///- [`CardDrawTree::paths`](crate::CardDrawTree::paths)
#[derive(Clone, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl<C> Default for CardDrawSequence<C>
where
    C: Eq + Hash,
{
    fn default() -> Self {
        Self {
            cards: Vec::default(),
            probability: PROBABILITY_ZERO,
        }
    }
}

impl<C> CardDrawSequence<C>
where
    C: Eq + Hash,
{
    /// Creates a new `CardDrawSequence` with the given values.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::{CardDrawSequence, PROBABILITY_ONE};
    ///
    /// let sequence = CardDrawSequence::new(vec![1, 2, 3], PROBABILITY_ONE);
    /// assert_eq!(sequence.cards(), &vec![1, 2, 3]);
    /// assert_eq!(sequence.probability(), &PROBABILITY_ONE);
    /// ```
    pub fn new(cards: Vec<C>, probability: Probability) -> Self {
        Self { cards, probability }
    }

    /// Returns the cards as they appear in their sequence.
    pub fn cards(&self) -> &Vec<C> {
        &self.cards
    }

    /// Returns the probability of this sequence to appear.
    pub fn probability(&self) -> &Probability {
        &self.probability
    }
}
