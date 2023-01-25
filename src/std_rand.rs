#![cfg(feature = "std-rand")]
use super::Generator;

use super::DEFAULT_SIZE;
use crate::alphabet::{Alphabet, DEFAULT};
use rand::{rngs::ThreadRng, thread_rng};

impl<'a, const N: usize> Generator<'a, ThreadRng, N> {
    /// Create a new randoid generator using a specific alphabet
    ///
    /// And using the default size and [`rand::thread_rng()`] as the RNG.
    pub fn with_alphabet(alphabet: &'a Alphabet<N>) -> Self {
        Self::new(DEFAULT_SIZE, alphabet, thread_rng())
    }
}

impl<'a> Generator<'a, ThreadRng> {
    /// Create a new randoid generator that generates ids of a specific size
    ///
    /// But use the default alphabet and [`rand::thread_rng()`] as the RNG.
    pub fn with_size(size: usize) -> Self {
        Self {
            alphabet: &DEFAULT,
            random: thread_rng(),
            size,
        }
    }
}

impl Default for Generator<'static, rand::rngs::ThreadRng> {
    fn default() -> Self {
        Self {
            alphabet: &DEFAULT,
            random: thread_rng(),
            size: DEFAULT_SIZE,
        }
    }
}

/// Simple API for generating a nano id
///
/// This creates a `String` containing a randomly generated id using the default size (21),
/// alphabet (url safe with 64 characters), and Rng (`rand::thread_rng()`).
///
/// # Example:
///
/// ```
/// use randoid::randoid;
///
/// let id = randoid();
///
/// assert_eq!(id.len(), 21);
/// for c in id.chars() {
///     assert!(c.is_alphanumeric() || c == '-' || c == '_');
/// }
/// ```
#[inline]
pub fn randoid() -> String {
    Generator::default().gen_id()
}
