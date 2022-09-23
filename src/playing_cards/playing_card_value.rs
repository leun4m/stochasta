use std::fmt::Display;

use enumset::EnumSetType;

/// The value of a playing card.
#[derive(EnumSetType, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PlayingCardValue {
    /// 2
    #[default]
    Two,
    /// 3
    Three,
    /// 4
    Four,
    /// 5
    Five,
    /// 6
    Six,
    /// 7
    Seven,
    /// 8
    Eight,
    /// 9
    Nine,
    /// 10
    Ten,
    /// J
    Jack,
    /// Q
    Queen,
    /// K
    King,
    /// A
    Ace,
}

pub const ALL_VALUES: [PlayingCardValue; 13] = [
    PlayingCardValue::Two,
    PlayingCardValue::Three,
    PlayingCardValue::Four,
    PlayingCardValue::Five,
    PlayingCardValue::Six,
    PlayingCardValue::Seven,
    PlayingCardValue::Eight,
    PlayingCardValue::Nine,
    PlayingCardValue::Ten,
    PlayingCardValue::Jack,
    PlayingCardValue::Queen,
    PlayingCardValue::King,
    PlayingCardValue::Ace,
];

impl PlayingCardValue {
    /// Returns `true` if the value is J, Q or K.
    pub fn is_picture(&self) -> bool {
        *self == PlayingCardValue::Jack
            || *self == PlayingCardValue::Queen
            || *self == PlayingCardValue::King
    }

    /// Returns `true` if the value is numeric (2-10).
    pub fn is_number(&self) -> bool {
        !self.is_picture() && !self.is_ace()
    }

    /// Returns `true` if the value is A.
    pub fn is_ace(&self) -> bool {
        *self == PlayingCardValue::Ace
    }

    fn as_str(&self) -> &'static str {
        match self {
            PlayingCardValue::Two => "2",
            PlayingCardValue::Three => "3",
            PlayingCardValue::Four => "4",
            PlayingCardValue::Five => "5",
            PlayingCardValue::Six => "6",
            PlayingCardValue::Seven => "7",
            PlayingCardValue::Eight => "8",
            PlayingCardValue::Nine => "9",
            PlayingCardValue::Ten => "10",
            PlayingCardValue::Jack => "J",
            PlayingCardValue::Queen => "Q",
            PlayingCardValue::King => "K",
            PlayingCardValue::Ace => "A",
        }
    }
}

impl Display for PlayingCardValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::PlayingCardValue;

    #[test]
    fn display_check() {
        assert_eq!(format!("{}", PlayingCardValue::Two), "2");
        assert_eq!(format!("{}", PlayingCardValue::Ten), "10");
        assert_eq!(format!("{}", PlayingCardValue::Ace), "A");
    }

    #[test]
    fn range_check() {
        let skat_deck = PlayingCardValue::Seven..=PlayingCardValue::Ace;
        assert!(!skat_deck.contains(&PlayingCardValue::Two));
        assert!(!skat_deck.contains(&PlayingCardValue::Three));
        assert!(!skat_deck.contains(&PlayingCardValue::Four));
        assert!(!skat_deck.contains(&PlayingCardValue::Five));
        assert!(!skat_deck.contains(&PlayingCardValue::Six));
        assert!(skat_deck.contains(&PlayingCardValue::Seven));
        assert!(skat_deck.contains(&PlayingCardValue::Eight));
        assert!(skat_deck.contains(&PlayingCardValue::Nine));
        assert!(skat_deck.contains(&PlayingCardValue::Ten));
        assert!(skat_deck.contains(&PlayingCardValue::Jack));
        assert!(skat_deck.contains(&PlayingCardValue::Queen));
        assert!(skat_deck.contains(&PlayingCardValue::King));
        assert!(skat_deck.contains(&PlayingCardValue::Ace));
    }
}
