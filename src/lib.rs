#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

use core::cell::RefCell;
use core::fmt::{self, Write};

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::string::String;

mod alphabets;
mod std_rand;

pub use alphabets::*;
use rand::Rng;
#[cfg(feature = "std-rand")]
pub use std_rand::*;

/// Size of the buffer to store batched random data in.
///
/// This should be big enough to fit the step size of random data
/// for up to 40 characters in the general case, and 64 if the alphabet has a
/// size that is a power of 2.
const BUFFER_SIZE: usize = 64;

/// Default length of a generated id (21)
pub const DEFAULT_SIZE: usize = 21;

///
#[derive(Clone)]
pub struct Generator<'a, R, const N: usize = 64> {
    alphabet: &'a Alphabet<N>,
    random: R,
    size: usize,
}

impl<'a, R: Rng, const N: usize> Generator<'a, R, N> {
    /// Create a new, fully specified id generator
    ///
    /// Create a new generator that genartes ids composed of `size` characters chosen at random
    /// from `alphabet`, using `random` as a source of random data.
    ///
    /// # Examples
    ///
    /// ```
    /// use randoid::{Generator, Alphabet};
    /// # use rand::SeedableRng;
    ///
    /// let rand = rand_xoshiro::Xoshiro256PlusPlus::seed_from_u64(0x04040404);
    /// let mut gen = Generator::new(8, &Alphabet::HEX, rand);
    /// assert_eq!(gen.gen_id(), "905c2761");
    /// assert_eq!(gen.gen_id(), "304ec655");
    /// ```
    pub fn new(size: usize, alphabet: &'a Alphabet<N>, random: R) -> Self {
        Self {
            size,
            alphabet,
            random,
        }
    }

    /// Update the size of an existing generator
    ///
    /// # Example
    ///
    /// ```
    /// # use randoid::Generator;
    ///
    /// let id = Generator::default().size(32).gen_id();
    /// assert_eq!(id.len(), 32);
    /// ```
    pub fn size(self, size: usize) -> Self {
        Self { size, ..self }
    }

    /// Update the alphabet of an existing generator
    ///
    /// # Example
    ///
    /// ```
    /// # use randoid::{Generator, Alphabet};
    ///
    /// let id = Generator::default().alphabet(&Alphabet::new(['a', 'b', 'c', 'd', 'e'])).gen_id();
    /// assert!(id.chars().all(|c| matches!(c, 'a'..='e')));
    /// ```
    pub fn alphabet<'b, const M: usize>(self, alphabet: &'b Alphabet<M>) -> Generator<'b, R, M> {
        Generator {
            alphabet,
            size: self.size,
            random: self.random,
        }
    }

    /// Generate a new id, and write the result to `out`
    ///
    /// This allows you to avoid creating a new string if you would simply
    /// be adding that string to something else.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ids = String::new();
    ///
    /// let mut id_gen = randoid::Generator::default();
    /// id_gen.write_to(&mut ids);
    /// ids.push('\n');
    /// id_gen.write_to(&mut ids);
    ///
    /// assert_eq!(ids.len(), 21 * 2 + 1);
    /// ```
    ///
    /// # See Also
    /// - [`Generator::fmt`]
    /// - [`Generator::gen_id`]
    /// - [`Generator::gen_smartstring`]
    /// - [`Generator::fmt`]
    pub fn write_to<W: Write>(&mut self, out: &mut W) -> fmt::Result {
        if self.size == 0 {
            return Ok(());
        }
        if N.is_power_of_two() {
            self.fast_impl(out)
        } else {
            self.generic_impl(out)
        }
    }

    /// Return an object which implements [`std::fmt::Display`]
    ///
    /// This allows you to pass a new generated id to `write!()`, `format!()`, etc.
    /// without having to create an intermediate string.
    ///
    /// # Warning
    ///
    /// The returned object will generate a unique id, each time it's `Display`
    /// implementation is used. It uses interior mutability in order to avoid
    /// having to store the actual id. Similarly, the random data isn't actually
    /// generated until it is written somewhere. In general I would advise against
    /// using it except as a temporary to a formatting macro.
    ///
    /// # Examples
    ///
    /// ```
    /// use randoid::Generator;
    /// # use rand::SeedableRng;
    ///
    /// let mut generator = Generator::with_random(rand_xoshiro::Xoshiro256PlusPlus::seed_from_u64(1));
    ///
    /// println!("Your new id is: {}", generator.fmt());
    ///
    /// assert_eq!(format!("uid-{}", generator.fmt()), "uid-kkb3tf6ZyJm49m5J3xuB8");
    /// let f = generator.fmt();
    ///
    /// assert_eq!(f.to_string(), "5jO6j5xWvMx17zY3e9NbN");
    /// assert_eq!(f.to_string(), "kGAK7hvw7AdqTcsFNZGtr");
    ///
    /// ```
    pub fn fmt(&mut self) -> Fmt<'_, 'a, R, N> {
        Fmt(RefCell::new(self))
    }

    /// Generate a random id as a string
    ///
    /// # Examples
    ///
    /// ```
    /// let random_id = randoid::Generator::default().gen_id();
    /// ```
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn gen_id(&mut self) -> String {
        let mut res = String::with_capacity(self.size);
        self.write_to(&mut res).unwrap();
        res
    }

    /// Generate a random id as a smartstring
    ///
    /// # Examples
    ///
    /// ```
    /// use randoid::Generator;
    /// use smartstring::alias::String;
    ///
    /// let random_id: String = Generator::default().gen_smartstring();
    /// ```
    #[cfg(feature = "smartstring")]
    pub fn gen_smartstring(&mut self) -> smartstring::alias::String {
        let mut res = smartstring::alias::String::new();
        self.write_to(&mut res).unwrap();
        res
    }

    fn fast_impl<W: Write>(&mut self, out: &mut W) -> fmt::Result {
        assert!(N.is_power_of_two());
        let mask: usize = N - 1;
        debug_assert!(mask.count_ones() == mask.trailing_ones());
        let mut buffer = [0u8; BUFFER_SIZE];
        let mut rem = self.size;
        while rem > 0 {
            let bytes = &mut buffer[..self.size.min(BUFFER_SIZE)];
            self.random.fill(bytes);
            for &b in &*bytes {
                let idx = b as usize & mask;
                debug_assert!(idx < N);
                // Safety: Since the alphabet size is a power of 2, applying the
                // mask ensures that idx is a valid index into the alphabet
                // And we assert that it is a power of 2 on the first line.
                out.write_char(self.alphabet.0[idx])?;
            }
            rem -= bytes.len();
        }
        Ok(())
    }

    fn generic_impl<W: Write>(&mut self, out: &mut W) -> fmt::Result {
        let mask = N.next_power_of_two() - 1;
        let mut buffer = [0u8; BUFFER_SIZE];
        let step: usize = BUFFER_SIZE.min(8 * self.size / 5);
        // We don't use the full buffer, because that might require generating
        // more random data than we need.
        let bytes = &mut buffer[..step];

        // Assert that the masking does not truncate the alphabet.
        debug_assert!(N <= mask + 1);
        let mut i = 0;

        while i < self.size {
            self.random.fill(bytes);

            for &byte in &*bytes {
                let byte = byte as usize & mask;

                if let Some(&c) = self.alphabet.0.get(byte) {
                    out.write_char(c)?;
                    i += 1;
                }
            }
        }
        Ok(())
    }
}

