use crate::{Probability, PROBABILITY_ZERO};
use std::{fmt::Debug, fmt::Display, hash::Hash};

/// A representation of a sequence of drawn cards.
///
/// # Type Parameters
/// - `C`: The type of a single card
///
/// # See also
///- [`CardDrawTree::paths`](crate::CardDrawTree::paths)
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CardDrawSequence<C>
where
    C: Eq + Hash,
{
    cards: Vec<C>,
    probability: Probability,
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

impl<C> Display for CardDrawSequence<C>
where
    C: Eq + Hash + Debug + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] ({})",
            self.cards
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("-"),
            self.probability
        )
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
