# STOCHASTA

This is `stochasta`.

> A (hopefully) simple stochastic analysis library.

It is written in [Rust](https://rust-lang.org) and published at [crates.io](https://crates.io/crates/stochasta).

## Example

```rust
use stochasta::CardDeck;
use stochasta::Probability;

fn main() {
    let coin = CardDeck::from(vec!["heads", "tails"]);
    
    assert_eq!(coin.size(), 2);
    assert_eq!(coin.probability(&"heads"), Probability::new(1, 2));
    assert_eq!(coin.probability(&"tails"), Probability::new(1, 2));
}
```

More examples can be found in the [examples](examples/) directory ([`cargo run --example <example-name>`](https://doc.rust-lang.org/cargo/commands/cargo-run.html#examples)).

## Documentation

The complete documentation is hosted at [docs.rs/stochasta](https://docs.rs/stochasta).

## Changelog

For Changelog / News / History / Releases, see [CHANGELOG.md](CHANGELOG.md).

## Bugs / Issues / Feature Requests

If there is anything at your disfavor or something behaves weird, please submit an [issue](https://github.com/leun4m/stochasta/issues) or open a pull request.
