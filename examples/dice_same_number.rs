use std::io;

use stochasta::{CardDeck, CardDrawTree};

fn main() {
    println!("Number of throws: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let throws: u32 = input.trim().parse().expect("Not a number");

    let dice = CardDeck::from(vec![1, 2, 3, 4, 5, 6]);
    let tree = CardDrawTree::without_shrinking(&dice, throws);
    let probabiltiy = tree.probability_of(&vec![6; throws as usize]);

    println!("Probability of throwing {}x a six: {}", throws, probabiltiy);
}
