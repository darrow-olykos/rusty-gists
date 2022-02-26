use std::mem::{align_of, size_of};

/// Returns byte alignment and number of bytes the provided type takes up in memory
///
/// # Arguments
///
/// `T` - any Rust type
///
/// # Meta
/// Demo usage of `#[repr(C)]`, `std::mem::align_of`, and `std::mem::size_of`
///
/// # Examples
/// ```rust
/// use rusty_gists::size_and_align_of;
///
/// #[repr(C)]
/// struct MyStruct {
///     a: u8,
///     b: u32,
///     c: u16,
/// }
///
/// assert_eq!(size_and_align_of::<MyStruct>(), (4, 12));
/// ```
///
/// # Explanation
///
/// - alignment of `MyStruct` is 4 bytes because largest field (`b: u32`) is 4 bytes
/// - size of `MyStruct` is `12 bytes` when using `#[repr(C)]`:
///   - +1 byte (a: u8)
///   - +3 bytes (padding to align the next u32)
///   - +4 bytes (b: u32)
///   - +2 bytes (c: u16) = total of 10 bytes,
///   - +2 bytes (add more padding at end of struct so new total of bytes (12 bytes) is a multiple of its overall alignment (4 bytes))
///
/// # References
///
/// - [`Rust Reference: Type Layout`](https://doc.rust-lang.org/reference/type-layout.html)
/// - [`Rust Nomicon: #[repr(C)]`](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc)
/// - [`std::mem::size_of`](https://doc.rust-lang.org/std/mem/fn.size_of.html)
/// - [`std::mem::align_of`](https://doc.rust-lang.org/std/mem/fn.align_of.html)
///
pub fn size_and_align_of<T>() -> (usize, usize) {
    (align_of::<T>(), size_of::<T>())
}

/// Returns length of something that can be converted into a &str
///
/// # Arguments
///
/// * `s` - can be any type that can be converted into a str reference.
///
/// # Meta
///
/// This is an example of how generics can make library function interface more ergonomic for the consumer.
/// The consumer can provide &str, String, or Box\<str\> and without the rust compiler complaining about these being different types.
///
/// # Examples
///
/// ```rust
/// use rusty_gists::strlen;
///
/// let my_str = "a";
/// let my_string = String::from("bc");
/// let my_boxed_str = String::from("def").into_boxed_str();
///
/// assert_eq!(strlen(my_str), 1);
/// assert_eq!(strlen(my_string), 2);
/// assert_eq!(strlen(my_boxed_str), 3);
/// ```
///
/// # References
///
/// [`std::convert::AsRef`](https://doc.rust-lang.org/std/convert/trait.AsRef.html)
///
pub fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

/// Returns the binomial coefficient for "n choose k"
/// the binomial coefficient is the number of ways of choosing k distinct integers from the set {1, ..., n}
///
/// # Examples
/// ```rust
/// use rusty_gists::binom;
///
/// assert_eq!(binom(4, 3), 4);
/// assert_eq!(binom(5, 3), 10);
/// assert_eq!(binom(9, 4), 126);
/// ```
///
pub fn binom(n: u64, k: u64) -> u64 {
    let factorial = |x| (1..=x).fold(1, |prev, x| prev * x);
    factorial(n) / (factorial(k) * factorial(n - k))
}

#[cfg(test)]
mod tests {}
