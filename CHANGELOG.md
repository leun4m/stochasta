# CHANGELOG for stochasta

## Unreleased

## [0.8.2] (2023-06-26)

### Changed

- Update dependency version of
  - `itertools` from 0.10 to 0.11
  - `enumset` from 1.0 to 1.1
- Apply clippy suggestions for more idiomatic rust code

## [0.8.1] (2022-10-13)

### Changed

- activate features for documentation at https://docs.rs

## [0.8.0] (2022-10-13)

### Added

- Feature: `playing_cards` (to create a classic card deck more easily)
  - `PlayingCard`
    - consisting of suit and value
  - `PlayingCardDeck`
    - a builder for `CardDeck<PlayingCard>`
  - `PlayingCardSuit`
    - enum for `♦ ♣ ♥ ♠`
  - `PlayingCardValue`
    - enum for `2 3 4 5 6 7 8 9 10 J Q K A`

### Changed

- `CardDrawTree`
  - `to_graphviz` works with space-containing roots

## [0.7.2] (2022-10-01)

### Added

- `[must_use]` flags
  - `CardDrawSequence::new`
  - `CardDrawSequence::cards`
  - `CardDrawSequence::probability`
  - `Probability::new`
  - `Probability::ratio`
  - `Probability::complementary`

## [0.7.1] (2022-09-14)

### Added

- `ProbabilityRatioError` gains the following traits
  - `Display`
  - `Error`

## [0.7.0] (2022-09-05)

### Added

- `CardDeck` gains the following traits
  - `Display`
  - `Hash`
  - `Ord`
  - `PartialOrd`
- `CardDrawSequence` gains the following traits
  - `Display`
  - `Hash`
  - `Ord`
  - `PartialOrd`
- `CardDrawTree` gains the following traits
  - `Display`
  - `Hash`
  - `Ord`
  - `PartialOrd`

### Changed

- Type parameter `<C>` in `CardDeck` and `CardDrawTree` requires now `Ord` trait
- `CardDrawSequence`
  - derives `PartialEq` instead of manually implementing

## [0.6.2] (2022-09-03)

### Added

- Implementation of `Default` for `Probability`

### Fixed

- `documentation` in Cargo.toml linked to the docs of serde instead of <https://docs.rs/stochasta/>

## [0.6.1] (2022-09-03)

### Fixed

- `CardDrawTree`
  - there was an issue with the calculation of the probabilities in the tree ([#13]) where the sum
    of all leaves would not resolve to 1 since parent probabilities would not be transferred
    properly to child nodes

### Changed

- `CardDrawTree::to_graphviz()`
  - replaced internally `push_str(format!(...))` with `write!`

## [0.6.0] (2022-06-09)

### Added

- `serde`-Support for:
  - `CardDrawSequence`
  - `CardDrawTree`
  - `Probability`

### Changed

- `CardDrawTree::to_graphviz()`
  - uses a prefix (`_`) for the node ids
  - is now generic

## [0.5.0] (2022-02-10)

### Changed

- `Probability` is now a wrapper for `Ratio<u64>` instead of a simple type reference
  ensuring a correct value range.

## [0.4.2] (2022-01-14)

### Changed

- Removed unnecessary `Box`ing of `HashSet` in `CardDrawTree`

## [0.4.0] (2022-01-14)

### Added

- `CardDeck<C>::draw(&self, C)`
- `CardDrawTree<C>::shrinking(&self, u32)`
- `CardDrawTree<C>::paths(&self)`
- `CardDrawSequence<C>` as a representation of a sequence of drawn cards

### Changed

- `CardDeck<C>::probabilities(&self)` - the probabilities are now guaranteed to be `> 0`.

## [0.3.0] (2022-01-01)

### Added

- `CardDrawTree<C>` as a representation of a card drawing process with methods
  - to create an empty one
  - a single and multilayered one from `CardDeck<C>` (without deck shrinking)
  - to create a [Graphviz](https://www.graphviz.org/) representation

### Fixed

- Serde-Serialization works with `CardDeck<C>`

## [0.2.2] (2021-21-31)

### Added

- Constants: `PROBABILITY_ZERO`, `PROBABILITY_ONE`

### Changed

- Read-only `CardDeck<C>` methods got the `#[must_use]` attribute

## [0.2.1] (2021-12-30)

### Fixed

- `CardDeck<C>` documentation

## [0.2.0] (2021-12-30)

### Added

- `CardDeck<C>` as a representation of a deck of cards with methods
  - to add and remove cards
  - to check the number of cards contained
  - to calculate the probability of a single card to be drawn

[#13]: https://github.com/leun4m/stochasta/issues/13
[0.2.0]: https://github.com/leun4m/stochasta/releases/tag/v0.2.0
[0.2.1]: https://github.com/leun4m/stochasta/releases/tag/v0.2.1
[0.2.2]: https://github.com/leun4m/stochasta/releases/tag/v0.2.2
[0.3.0]: https://github.com/leun4m/stochasta/releases/tag/v0.3.0
[0.4.0]: https://github.com/leun4m/stochasta/releases/tag/v0.4.0
[0.4.2]: https://github.com/leun4m/stochasta/releases/tag/v0.4.2
[0.5.0]: https://github.com/leun4m/stochasta/releases/tag/v0.5.0
[0.6.0]: https://github.com/leun4m/stochasta/releases/tag/v0.6.0
[0.6.1]: https://github.com/leun4m/stochasta/releases/tag/v0.6.1
[0.6.2]: https://github.com/leun4m/stochasta/releases/tag/v0.6.2
[0.7.0]: https://github.com/leun4m/stochasta/releases/tag/v0.7.0
[0.7.1]: https://github.com/leun4m/stochasta/releases/tag/v0.7.1
[0.7.2]: https://github.com/leun4m/stochasta/releases/tag/v0.7.2
[0.8.0]: https://github.com/leun4m/stochasta/releases/tag/v0.8.0
[0.8.1]: https://github.com/leun4m/stochasta/releases/tag/v0.8.1
[0.8.2]: https://github.com/leun4m/stochasta/releases/tag/v0.8.2
