use crate::cards::Card;
use crate::Probability;
use std::collections::HashMap;

/// A deck of [Cards][Card].
/// 
/// This may contain multiple cards which are equal.
/// 
/// [Card]: crate::cards::Card
#[derive(Clone, Eq, Debug)]
pub struct CardDeck {
    cards: HashMap<Card, u64>,
}

impl From<Vec<&Card>> for CardDeck {
    fn from(t: Vec<&Card>) -> Self {
        let mut cards: HashMap<Card, u64> = HashMap::new();

        for card in t {
            *cards.entry(card.clone()).or_insert(0) += 1;
        }

        Self { cards }
    }
}

impl FromIterator<Card> for CardDeck {
    fn from_iter<T>(t: T) -> Self
    where
        T: std::iter::IntoIterator<Item = Card>,
    {
        let mut cards: HashMap<Card, u64> = HashMap::new();

        for card in t {
            *cards.entry(card.clone()).or_insert(0) += 1;
        }

        Self { cards }
    }
}

impl FromIterator<&'static str> for CardDeck {
    fn from_iter<T>(t: T) -> Self
    where
        T: std::iter::IntoIterator<Item = &'static str>,
    {
        let mut cards: HashMap<Card, u64> = HashMap::new();

        for card in t {
            *cards.entry(Card::new(card)).or_insert(0) += 1;
        }

        Self { cards }
    }
}

impl PartialEq for CardDeck {
    fn eq(&self, rhs: &Self) -> bool {
        self.cards == rhs.cards
    }
}

impl Default for CardDeck {
    fn default() -> Self {
        Self {
            cards: HashMap::new(),
        }
    }
}

impl CardDeck {
    /// Returns the number of cards in the deck
    pub fn size(&self) -> u64 {
        self.cards.iter().map(|x| x.1).sum()
    }

    /// Returns the probability of an equal card to be drawn.
    ///
    /// ```
    /// use stochasta::Probability;
    /// use stochasta::cards::Card;
    /// use stochasta::cards::CardDeck;
    ///
    /// let dice = CardDeck::from_iter(vec!["1", "2", "3", "4", "5", "6"]);
    /// assert_eq!(dice.probability(&Card::new("1")), Probability::new(1, 6));
    /// ```
    pub fn probability(&self, card: &Card) -> Probability {
        if let Some(count) = self.cards.get(card) {
            Probability::new(*count, self.size())
        } else {
            Probability::new(0, 1)
        }
    }
}
