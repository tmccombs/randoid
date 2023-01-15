#![cfg(feature = "std-rand")]
use super::Generator;

use super::DEFAULT_SIZE;
use crate::alphabets::URL;
use crate::randfill::Rng;

impl<'a> Generator<'a, Rng<rand::rngs::ThreadRng>> {
    pub fn with_alphabet(alphabet: &'a [char]) -> Self {
        Self::new(DEFAULT_SIZE, alphabet, default_random())
    }

    pub fn with_size(size: usize) -> Self {
        Self {
            alphabet: &URL,
            random: default_random(),
            size,
        }
    }
}

impl Default for Generator<'static, Rng<rand::rngs::ThreadRng>> {
    fn default() -> Self {
        Self {
            alphabet: &URL,
            random: default_random(),
            size: DEFAULT_SIZE,
        }
    }
}

fn default_random() -> Rng<rand::rngs::ThreadRng> {
    Rng(rand::thread_rng())
}

#[inline]
pub fn randoid() -> String {
    Generator::default().gen_id()
}