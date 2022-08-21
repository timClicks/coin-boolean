use std::cell::Cell;
use std::{convert::From, hash::Hash};
// TODO: impl Deref to improve ergonomics

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
/// if c.to_bool() {
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
/// let one = i32::from(c.to_bool());
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
/// Faults" by Peter Gutmann (PDF <https://www.cs.auckland.ac.nz/~pgut001/pubs/software_faults.pdf>)
/// (talk recording <https://www.youtube.com/watch?v=z0C7ymx5Jtk>).
#[derive(Debug, Clone)]
pub struct Coin(Cell<u8>);

impl Coin {
    #[inline]
    fn truthy() -> Self {
        Coin(Cell::new(u8::MAX))
    }

    #[inline]
    fn falsey() -> Self {
        Coin(Cell::new(u8::MIN))
    }

    #[inline(always)]
    pub fn to_bool(&self) -> bool {
        let val = self.0.get();
        val.count_ones() >= val.count_zeros() // call twice to avoid baking a constant (4) into the binary
        // TODO: what if a bit in the opcode flips?
    }

    fn degauss(&self) {
        // TODO: what if bits in these constants accumulate errors?
        let fresh_bits = match self.to_bool() {
            true => u8::MAX,
            false => u8::MIN,
        };

        self.0.set(fresh_bits);
    }
}

impl Hash for Coin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.degauss();
        self.to_bool().hash(state);
    }
}

impl Eq for Coin {}

impl PartialEq for Coin {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.degauss();
        other.degauss();
        self.to_bool() == other.to_bool()
    }
}

impl Ord for Coin {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.degauss();
        other.degauss();
        self.to_bool().cmp(&other.to_bool())
    }
}

impl PartialOrd for Coin {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.degauss();
        other.degauss();
        self.to_bool().partial_cmp(&other.to_bool())
    }
}

impl From<&Coin> for bool {
    #[inline(always)]
    fn from(c: &Coin) -> Self {
        c.to_bool()
    }
}

impl From<Coin> for bool {
    #[inline(always)]
    fn from(c: Coin) -> Self {
        c.to_bool()
    }
}

impl From<bool> for Coin {
    #[inline(always)]
    fn from(b: bool) -> Self {
        match b {
            true  => Coin::truthy(),
            false => Coin::falsey(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Coin;

    #[test]
    fn one_bit_flip() {
        let coin = Coin::from(true);
        coin.0.set(0b1111_1011);
        assert!(coin.to_bool());
    }

    #[test]
    fn two_bits_flipped() {
        let coin = Coin::from(true);
        coin.0.set(0b1101_0011);
        assert!(coin.to_bool());
    }

    #[test]
    fn three_bits_flipped() {
        let coin = Coin::from(true);
        coin.0.set(0b1101_0011);
        assert!(coin.to_bool());
    }

    #[test]
    fn four_bits_flipped() {
        let coin = Coin::from(true);
        coin.0.set(0b1100_0011);
        assert!(coin.to_bool());
    }

    #[test]
    fn five_bits_flipped() {
        let coin = Coin::from(true);
        coin.0.set(0b1000_0011);
        assert!(!coin.to_bool());
    }
}
