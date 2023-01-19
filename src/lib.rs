#![cfg_attr(not(feature = "std"), no_std)]

mod alphabets;
mod std_rand;

pub use alphabets::*;
use rand::Rng;
#[cfg(feature = "std-rand")]
pub use std_rand::*;

use core::fmt::{self, Write};

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::string::String;

/// Size of the buffer to store batched random data in.
///
/// This should be big enough to fit the step size of random data
/// for up to 40 characters in the general case, and 64 if the alphabet has a
/// size that is a power of 2.
const BUFFER_SIZE: usize = 64;

/// Default length of a gnerated id (21)
pub const DEFAULT_SIZE: usize = 21;

#[derive(Clone)]
pub struct Generator<'a, R, const N: usize = 64> {
    alphabet: &'a Alphabet<N>,
    random: R,
    size: usize,
}

impl<'a, R: Rng, const N: usize> Generator<'a, R, N> {
    pub fn new(size: usize, alphabet: &'a Alphabet<N>, random: R) -> Self {
        Self {
            size,
            alphabet,
            random,
        }
    }

    pub fn size(self, size: usize) -> Self {
        Self { size, ..self }
    }

    pub fn alphabet<'b, const M: usize>(self, alphabet: &'b Alphabet<M>) -> Generator<'b, R, M> {
        Generator {
            alphabet,
            size: self.size,
            random: self.random,
        }
    }

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

    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn gen_id(&mut self) -> String {
        let mut res = String::with_capacity(self.size);
        self.write_to(&mut res).unwrap();
        res
    }

    #[cfg(smartstring)]
    pub fn gen_smartstring(&mut self) -> smartstring::String {
        let mut res = smartstring::String::new();
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
    pub fn with_random(random: R) -> Self {
        Self {
            alphabet: &Alphabet::URL,
            random,
            size: DEFAULT_SIZE,
        }
    }
}

#[cfg(feature = "std-rand")]
#[macro_export]
macro_rules! randoid {
    () => {
        $crate::Generator::default().gen_id()
    };
    ($size:expr) => {
        $crate::Generator::with_size($size).gen_id()
    };
    ($size:expr, $alphabet:expr) => {
        $crate::Generator::new($size, &$alphabet, rand::thread_rng()).gen_id()
    };
    ($size:expr, $alphabet:expr, $rand:expr) => {
        $crate::Generator::new($size, &$alphabet, $rand).gen_id()
    };
}
