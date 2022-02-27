/// Returns the binomial coefficient for "n choose k"
/// the binomial coefficient is the number of ways of choosing k distinct integers from the set {1, ..., n}
///
/// # Examples
/// ```rust
/// use math::BinomialCoefficient;
///
/// let a = BinomialCoefficient::new(4, 3).unwrap();
/// let b = BinomialCoefficient::new(5, 3).unwrap();
/// let c = BinomialCoefficient::new(9, 4).unwrap();
///
/// assert_eq!(a.get_num_unordered_subsets(), 4);
/// assert_eq!(b.get_num_unordered_subsets(), 10);
/// assert_eq!(c.get_num_unordered_subsets(), 126);
/// ```
///

#[derive(Debug)]
pub enum BinomialCoefficientCreationError {
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
    pub fn new(n: u64, k: u64) -> Result<Self, BinomialCoefficientCreationError> {
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

    pub fn get_num_unordered_subsets(&self) -> u64 {
        let factorial = |x| (1..=x).fold(1, |prev, x| prev * x);
        factorial(self.n) / (factorial(self.k) * factorial(self.n - self.k))
    }
}
