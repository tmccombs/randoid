/// Type for an alphabet to use for generating ids
///
/// It has a fixed length, because that can provide the compiler
/// with more optimization opportunities, and in almost all cases
/// the alphabet used will be a constant anyway.
pub struct Alphabet<const N: usize = 64>(pub [char; N]);

pub type HexAlphabet = Alphabet<16>;

impl Default for Alphabet {
    fn default() -> Self {
        Self::URL
    }
}

impl Default for HexAlphabet {
    fn default() -> Self {
        Self::HEX
    }
}

impl Alphabet {
    /// Alphabet that is safe to use in a url
    ///
    /// Uses 64 characters.
    ///
    /// This is the default value for an alphabet of length 64
    pub const URL: Alphabet = Alphabet([
        '_', '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
        'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
        'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ]);
}

impl HexAlphabet {
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
}
