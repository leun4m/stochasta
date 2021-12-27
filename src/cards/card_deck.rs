use crate::Probability;
use std::collections::HashMap;
use std::hash::Hash;

/// A deck of cards.
///
/// This may contain multiple cards which are equal.
#[derive(Clone, Eq, Debug)]
pub struct CardDeck<C>
where
    C: Eq + Hash,
{
    cards: HashMap<C, u64>,
}

impl<C> From<Vec<C>> for CardDeck<C>
where
    C: Eq + Hash + Default,
{
    fn from(cards: Vec<C>) -> Self {
        let mut deck = CardDeck::default();

        for card in cards {
            deck.add(card.into());
        }

        deck
    }
}

impl<C> FromIterator<C> for CardDeck<C>
where
    C: Eq + Hash + Default,
{
    fn from_iter<T>(cards: T) -> Self
    where
        T: std::iter::IntoIterator<Item = C>,
    {
        let mut deck = CardDeck::default();

        for card in cards {
            deck.add(card);
        }

        deck
    }
}

impl<C> PartialEq for CardDeck<C>
where
    C: Eq + Hash,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.cards == rhs.cards
    }
}

impl<C> Default for CardDeck<C>
where
    C: Eq + Hash + Default,
{
    fn default() -> Self {
        Self {
            cards: HashMap::new(),
        }
    }
}

impl<C> CardDeck<C>
where
    C: Eq + Hash,
{
    /// Returns the number of cards in the deck
    pub fn size(&self) -> u64 {
        self.cards.iter().map(|x| x.1).sum()
    }

    /// Returns the probability of an equal card to be drawn.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::Probability;
    /// use stochasta::cards::CardDeck;
    ///
    /// let dice = CardDeck::from(vec!["1", "2", "3", "4", "5", "6"]);
    /// assert_eq!(dice.probability(&"1"), Probability::new(1, 6));
    /// ```
    pub fn probability(&self, card: &C) -> Probability {
        if let Some(count) = self.cards.get(card) {
            Probability::new(*count, self.size())
        } else {
            Probability::new(0, 1)
        }
    }

    /// Adds the given card once to the deck.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::cards::CardDeck;
    ///
    /// let card = "demo";
    /// let mut deck = CardDeck::default();
    /// assert_eq!(deck.count(&card), 0);
    ///
    /// deck.add(card);
    /// assert_eq!(deck.count(&card), 1);
    /// ```
    pub fn add(&mut self, card: C) {
        self.add_times(card, 1);
    }

    /// Adds the card `n` times to the deck.
    pub fn add_times(&mut self, card: C, n: u64) {
        *self.cards.entry(card).or_insert(0) += n;
    }

    /// Checks whether the card is contained at least once in the deck.
    pub fn contains(&self, card: &C) -> bool {
        self.count(&card) > 0
    }

    /// Checks the amount of equal cards.
    pub fn count(&self, card: &C) -> u64 {
        self.cards.get(card).copied().unwrap_or_default()
    }
}
