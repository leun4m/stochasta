use num_rational::Ratio;
use stochasta::{CardDeck, CardDrawTree};

fn main() {
    let mut deck = CardDeck::new();
    for i in 1..4 {
        deck.add(i);
    }
    println!("Cards: {}", deck.size());

    // p_draw_sum(&deck, 1);
    // p_draw_sum(&deck, 2);
    p_draw_sum(&deck, 3);
}

fn p_draw_sum(deck: &CardDeck<u8>, draws: u32) {
    let tree = CardDrawTree::shrinking(&deck, draws);
    // println!("{:?}", tree.paths());
    println!(
        "Sum P of {} draws: {:?}\n{}",
        draws,
        tree.paths()
            .iter()
            .map(|x| x.probability().ratio())
            .sum::<Ratio<_>>(),
        tree.to_graphviz()
    );
}
