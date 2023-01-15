pub trait RandomFiller {
    /// Fill a buffer with random bytes
    fn fill_random(&mut self, buf: &mut [u8]);
}

impl RandomFiller for fn(&mut [u8]) {
    fn fill_random(&mut self, buf: &mut [u8]) {
        self(buf)
    }
}

pub struct RandFn<F: FnMut(&mut [u8])>(pub F);

impl<F: FnMut(&mut [u8])> RandomFiller for RandFn<F> {
    fn fill_random(&mut self, buf: &mut [u8]) {
        self.0(buf)
    }
}

impl<F: FnMut(&mut [u8])> From<F> for RandFn<F> {
    fn from(f: F) -> Self {
        Self(f)
    }
}

#[cfg(feature = "rand")]
mod rand_impl {
    use super::RandomFiller;

    impl<'a, R: rand::Rng> RandomFiller for &'a mut R {
        fn fill_random(&mut self, buf: &mut [u8]) {
            self.fill(buf)
        }
    }

    #[derive(Clone)]
    pub struct Rng<R: rand::Rng>(pub R);

    impl<R: rand::Rng> RandomFiller for Rng<R> {
        fn fill_random(&mut self, buf: &mut [u8]) {
            self.0.fill(buf);
        }
    }

    impl<R: rand::Rng> From<R> for Rng<R> {
        fn from(rng: R) -> Self {
            Rng(rng)
        }
    }
}

#[cfg(feature = "rand")]
pub use rand_impl::*;
