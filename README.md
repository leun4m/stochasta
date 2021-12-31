# STOCHASTA

This is `stochasta`.

> A (hopefully) simple stochastic analysis library.

It is written in [Rust](https://rust-lang.org) and published at [crates.io](https://crates.io/crates/stochasta).

## Example

```rust
use stochasta::CardDeck;
use stochasta::Probability;

let coin = CardDeck::from(vec!["head", "tails"]);

assert_eq!(coin.size(), 2);
assert_eq!(coin.probability(&"head"), Probability::new(1, 2));
assert_eq!(coin.probability(&"tails"), Probability::new(1, 2));
```

## Changelog

*For Changelog / News / History / Releases, see [CHANGELOG.md](CHANGELOG.md).*

## Bugs / Issues / Feature Requests

If there is anything at your disfavor or something behaves weird, please submit an [issue](https://github.com/leun4m/stochasta/issues) or open a pull request.
