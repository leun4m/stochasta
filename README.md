# STOCHASTA

This is `stochasta`. A (hopefully) simple stochastic analysis library.

## Example

```rust
use stochasta::cards::CardDeck;
use stochasta::Probability;

let coin = CardDeck::from(vec!["head", "tails"]);

assert_eq!(coin.size(), 2);
assert_eq!(coin.probability(&"head"), Probability::new(1, 2));
assert_eq!(coin.probability(&"tails"), Probability::new(1, 2));
```
