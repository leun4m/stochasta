use num_rational::Ratio;
use std::fmt::{Display, Formatter};
use std::ops::Mul;

/// A probability is a [rational number (ℚ)](https://en.wikipedia.org/wiki/Rational_number)
/// in the range of 0 and 1 (both inclusive).
///
/// In other words: if you got a variable `p` from type `Probability`
/// you can be sure about the following: `0 <= p && p <= 1`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Probability {
    ratio: Ratio<u64>,
}

impl Display for Probability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ratio)
    }
}

impl Default for Probability {
    fn default() -> Self {
        PROBABILITY_ZERO
    }
}

impl Mul<Probability> for Probability {
    type Output = Probability;

    fn mul(self, rhs: Probability) -> Self::Output {
        // Multiplication between two probabilities is always safe to be in bounds!
        Self {
            ratio: self.ratio * rhs.ratio,
        }
    }
}

impl From<Ratio<u64>> for Probability {
    /// Creates a new `Probability` from the given ratio.
    ///
    /// # Panics
    ///
    /// - if ratio > 1 ⇒ value out of bounds!
    fn from(ratio: Ratio<u64>) -> Self {
        if ratio > Ratio::from(1) {
            panic!("ratio is not in the bounds of 0 and 1");
        }

        Self { ratio }
    }
}

impl Probability {
    /// Creates a new `Probability`.
    ///
    /// For a safer method (panic-free), please consider using: [Probability::try_new].
    ///
    /// # Panics
    ///
    /// - if numerator > denominator ⇒ ratio > 1 ⇒ value out of bounds!
    /// - if denominator == 0 ⇒ impossible value!
    ///
    /// # Example
    ///
    /// ```
    /// use num_rational::Ratio;
    /// use stochasta::Probability;
    ///
    /// let p = Probability::new(1, 3);
    /// assert_eq!(p.ratio(), &Ratio::new(1, 3));
    /// ```
    pub fn new(numerator: u64, denominator: u64) -> Self {
        Self::from(Ratio::new(numerator, denominator))
    }

    /// Tries to create a new `Probability` from the given ratio.
    ///
    /// # Error
    ///
    /// - ratio > 1 => value out of bounds!
    ///
    /// ```
    /// use num_rational::Ratio;
    /// use stochasta::{Probability, ProbabilityRatioError};
    ///
    /// assert!(Probability::try_new(1, 2).is_ok());
    /// assert_eq!(Probability::try_new(1, 0), Err(ProbabilityRatioError::DenominatorZero));
    /// assert_eq!(Probability::try_new(2, 1), Err(ProbabilityRatioError::RatioGreaterOne));
    /// ```
    pub fn try_new(numerator: u64, denominator: u64) -> Result<Self, ProbabilityRatioError> {
        if denominator == 0 {
            Err(ProbabilityRatioError::DenominatorZero)
        } else if numerator > denominator {
            Err(ProbabilityRatioError::RatioGreaterOne)
        } else {
            Ok(Self {
                ratio: Ratio::new(numerator, denominator),
            })
        }
    }

    /// Returns the inner ratio
    pub fn ratio(&self) -> &Ratio<u64> {
        &self.ratio
    }

    /// Returns the complementary probability: `1 - self`.
    ///
    /// # Example
    ///
    /// ```
    /// use stochasta::Probability;
    ///
    /// let one_third = Probability::new(1, 3);
    /// let two_third = one_third.complementary();
    /// assert_eq!(two_third, Probability::new(2, 3));
    /// ```
    pub fn complementary(&self) -> Self {
        Self {
            ratio: RATIO_ONE - self.ratio,
        }
    }
}

/// A probability of 0%.
///
/// An event with the same probability **must never occur**.
pub const PROBABILITY_ZERO: Probability = Probability { ratio: RATIO_ZERO };

const RATIO_ZERO: Ratio<u64> = Ratio::new_raw(0, 1);

/// A probability of 100%.
///
/// An event with the same probability **must occur.**
pub const PROBABILITY_ONE: Probability = Probability { ratio: RATIO_ONE };

const RATIO_ONE: Ratio<u64> = Ratio::new_raw(1, 1);

/// Errors that may happen when trying to create a probability.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProbabilityRatioError {
    /// The denominator must not be 0. That's a basic math rule!
    DenominatorZero,
    /// The ratio of `Probability` cannot be lower than 0.
    RatioLowerZero,
    /// The ratio of `Probability` cannot be higher than 1.
    RatioGreaterOne,
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_rational::Ratio;

    #[test]
    fn constants() {
        assert_eq!(PROBABILITY_ZERO.ratio(), &Ratio::new(0, 1));
        assert_eq!(PROBABILITY_ONE.ratio(), &Ratio::new(1, 1));
    }

    #[test]
    fn new_standard() {
        assert_eq!(Probability::new(0, 2).ratio(), &Ratio::new(0, 1));
        assert_eq!(Probability::new(1, 2).ratio(), &Ratio::new(1, 2));
        assert_eq!(Probability::new(2, 2).ratio(), &Ratio::new(1, 1));
    }

    #[test]
    #[should_panic]
    fn new_out_of_bounds() {
        Probability::new(2, 1);
    }

    #[test]
    #[should_panic]
    fn new_zero_denominator() {
        Probability::new(1, 0);
    }

    #[test]
    fn from_ratio_standard() {
        assert_eq!(
            Probability::from(Ratio::new(0, 7)).ratio(),
            &Ratio::new(0, 1)
        );
        assert_eq!(
            Probability::from(Ratio::new(4, 9)).ratio(),
            &Ratio::new(4, 9)
        );
        assert_eq!(
            Probability::from(Ratio::new(9, 9)).ratio(),
            &Ratio::new(1, 1)
        );
    }

    #[test]
    #[should_panic]
    fn from_ratio_out_of_bounds() {
        let _ = Probability::from(Ratio::new(2, 1));
    }

    #[test]
    #[should_panic]
    fn from_ratio_zero_denominator() {
        let _ = Probability::from(Ratio::new(1, 0));
    }

    #[test]
    fn derive_copy() {
        let x = Probability::new(1, 3);
        let y = x;
        assert_eq!(x, y);
    }

    #[test]
    fn derive_ord() {
        let one_over_three = Probability::new(1, 3);
        let four_over_seven = Probability::new(4, 7);
        let eight_over_nine = Probability::new(8, 9);
        assert!(one_over_three < four_over_seven);
        assert!(four_over_seven < eight_over_nine);
        assert!(one_over_three < eight_over_nine);
    }

    #[test]
    fn derive_eq() {
        let one_over_four = Probability::new(1, 4);
        let two_over_eight = Probability::new(2, 8);
        assert_eq!(one_over_four, two_over_eight);
        assert_ne!(one_over_four, PROBABILITY_ZERO);
        assert_ne!(one_over_four, PROBABILITY_ONE);
    }
}
