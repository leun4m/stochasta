use crate::Probability;
use std::collections::HashMap;
use std::hash::Hash;

/// A deck of cards.
///
/// This may contain multiple cards which are equal.
/// 
/// # Example: Uneven dice
///
/// The following code shows how to construct an uneven dice with a second one instead of a six.
///
/// ```
/// use stochasta::CardDeck;
/// use stochasta::Probability;
///
/// let dice = CardDeck::from(vec!["1", "2", "3", "4", "5", "1"]);
///
/// assert_eq!(dice.size(), 6);
/// assert_eq!(dice.probability(&"1"), Probability::new(1, 3));
/// assert_eq!(dice.probability(&"2"), Probability::new(1, 6));
/// assert_eq!(dice.probability(&"5"), Probability::new(1, 6));
/// assert_eq!(dice.probability(&"6"), Probability::new(0, 6));
/// ```
#[derive(Clone, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
        let mut deck = CardDeck::new();

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
        let mut deck = CardDeck::new();

        for card in cards {
            deck.add(card);
        }

        deck
    }
}

impl<C> Extend<C> for CardDeck<C>
where
    C: Eq + Hash,
{
    fn extend<T: IntoIterator<Item = C>>(&mut self, cards: T) {
        for card in cards {
            self.add(card);
        }
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
        CardDeck::new()
    }
}

impl<C> CardDeck<C>
where
    C: Eq + Hash,
{
    /// Creates a new empty deck.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::CardDeck;
    ///
    /// let cards: CardDeck<i32> = CardDeck::new();
    /// assert_eq!(cards.is_empty(), true);
    /// ```
    pub fn new() -> Self {
        Self {
            cards: HashMap::new(),
        }
    }

    /// Adds the given card once to the deck.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::CardDeck;
    ///
    /// let card = "demo";
    /// let mut deck = CardDeck::new();
    /// assert_eq!(deck.count(&card), 0);
    ///
    /// deck.add(card);
    /// assert_eq!(deck.count(&card), 1);
    /// ```
    pub fn add(&mut self, card: C) {
        self.add_times(card, 1);
    }

    /// Adds the card `n` times to the deck.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::CardDeck;
    ///
    /// let card = "demo";
    /// let mut deck = CardDeck::new();
    /// assert_eq!(deck.count(&card), 0);
    ///
    /// deck.add_times(card, 5);
    /// assert_eq!(deck.count(&card), 5);
    /// ```
    pub fn add_times(&mut self, card: C, n: u64) {
        *self.cards.entry(card).or_insert(0) += n;
    }

    /// Removes all appearences of the given card from the deck.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::CardDeck;
    ///
    /// let mut deck = CardDeck::from(vec![1, 3, 3]);
    /// assert_eq!(deck.count(&3), 2);
    ///
    /// deck.remove_all(&3);
    /// assert_eq!(deck.count(&3), 0);
    /// ```
    pub fn remove_all(&mut self, card: &C) {
        self.cards.remove(card);
    }

    /// Removes the card `n` times from the deck.
    ///
    /// If `n` is greater than [count](CardDeck::count) the amount will simply set to zero.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::CardDeck;
    ///
    /// let mut deck = CardDeck::from(vec![3, 3, 3]);
    /// assert_eq!(deck.count(&3), 3);
    ///
    /// deck.remove_times(3, 2);
    /// assert_eq!(deck.count(&3), 1);
    /// ```
    pub fn remove_times(&mut self, card: C, n: u64) {
        let value = self.cards.entry(card).or_insert(0);
        if *value >= n {
            *value -= n;
        } else {
            *value = 0;
        }
    }

    /// Sets the amount of `card`s to `n. Will overwrite any pre-existing value.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::CardDeck;
    ///
    /// let mut deck = CardDeck::new();
    /// assert_eq!(deck.count(&"alpha"), 0);
    ///
    /// deck.set_card("alpha", 10);
    /// assert_eq!(deck.count(&"alpha"), 10);
    /// ```
    pub fn set_card(&mut self, card: C, n: u64) {
        self.cards.insert(card, n);
    }

    /// Returns `true`, if the deck is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::CardDeck;
    ///
    /// let cards: CardDeck<i32> = CardDeck::new();
    /// // ...
    /// assert_eq!(cards.is_empty(), cards.size() == 0);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Returns the number of cards in the deck.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::CardDeck;
    ///
    /// let weird_dice = CardDeck::from(vec![1, 2, 1]);
    /// assert_eq!(weird_dice.size(), 3);
    /// ```
    pub fn size(&self) -> u64 {
        self.cards.iter().map(|x| x.1).sum()
    }

    /// Returns the probability of an equal card to be drawn.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::Probability;
    /// use stochasta::CardDeck;
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

    /// Returns the probability of the cards to be drawn.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use stochasta::Probability;
    /// use stochasta::CardDeck;
    ///
    /// let coin = CardDeck::from(vec!["head", "tails"]);
    /// assert_eq!(
    ///     coin.probabilities(),
    ///     HashMap::from([
    ///         (&"head",  Probability::new(1, 2)),
    ///         (&"tails", Probability::new(1, 2))
    ///     ])
    /// );
    /// ```
    pub fn probabilities(&self) -> HashMap<&C, Probability> {
        let size = self.size();
        self.cards
            .iter()
            .map(|(card, count)| (card, Probability::new(*count, size)))
            .collect()
    }

    /// Checks whether the card is contained at least once in the deck.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::CardDeck;
    ///
    /// let card = "demo";
    /// let mut deck = CardDeck::new();
    /// assert_eq!(deck.contains(&card), false);
    ///
    /// deck.add(card);
    /// assert_eq!(deck.contains(&card), true);
    /// ```
    pub fn contains(&self, card: &C) -> bool {
        self.count(&card) > 0
    }

    /// Checks the amount of equal cards.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::CardDeck;
    ///
    /// let card = "demo";
    /// let mut deck = CardDeck::from(vec![1, 3, 3]);
    ///
    /// assert_eq!(deck.count(&1), 1);
    /// assert_eq!(deck.count(&3), 2);
    /// assert_eq!(deck.count(&5), 0);
    /// ```
    pub fn count(&self, card: &C) -> u64 {
        self.cards.get(card).copied().unwrap_or_default()
    }
}
