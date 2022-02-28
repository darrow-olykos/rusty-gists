// Credit to: https://doc.rust-lang.org/book/ch13-01-closures.html
// (I've modified their example to use a HashMap instead of a single value)
use std::collections::HashMap;

/// cacher for calculation with two u64's as input and u64 as output
/// can be generalized more
pub struct Cacher<T>
where
    T: FnMut(u64, u64) -> u64,
{
    calculation: T,
    values: HashMap<String, u64>,
}

impl<T> Cacher<T>
where
    T: FnMut(u64, u64) -> u64,
{
    /// Returns a Cacher<T> which can cache results of calculations for the provided closure.
    ///
    /// # Arguments
    ///
    /// `T` - Closure that computes produces a value. Value is cached based on args. Cached value is returend on subsequent calls if args are the same.
    ///
    /// # Examples
    /// ```rust
    /// use my_cache::Cacher;
    /// let mut cacher = Cacher::new(|x,y|x+y);
    /// ```
    pub fn new(calculation: T) -> Self {
        let values = HashMap::new();
        Cacher {
            calculation,
            values,
        }
    }

    /// Returns value of calculation `T`. Cached value is returned if unique `n`, `k` pair provided, otherwise calcuation runs and then value is cached.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::rc::Rc;
    /// use std::cell::{RefCell, RefMut};
    ///
    /// use my_cache::Cacher;
    ///
    /// let mut count = Rc::new(RefCell::new(0));
    /// let add = |x, y| {
    ///     let mut count_mut_ref = count.borrow_mut();
    ///     *count_mut_ref += 1; x + y
    /// };
    /// let mut cacher = Cacher::new(add);
    ///
    /// assert_eq!(*count.borrow(), 0);
    /// assert_eq!(cacher.value(2, 3), 5); // new calculation, count += 1
    /// assert_eq!(*count.borrow(), 1);
    /// assert_eq!(cacher.value(2, 3), 5); // repeat, use cache
    /// assert_eq!(*count.borrow(), 1);
    /// assert_eq!(cacher.value(2, 4), 6); // new calculation, count += 1
    /// assert_eq!(*count.borrow(), 2);
    /// ```
    pub fn value(&mut self, n: u64, k: u64) -> u64 {
        let key = n.to_string() + &k.to_string();
        let cached_result = self.values.get(&key);
        if let Some(value) = cached_result {
            *value
        } else {
            let v = (self.calculation)(n, k);
            self.values.insert(key, v);
            v
        }
    }
}
