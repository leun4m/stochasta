use num_rational::Ratio;
use stochasta::{CardDeck, CardDrawTree};

fn main() {
    let deck = CardDeck::from(vec![1, 2, 3]);
    let tree = CardDrawTree::shrinking(&deck, 3);

    // println!("{:?}", tree);
    println!("{}", tree.to_graphviz());
    assert_eq!(
        tree.paths()
            .iter()
            .map(|x| x.probability().ratio())
            .sum::<Ratio<_>>(),
        Ratio::new(1, 1)
    );
}