impl<'a, R: Rng> Generator<'a, R> {
    /// Create a new randoid generator from an Rng
    ///
    /// Using the default size and alphabet
    pub fn with_random(random: R) -> Self {
        Self {
            alphabet: &Alphabet::URL,
            random,
            size: DEFAULT_SIZE,
        }
    }
}

/// See [`Generator::fmt`]
pub struct Fmt<'g, 'a: 'g, R: Rng, const N: usize>(RefCell<&'g mut Generator<'a, R, N>>);

impl<'g, 'a: 'g, R: Rng, const N: usize> fmt::Display for Fmt<'g, 'a, R, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.borrow_mut().write_to(f)
    }
}

/// Convenience macro for emulating default arguments
///
/// This macro takes zero to three arguments, and generates a random id as a string.
///
/// This simulates a function with default arguments.
///
/// The first argument is the length (in characters) of the generated id. Defaults to
/// [`DEFAULT_SIZE`].
///
/// The second argument is the alphabet to use. This macro will automatically add borrow the
/// alphabet, if an owned value is passed. Defaults to [`Alphabet::URL`].
///
/// The third argument is the random number generator to use. Defaults to [`rand::thread_rng()`].
///
///
/// # Examples
///
/// ```
/// use randoid::randoid;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// // Use all defaults
/// let id = randoid!();
/// assert_eq!(id.len(), 21);
/// assert!(id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'));
/// // Generate id with 32 characters, but default alphabet and rng
/// let id = randoid!(32);
/// assert_eq!(id.len(), 32);
/// assert!(id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'));
/// // Generate id with 32 hex characters, but default rng
/// let id = randoid!(32, randoid::HexAlphabet::HEX);
/// assert_eq!(id.len(), 32);
/// assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
/// // Generate id with 32 hex characters, using SmallRng for the RNG
/// let id = randoid!(32, randoid::HexAlphabet::HEX, StdRng::from_entropy());
/// assert_eq!(id.len(), 32);
/// assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
///
///
/// ```
#[cfg(feature = "std-rand")]
#[macro_export]
macro_rules! randoid {
    () => {
        $crate::randoid()
    };
    ($size:expr) => {
        $crate::Generator::with_size($size).gen_id()
    };
    ($size:expr, $alphabet:expr) => {
        $crate::Generator::new($size, &$alphabet, rand::thread_rng()).gen_id()
    };
    ($size:expr, $alphabet:expr, $rand:expr) => {
        $crate::Generator::new($size, ::std::borrow::Borrow::borrow(&$alphabet), $rand).gen_id()
    };
}
