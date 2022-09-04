use crate::{CardDeck, CardDrawSequence, Probability, PROBABILITY_ONE, PROBABILITY_ZERO};
use itertools::Itertools;
use std::collections::BTreeMap;
use std::{
    fmt::{Display, Write},
    hash::Hash,
};

/// Prefix used for graphviz ids
const GRAPHVIZ_PREFIX: &str = "_";

/// A representation of a card drawing process.
///
/// # Type Parameters
/// - `C`: The type of a single card
#[derive(Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CardDrawTree<C>
where
    C: Eq + Hash + Ord,
{
    probability: Probability,
    probability_in_tree: Probability,
    nodes: BTreeMap<C, CardDrawTree<C>>,
}

impl<C> Default for CardDrawTree<C>
where
    C: Eq + Hash + Ord + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C> Display for CardDrawTree<C>
where
    C: Eq + Hash + Ord + Clone + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.nodes
                .iter()
                .map(|x| format!(
                    "* {} ({})\n{}",
                    x.0,
                    x.1.probability_in_tree,
                    x.1.to_string()
                        .lines()
                        .map(|x| format!("\t{}", x))
                        .collect::<Vec<String>>()
                        .join("\n")
                ))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl<C> CardDrawTree<C>
where
    C: Eq + Hash + Ord + Clone,
{
    /// Creates a new empty tree.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::CardDrawTree;
    ///
    /// let tree: CardDrawTree<i32> = CardDrawTree::new();
    /// assert!(tree.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            probability: PROBABILITY_ONE,
            probability_in_tree: PROBABILITY_ONE,
            nodes: BTreeMap::new(),
        }
    }

    /// Creates a new empty tree node.
    #[must_use]
    fn new_node(probability: Probability, parent_probability: Probability) -> Self {
        Self {
            probability,
            probability_in_tree: parent_probability * probability,
            nodes: BTreeMap::<C, CardDrawTree<C>>::new(),
        }
    }

    /// Creates a tree with one level from the given card deck.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::{CardDeck, CardDrawTree};
    ///
    /// let deck = CardDeck::from(vec!["heads", "tails"]);
    /// let tree = CardDrawTree::create_from(&deck);
    /// ```
    #[must_use]
    pub fn create_from(card_deck: &CardDeck<C>) -> Self {
        let mut tree = Self::new_node(PROBABILITY_ONE, PROBABILITY_ONE);
        for (card, probability) in card_deck.probabilities() {
            tree.nodes
                .insert(card.clone(), Self::new_node(probability, PROBABILITY_ONE));
        }
        tree
    }

    /// Creates a new tree with the number of `draws` with an unshrinking stack.
    ///
    /// The stack won't shrink from drawing cards;
    /// instead every drawn card is put back to the stack.
    ///
    /// For a shrinking deck, see [`Self::shrinking()`].
    #[must_use]
    pub fn without_shrinking(card_deck: &CardDeck<C>, draws: u32) -> Self {
        Self::without_shrinking_root_probability(card_deck, draws, PROBABILITY_ONE, PROBABILITY_ONE)
    }

    /// Creates a new tree with the number of `draws` with a shrinking stack.
    ///
    /// The stack will shrink from drawing cards;
    /// once a card is drawn it is no longer part of the stack.
    ///
    /// For a non-shrinking deck, see [`Self::without_shrinking()`].
    #[must_use]
    pub fn shrinking(card_deck: &CardDeck<C>, draws: u32) -> Self {
        Self::shrinking_root_probability(card_deck, draws, PROBABILITY_ONE, PROBABILITY_ONE)
    }

    fn without_shrinking_root_probability(
        card_deck: &CardDeck<C>,
        draws: u32,
        probability: Probability,
        parent_probability: Probability,
    ) -> Self {
        let mut tree = Self::new_node(probability, parent_probability);
        if 0 < draws {
            for (card, card_probability) in card_deck.probabilities() {
                tree.nodes.insert(
                    card.clone(),
                    Self::without_shrinking_root_probability(
                        card_deck,
                        draws - 1,
                        card_probability,
                        tree.probability_in_tree,
                    ),
                );
            }
        }
        tree
    }

    fn shrinking_root_probability(
        card_deck: &CardDeck<C>,
        draws: u32,
        probability: Probability,
        parent_probability: Probability,
    ) -> Self {
        let mut tree = Self::new_node(probability, parent_probability);
        if 0 < draws {
            for (card, card_probability) in card_deck.probabilities() {
                let new_stack = card_deck.draw(card.clone());
                tree.nodes.insert(
                    card.clone(),
                    Self::shrinking_root_probability(
                        &new_stack,
                        draws - 1,
                        card_probability,
                        tree.probability_in_tree,
                    ),
                );
            }
        }
        tree
    }

    /// Returns the probability of a certain sequence in the tree.
    ///
    /// The order is important as well as the position - the first entry will be searched among the
    /// root nodes.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::{CardDeck, CardDrawTree, Probability, PROBABILITY_ONE, PROBABILITY_ZERO};
    ///
    /// let coin = CardDeck::from(vec!["H", "T"]);
    /// let tree = CardDrawTree::without_shrinking(&coin, 2);
    ///
    /// assert_eq!(tree.probability_of(&[]), PROBABILITY_ONE);
    /// assert_eq!(tree.probability_of(&["H"]), Probability::new(1, 2));
    /// assert_eq!(tree.probability_of(&["H", "H"]), Probability::new(1, 4));
    /// // 3x heads is impossible when only throwing 2x
    /// assert_eq!(tree.probability_of(&["H", "H", "H"]), PROBABILITY_ZERO);
    /// ```
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

    /// Returns `true` if the tree has no nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns all paths.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::{CardDeck, CardDrawTree, CardDrawSequence, Probability};
    ///
    /// let coin = CardDeck::from(vec!["H", "T"]);
    /// let tree = CardDrawTree::without_shrinking(&coin, 2);
    ///
    /// let result = tree.paths();
    /// let one_quarter = Probability::new(1, 4);
    ///
    /// assert_eq!(result.len(), 4);
    /// assert!(result.contains(&CardDrawSequence::new(vec!["H", "H"], one_quarter)));
    /// assert!(result.contains(&CardDrawSequence::new(vec!["H", "T"], one_quarter)));
    /// assert!(result.contains(&CardDrawSequence::new(vec!["T", "H"], one_quarter)));
    /// assert!(result.contains(&CardDrawSequence::new(vec!["T", "T"], one_quarter)));
    /// ```
    #[must_use]
    pub fn paths(&self) -> Vec<CardDrawSequence<C>> {
        self.create_paths(&[])
    }

    fn create_paths(&self, sequence: &[C]) -> Vec<CardDrawSequence<C>> {
        let mut result = Vec::new();

        if self.is_empty() {
            result.push(CardDrawSequence::new(
                Vec::from(sequence),
                self.probability_in_tree,
            ));
        } else {
            for (card, tree) in self.nodes.iter() {
                let mut s = Vec::new();
                s.extend(sequence.iter().cloned());
                s.push(card.clone());
                result.extend(tree.create_paths(&s));
            }
        }

        result
    }
}

