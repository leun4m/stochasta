# Changelog

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

[0.4.2]: https://github.com/leun4m/stochasta/releases/tag/v0.4.2
[0.4.0]: https://github.com/leun4m/stochasta/releases/tag/v0.4.0
[0.3.0]: https://github.com/leun4m/stochasta/releases/tag/v0.3.0
[0.2.2]: https://github.com/leun4m/stochasta/releases/tag/v0.2.2
[0.2.1]: https://github.com/leun4m/stochasta/releases/tag/v0.2.1
[0.2.0]: https://github.com/leun4m/stochasta/releases/tag/v0.2.0
