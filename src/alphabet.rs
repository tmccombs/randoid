//! Definitions for the [`Alphabet`] type and common alphabets to use.
//!
//! Inlcuding the default alphabet.

/// Type for an alphabet to use for generating ids
///
/// It has a fixed length, because that can provide the compiler
/// with more optimization opportunities, and in almost all cases
/// the alphabet used will be a constant anyway.
#[derive(Debug)]
pub struct Alphabet<const N: usize = 64>(pub(crate) [char; N]);

/// Type for alphabet with 16 possible characters
pub type HexAlphabet = Alphabet<16>;

impl Default for &'static Alphabet {
    fn default() -> Self {
        &DEFAULT
    }
}

impl Default for &'static HexAlphabet {
    fn default() -> Self {
        &HEX
    }
}

impl<const N: usize> Alphabet<N> {
    /// Create a new alphabe from a set of characters
    ///
    /// The length of the array should be at least 1 and at most `u8::MAX`.
    ///
    /// Each element of the array should be unique.
    ///
    /// # Panics
    ///
    /// Panics if the number of character is greater than the maximum value of a u8,
    /// since no possible random byte would be able to map to some values. Similarly,
    /// an empty alphabet will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// # use randoid::Alphabet;
    /// let alph = Alphabet::new(['1', '2', '3', '4']);
    /// ```
    ///
    /// ```should_panic
    /// # use randoid::Alphabet;
    /// let alph = Alphabet::new([]);
    /// ```
    ///
    /// ```should_panic
    /// # use randoid::Alphabet;
    /// let c = ['0'; (u8::MAX as usize) + 1];
    /// let alph = Alphabet::new(c);
    /// ```
    pub const fn new(chars: [char; N]) -> Self {
        assert!(N != 0, "Alphabet cannot be empty");
        assert!(
            N <= u8::max_value() as usize,
            "The alphabet cannot be longer than a `u8`"
        );
        Alphabet(chars)
    }
}

/// Default alphabet for randoid
///
/// This alphabet that is safe to use in a url and uses 64 characters.
///
/// This is the default value for an alphabet of length 64
pub const DEFAULT: Alphabet = Alphabet([
    '_', '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
]);

/// Standard hexadecimal alphabet with lowercase letters
///
/// This is the default value for an alphabet of length 16
pub const HEX: Alphabet<16> = Alphabet([
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
]);

/// Standard hexadecimal alphabet with upercase letters
pub const HEX_UPPER: Alphabet<16> = Alphabet([
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
]);
