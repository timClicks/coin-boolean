use std::convert::From;

/// A bit flip resistant Boolean type
///
/// Prefer `Coin` to `bool` in safety-critical environments with long-lived
/// variables, such as global variables. `Coin` uses the same space as `bool`.
///
/// `Coin` imposes a small runtime and moderate ergonomic costs when being
/// used in places where you would normally use `bool`. Therefore, you'll
/// generally convert it to a standard `bool` as a local variable.
///
/// ## Examples
///
/// To use `Coin` in an `if` expression, it must first be converted to `bool`.
///
/// ```
/// # use coin::Coin;
/// # fn main() {
/// let c = Coin::from(true);
///
/// if bool::from(c) {
///     println!("Clunky, but effective.");
/// }
/// # }
/// ```
///
/// ## Warnings
///
/// Rust's `true` is converted to numeric types, such as `i32`, it becomes 1.
/// This bit pattern (`0b0000_0001`) is considered to be `false` within the
/// internal representation that's used by `Coin`.
///
/// ```
/// # use coin::Coin;
/// # fn main() {
/// let c = Coin::from(true);
/// let one = i32::from(bool::from(c));
/// assert_eq!(Coin::from(one == 1), c);
/// # }
/// ```
///
/// ## Background and implementation notes
///
/// A standard `bool` is truth-biased, because `false` matches a single
/// bit pattern (all zeros). A single bit flip invalidates the value.
///
/// `Coin` counts the number of bits to determine its truth value. When
/// 4 or more bits are 1, the value is interpreted as `true`. `Coin` can
/// tolerate 3 bit flips per byte before an incorrect value is returned.
///
/// For a more thorough introduction, see the talk "Software Security in the Presence of
/// Faults" by Peter Gutman (PDF <https://www.cs.auckland.ac.nz/~pgut001/pubs/software_faults.pdf>)
/// (talk recording <https://www.youtube.com/watch?v=z0C7ymx5Jtk>).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coin(u8);

impl From<Coin> for bool {
    #[inline(always)]
    fn from(c: Coin) -> Self {
        c.0.count_ones() >= 4
    }
}

impl From<bool> for Coin {
    #[inline(always)]
    fn from(b: bool) -> Self {
        match b {
            true  => Coin(0b1111_1111),
            false => Coin(0b0000_0000),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Coin;

    #[test]
    fn one_bit_flip() {
        let mut coin = Coin::from(true);
        coin.0 = 0b1111_1011;
        assert!(bool::from(coin));
    }

    #[test]
    fn two_bits_flipped() {
        let mut coin = Coin::from(true);
        coin.0 = 0b1101_0011;
        assert!(bool::from(coin));
    }

    #[test]
    fn three_bits_flipped() {
        let mut coin = Coin::from(true);
        coin.0 = 0b1101_0011;
        assert!(bool::from(coin));
    }

    #[test]
    fn four_bits_flipped() {
        let mut coin = Coin::from(true);
        coin.0 = 0b1100_0011;
        assert!(bool::from(coin));
    }

    #[test]
    fn five_bits_flipped() {
        let mut coin = Coin::from(true);
        coin.0 = 0b1000_0011;
        assert!(!bool::from(coin));
    }
}
