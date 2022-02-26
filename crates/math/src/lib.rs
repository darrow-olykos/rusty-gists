
/// Returns the binomial coefficient for "n choose k"
/// the binomial coefficient is the number of ways of choosing k distinct integers from the set {1, ..., n}
///
/// # Examples
/// ```rust
/// use rusty_gists::BinomialCoefficient;
///
/// let a = BinomialCoefficient { n: 4, k: 3 };
/// let b = BinomialCoefficient { n: 5, k: 3 };
/// let c = BinomialCoefficient { n: 9, k: 4 };
/// 
///
/// assert_eq!(a.get_num_unordered_subsets().unwrap(), 4);
/// assert_eq!(b.get_num_unordered_subsets().unwrap(), 10);
/// assert_eq!(c.get_num_unordered_subsets().unwrap(), 126);
///
/// ```
///
enum BinomialCoefficientCreationError {
    NumberOfElementsInSetTooLarge(u64),
    NoElementsInSet(u64),
    ChosenLessThanZero(u64),
    ChosenGreaterThanNumberOfElementsInSet(u64, u64),
}
pub struct BinomialCoefficient {
    n: u64,
    k: u64,
}

impl BinomialCoefficient {
    fn new(n: u64, k: u64) -> Result<Self, BinomialCoefficientCreationError> {
        if n == 0 {
            Err(BinomialCoefficientCreationError::NoElementsInSet(n))
        } else if n > 20 {
            Err(BinomialCoefficientCreationError::NumberOfElementsInSetTooLarge(n))
        } else if k > n {
            Err(BinomialCoefficientCreationError::ChosenGreaterThanNumberOfElementsInSet(n, k))
        } else {
            Ok(Self { n, k })
        }
    }

    fn get_num_unordered_subsets(&self) -> u64 {
        let factorial = |x| (1..=x).fold(1, |prev, x| prev * x);
        factorial(self.n) / (factorial(self.k) * factorial(self.n - self.k))
    }
}