impl<C> CardDrawTree<C>
where
    C: Eq + Hash + Ord + Display,
{
    /// Creates a [Graphviz](https://www.graphviz.org/)-graph from the decision tree.
    ///
    /// # Example
    ///
    /// For a more interesting graph this example covers an oddly weighted coin where *heads* is
    /// twice as likely to be thrown as *tails*.
    ///
    /// ```
    /// use stochasta::{CardDeck, CardDrawTree};
    ///
    /// let odd_coin = CardDeck::from(vec!["heads", "heads", "tails"]);
    /// let tree = CardDrawTree::without_shrinking(&odd_coin, 2);
    /// let output = r#"digraph {
    /// _root[label="", shape="circle"];
    /// _root->_heads_2[label="2/3"];
    /// _heads_2[label="heads (2/3)"];
    /// _heads_2->_heads_3[label="2/3"];
    /// _heads_3[label="heads (4/9)"];
    /// _heads_2->_tails_4[label="1/3"];
    /// _tails_4[label="tails (2/9)"];
    /// _root->_tails_5[label="1/3"];
    /// _tails_5[label="tails (1/3)"];
    /// _tails_5->_heads_6[label="2/3"];
    /// _heads_6[label="heads (2/9)"];
    /// _tails_5->_tails_7[label="1/3"];
    /// _tails_7[label="tails (1/9)"];
    /// }"#;
    /// assert_eq!(tree.to_graphviz(), output);
    /// ```
    ///
    /// # ASCII Visualisation
    ///
    /// This will result in the following graph (here sideways for better visualisation):
    ///
    /// ```plain
    ///                         2/3
    ///                       +-----[ heads (4/9) ]
    ///       2/3            /
    ///     +-----[ heads (2/3) ]
    ///    /                 \  1/3
    ///   /                   +-----[ tails (2/9) ]
    ///  /
    /// O
    ///  \                      2/3
    ///   \                   +-----[ heads (2/9) ]
    ///    \  1/3            /
    ///     +-----[ tails (1/3) ]
    ///                      \  1/3
    ///                       +-----[ tails (1/9) ]
    ///```
    ///
    /// # Output
    ///
    /// - the paths have the probability from their parent node
    /// - the cards have additionally the total probability to reach it from the root node in
    ///   brackets
    #[must_use]
    pub fn to_graphviz(&self) -> String {
        let mut result = String::new();

        let root = "root";

        write!(
            result,
            "digraph {{\n{prefix}{root}[label=\"\", shape=\"circle\"];\n",
            prefix = GRAPHVIZ_PREFIX,
            root = root
        )
        .expect("Not written");

        let (subtree, _) = self.to_graphviz_iter(root, 1);
        result.push_str(&subtree);

        result.push('}');
        result
    }

    fn to_graphviz_sub(&self, root: &str, card: &str, id: u32) -> (String, u32) {
        let mut result = String::new();
        let new_root = format!("{}_{}", card, id);

        write!(
            result,
            "{prefix}{root_id}->{prefix}{node_id}[label=\"{prob_edge}\"];\n\
             {prefix}{node_id}[label=\"{node_label} ({prob_node})\"];\n",
            prefix = GRAPHVIZ_PREFIX,
            root_id = root,
            node_id = new_root,
            node_label = card,
            prob_edge = self.probability,
            prob_node = self.probability_in_tree
        )
        .expect("Not written");

        let (subtree, new_id) = self.to_graphviz_iter(&new_root, id);
        result.push_str(&subtree);

        (result, new_id)
    }

    fn to_graphviz_iter(&self, root: &str, id: u32) -> (String, u32) {
        let mut result = String::new();
        let mut new_id = id;
        for (card, subtree) in self.nodes.iter().sorted_by_key(|&(c, _)| c) {
            let (graphviz, last_id) = subtree.to_graphviz_sub(root, &card.to_string(), new_id + 1);
            new_id = last_id;
            result.push_str(&graphviz);
        }
        (result, new_id)
    }
}

