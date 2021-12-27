pub mod cards;

use num_rational::Ratio;

/// The probability of a certain event
pub type Probability = Ratio<u64>;

#[cfg(test)]
mod tests {

    use crate::cards;
    use crate::Probability;

    #[test]
    fn coin_toss_works() {
        let head = cards::Card::new("Head");
        let tails = cards::Card::new("Tails");
        let coin = cards::CardDeck::from(vec![&head, &tails]);
        assert_eq!(coin.size(), 2);
        assert_eq!(coin.probability(&head), Probability::new(1, 2));
        assert_eq!(coin.probability(&tails), Probability::new(1, 2));
        assert_eq!(
            coin.probability(&cards::Card::new("Edge")),
            Probability::from(0)
        );
    }

    #[test]
    fn uneven_dice_works() {
        let d1 = cards::Card::new("1");
        let d2 = cards::Card::new("2");
        let d3 = cards::Card::new("3");
        let d4 = cards::Card::new("4");
        let d5 = cards::Card::new("5");
        let d6 = cards::Card::new("6");

        let coin = cards::CardDeck::from(vec![&d1, &d2, &d3, &d4, &d5, &d1]);

        assert_eq!(coin.size(), 6);

        assert_eq!(coin.probability(&d1), Probability::new(1, 3));
        assert_eq!(coin.probability(&d2), Probability::new(1, 6));
        assert_eq!(coin.probability(&d5), Probability::new(1, 6));
        assert_eq!(coin.probability(&d6), Probability::new(0, 6));
    }
}
