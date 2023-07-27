pub mod rand {
    //! This is NOT secure for any cryptographic uses, and is not designed to be.
    //!
    //! This module is an implementation of the Linear Congruential Generator algorithm.
    //!
    //! See the [Wikipedia page](https://en.wikipedia.org/wiki/Linear_congruential_generator) for more
    //! information.
    use std::time::SystemTime;

    use rand_core::{RngCore, SeedableRng};

    const MODULUS: u128 = 2u128.pow(64);
    const INCREMENT: u64 = 1442695040888963407;
    const MULTIPLIER: u64 = 6364136223846793005;
    const DEFAULT_SEED: u64 = 524764;

    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct LCG {
        seed: u64,
        state: u64,
        modulus: u128,
        increment: u64,
        multiplier: u64,
    }

    impl LCG {
        pub fn new(seed: u64) -> LCG {
            //! Generate a new LCG object using the given seed, and default mod, inc, and mul.
            LCG {
                seed,
                state: seed,
                modulus: MODULUS,
                increment: INCREMENT,
                multiplier: MULTIPLIER,
            }
        }

        pub fn new_specific(seed: u64, modulus: u128, increment: u64, multiplier: u64) -> LCG {
            //! Generate an LCG object with all parameters overridden.
            LCG {
                seed,
                state: seed,
                modulus,
                increment,
                multiplier,
            }
        }

        pub fn range(&mut self, start: u64, end: u64) -> u64 {
            //! Restrain the value of [next()](crate::rand::LCG::next) to given values.
            //!
            //! This is inclusive, so both the `start` and `end` parameters could be the result.
            let _ = self.next_u64();

            (self.state % (end - start + 1)) + start
        }

        pub fn choose<'a, T>(&'a mut self, v: &'a [T]) -> &T {
            //! Take a slice and return a random item in it.
            //!
            //! **Warning**: Will panic if given empty slice.
            return &v[self.range(0, (v.len() - 1) as u64) as usize];
        }

        pub fn set_seed(&mut self, seed: u64) {
            self.seed = seed;
        }
    }

    impl Default for LCG {
        fn default() -> Self {
            let seed = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|it| it.as_secs())
                .unwrap_or(DEFAULT_SEED);

            LCG::new(seed)
        }
    }

    impl RngCore for LCG {
        fn next_u32(&mut self) -> u32 {
            self.next_u64() as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.state = (((self.multiplier as u128) * (self.state as u128)
                + self.increment as u128)
                % self.modulus) as u64;

            self.state
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            let count = (dest.len() / 8) + (if dest.len() % 8 != 0 { 1 } else { 0 });
            let source = Vec::from_iter((0..count).map(|_| self.next_u64()));
            rand_core::impls::fill_via_u64_chunks(&source, dest);
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    impl SeedableRng for LCG {
        type Seed = [u8; 8];

        fn from_seed(seed: Self::Seed) -> Self {
            let mut seed_u64 = [0u64; 1];
            rand_core::le::read_u64_into(&seed, &mut seed_u64);
            LCG::new(u64::from(seed_u64[0]))
        }
    }
}
