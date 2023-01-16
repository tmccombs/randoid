#![cfg(feature = "std-rand")]
use super::Generator;

use super::DEFAULT_SIZE;
use crate::alphabets::URL;

use rand::{thread_rng, rngs::ThreadRng};

impl<'a> Generator<'a, ThreadRng> {
    pub fn with_alphabet(alphabet: &'a [char]) -> Self {
        Self::new(DEFAULT_SIZE, alphabet, thread_rng())
    }

    pub fn with_size(size: usize) -> Self {
        Self {
            alphabet: &URL,
            random: thread_rng(),
            size,
        }
    }
}

impl Default for Generator<'static, rand::rngs::ThreadRng> {
    fn default() -> Self {
        Self {
            alphabet: &URL,
            random: thread_rng(),
            size: DEFAULT_SIZE,
        }
    }
}

#[inline]
pub fn randoid() -> String {
    Generator::default().gen_id()
}
