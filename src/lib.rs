#![cfg_attr(not(feature = "std"), no_std)]

pub mod alphabets;
mod randfill;
mod std_rand;

pub use alphabets::URL;
pub use randfill::*;
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

/// Default length of a gnerated id
pub const DEFAULT_SIZE: usize = 21;

pub struct Generator<'a, R> {
    alphabet: &'a [char],
    random: R,
    size: usize,
}

impl<'a, R: RandomFiller> Generator<'a, R> {
    pub fn new(size: usize, alphabet: &'a [char], random: R) -> Self {
        assert!(
            alphabet.len() <= u8::max_value() as usize,
            "The alphabet cannot be longer than a `u8`"
        );
        Self {
            size,
            alphabet,
            random,
        }
    }

    pub fn with_random(random: R) -> Self {
        Self {
            alphabet: &URL,
            random,
            size: DEFAULT_SIZE,
        }
    }

    pub fn size(self, size: usize) -> Self {
        Self { size, ..self }
    }

    pub fn alphabet(self, alphabet: &'a [char]) -> Self {
        Self { alphabet, ..self }
    }

    pub fn write_to<W: Write>(&mut self, out: &mut W) -> fmt::Result {
        if self.size == 0 {
            return Ok(());
        }
        if self.size.is_power_of_two() {
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
        assert!(self.size.is_power_of_two());
        let mask = self.alphabet.len() - 1;
        debug_assert!(mask.count_ones() == mask.trailing_ones());
        let mut buffer = [0u8; BUFFER_SIZE];
        let mut rem = self.size;
        while rem > 0 {
            let bytes = &mut buffer[..self.size.min(BUFFER_SIZE)];
            self.random.fill_random(bytes);
            for &b in &*bytes {
                let idx = b as usize & mask;
                debug_assert!(idx < self.alphabet.len());
                // Safety: Since the alphabet size is a power of 2, applying the
                // mask ensures that idx is a valid index into the alphabet
                // And we assert that it is a power of 2 on the first line.
                out.write_char(*unsafe { self.alphabet.get_unchecked(idx) })?;
            }
            rem -= bytes.len();
        }
        Ok(())
    }

    fn generic_impl<W: Write>(&mut self, out: &mut W) -> fmt::Result {
        let mask = self.alphabet.len().next_power_of_two() - 1;
        let mut buffer = [0u8; BUFFER_SIZE];
        let step: usize = BUFFER_SIZE.min(8 * self.size / 5);
        // We don't use the full buffer, because that might require generating
        // more random data than we need.
        let bytes = &mut buffer[..step];

        // Assert that the masking does not truncate the alphabet.
        debug_assert!(self.alphabet.len() <= mask + 1);
        let mut i = 0;

        while i < self.size {
            self.random.fill_random(bytes);

            for &byte in &*bytes {
                let byte = byte as usize & mask;

                if self.alphabet.len() > byte {
                    out.write_char(self.alphabet[byte])?;
                    i += 1;
                }
            }
        }
        Ok(())
    }
}
