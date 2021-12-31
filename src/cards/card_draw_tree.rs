use crate::{CardDeck, Probability, PROBABILITY_ONE, PROBABILITY_ZERO};
use itertools::Itertools;
use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Debug)]
pub struct CardDrawTree<C>
where
    C: Eq + Hash,
{
    probability: Probability,
    probability_in_tree: Probability,
    nodes: Box<HashMap<C, CardDrawTree<C>>>,
}

impl<C> Default for CardDrawTree<C>
where
    C: Eq + Hash,
{
    fn default() -> Self {
        Self {
            probability: PROBABILITY_ONE,
            probability_in_tree: PROBABILITY_ONE,
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
    pub fn new(probability: Probability, parent_proability: Probability) -> Self {
        Self {
            probability,
            probability_in_tree: parent_proability * probability,
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
    /// let tree = CardDrawTree::create_from(&deck);
    /// ```
    #[must_use]
    pub fn create_from(card_deck: &CardDeck<C>) -> Self {
        let mut tree = Self::new(PROBABILITY_ONE, PROBABILITY_ONE);
        for (card, probability) in card_deck.probabilities() {
            tree.nodes
                .insert(card.clone(), Self::new(probability, PROBABILITY_ONE));
        }
        tree
    }

    pub fn without_drawing(card_deck: &CardDeck<C>, draws: u32) -> Self {
        Self::without_drawing_root_probality(card_deck, draws, PROBABILITY_ONE, PROBABILITY_ONE)
    }

    fn without_drawing_root_probality(
        card_deck: &CardDeck<C>,
        draws: u32,
        probability: Probability,
        partent_probability: Probability,
    ) -> Self {
        let mut tree = Self::new(probability, partent_probability);
        if 0 < draws {
            for (card, card_probability) in card_deck.probabilities() {
                tree.nodes.insert(
                    card.clone(),
                    Self::without_drawing_root_probality(
                        card_deck,
                        draws - 1,
                        card_probability,
                        probability,
                    ),
                );
            }
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

impl CardDrawTree<&str> {
    pub fn to_graphviz(&self) -> String {
        let mut result = String::from("digraph {\n");

        let root = "root";
        result.push_str(&format!("{}[label=\"\", shape=\"circle\"];\n", root));

        let (subtree, _) = self.to_graphviz_iter(root, 1);
        result.push_str(&subtree);

        result.push('}');
        result
    }

    fn to_graphviz_sub(&self, root: &str, card: &str, id: u32) -> (String, u32) {
        let mut result = String::new();
        let new_root = format!("{}_{}", card, id);
        result.push_str(&format!(
            "{}->{}[label=\"{}\"];\n",
            root, new_root, self.probability
        ));
        result.push_str(&format!(
            "{}[label=\"{} ({})\"];\n",
            new_root, card, self.probability_in_tree
        ));

        let (subtree, new_id) = self.to_graphviz_iter(&new_root, id);
        result.push_str(&subtree);

        (result, new_id)
    }

    fn to_graphviz_iter(&self, root: &str, id: u32) -> (String, u32) {
        let mut result = String::new();
        let mut new_id = id;
        for (card, subtree) in self.nodes.iter().sorted_by_key(|&(c, _)| c) {
            let (graphviz, last_id) = subtree.to_graphviz_sub(root, card, new_id + 1);
            new_id = last_id;
            result.push_str(&graphviz);
        }
        (result, new_id)
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

    #[test]
    fn to_graphviz() {
        let coin = CardDeck::from(vec!["head", "head", "tails"]);
        let tree = CardDrawTree::without_drawing(&coin, 2);
        let output = r#"digraph {
root[label="", shape="circle"];
root->head_2[label="2/3"];
head_2[label="head (2/3)"];
head_2->head_3[label="2/3"];
head_3[label="head (4/9)"];
head_2->tails_4[label="1/3"];
tails_4[label="tails (2/9)"];
root->tails_5[label="1/3"];
tails_5[label="tails (1/3)"];
tails_5->head_6[label="2/3"];
head_6[label="head (2/9)"];
tails_5->tails_7[label="1/3"];
tails_7[label="tails (1/9)"];
}"#;
        assert_eq!(tree.to_graphviz(), output);
    }
}
