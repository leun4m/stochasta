/// A [Binomial coefficient](https://en.wikipedia.org/wiki/Binomial_coefficient).
///
/// Described as 'n over k'
#[derive(Default)]
pub struct BinomialCoefficient {
    n: u64,
    k: u64,
}

impl BinomialCoefficient {
    /// Creates a new `BinomialCoefficient`
    /// 
    /// # Panics
    /// - if n < k
    pub fn new(n: u64, k: u64) -> Self {
        assert!(n >= k);

        Self { n, k }
    }

    /// Tries to create a new `BinomialCoefficient`
    ///
    /// This will return None if invalid.
    pub fn try_new(n: u64, k:u64) -> Option<Self> {
        if n >= k {
            Some(Self {n, k})
        } else {
            None
        }
    }

    /// Returns `n`
    pub fn n(&self) -> u64 {
        self.n
    }

    /// Returns `k`
    pub fn k(&self) -> u64 {
        self.k
    }
}