#[cfg(test)]
mod tests {
    use num_rational::Ratio;

    use super::*;

    #[test]
    fn probability_of_empty() {
        let tree: CardDrawTree<i32> = CardDrawTree::default();
        assert_eq!(tree.probability_of(&[]), PROBABILITY_ONE);
    }

    #[test]
    fn probability_of_coin() {
        let coin = CardDeck::from(vec!["heads", "tails"]);
        let tree = CardDrawTree::create_from(&coin);

        assert_eq!(tree.probability_of(&["heads"]), Probability::new(1, 2));
        assert_eq!(tree.probability_of(&["tails"]), Probability::new(1, 2));
        assert_eq!(tree.probability_of(&["side"]), PROBABILITY_ZERO);
        assert_eq!(tree.probability_of(&["heads", "tails"]), PROBABILITY_ZERO);
    }

    #[test]
    fn to_graphviz_empty() {
        let deck = CardDeck::<String>::new();
        let tree = CardDrawTree::without_shrinking(&deck, 1);
        let output = r#"digraph {
_root[label="", shape="circle"];
}"#;
        assert_eq!(tree.to_graphviz(), output);
    }

    #[test]
    fn to_graphviz_number() {
        let odd_coin = CardDeck::from(vec![7, 42]);
        let tree = CardDrawTree::without_shrinking(&odd_coin, 1);
        let output = r#"digraph {
_root[label="", shape="circle"];
_root->_7_2[label="1/2"];
_7_2[label="7 (1/2)"];
_root->_42_3[label="1/2"];
_42_3[label="42 (1/2)"];
}"#;
        assert_eq!(output, tree.to_graphviz());
    }

    #[test]
    fn shrinking_empty() {
        let deck: CardDeck<i32> = CardDeck::new();
        let tree = CardDrawTree::shrinking(&deck, 1);
        assert!(tree.is_empty());
    }

    #[test]
    fn shrinking_multiple_draws() {
        let deck = CardDeck::from(vec![1, 2, 3]);
        let tree = CardDrawTree::shrinking(&deck, 3);
        assert_eq!(tree.probability_of(&[1, 2, 1]), PROBABILITY_ZERO);
        assert_eq!(tree.probability_of(&[1, 2, 2]), PROBABILITY_ZERO);
        assert_eq!(tree.probability_of(&[1, 2, 3]), Probability::new(1, 6));
    }

    #[test]
    fn without_shrinking_multiple_draws() {
        let deck = CardDeck::from(vec![1, 2, 3]);
        let tree = CardDrawTree::without_shrinking(&deck, 3);
        assert_eq!(tree.probability_of(&[1, 2, 1]), Probability::new(1, 27));
        assert_eq!(tree.probability_of(&[1, 2, 2]), Probability::new(1, 27));
        assert_eq!(tree.probability_of(&[1, 2, 3]), Probability::new(1, 27));
    }

    #[test]
    fn shrinking_sum_resolves_to_one() {
        let deck = CardDeck::from(vec![1, 2, 3]);
        let tree = CardDrawTree::shrinking(&deck, 3);

        assert_eq!(
            tree.paths()
                .iter()
                .map(|x| x.probability().ratio())
                .sum::<Ratio<_>>(),
            Ratio::new(1, 1)
        );
    }

    #[test]
    fn without_shrinking_sum_resolves_to_one() {
        let deck = CardDeck::from(vec![1, 2, 3]);
        let tree = CardDrawTree::without_shrinking(&deck, 3);

        assert_eq!(
            tree.paths()
                .iter()
                .map(|x| x.probability().ratio())
                .sum::<Ratio<_>>(),
            Ratio::new(1, 1)
        );
    }

    #[test]
    fn tree_to_string() {
        let deck = CardDeck::from(vec![1, 2, 3]);
        let tree = CardDrawTree::shrinking(&deck, 3);
        assert_eq!(
            r#"* 1 (1/3)
	* 2 (1/6)
		* 3 (1/6)
	* 3 (1/6)
		* 2 (1/6)
* 2 (1/3)
	* 1 (1/6)
		* 3 (1/6)
	* 3 (1/6)
		* 1 (1/6)
* 3 (1/3)
	* 1 (1/6)
		* 2 (1/6)
	* 2 (1/6)
		* 1 (1/6)"#,
            tree.to_string()
        );
    }
}
