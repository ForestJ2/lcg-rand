pub mod rand {
    //! This is NOT secure for any cryptographic uses, and is not designed to be.
    //!
    //! This module is an implementation of the Linear Congruential Generator algorithm.
    //!
    //! See the [Wikipedia page](https://en.wikipedia.org/wiki/Linear_congruential_generator) for more
    //! information.
    use std::time::SystemTime;

    const MODULUS: u128 = 2u128.pow(64);
    const INCREMENT: u64 = 1442695040888963407;
    const MULTIPLIER: u64 = 6364136223846793005;

    pub struct LCG {
        pub seed: u64,
        value: u64,
        modulus: u128,
        increment: u64,
        multiplier: u64,
    }

    impl LCG {
        pub fn new() -> LCG {
            //! Generates new LCG object using default parameters, and seconds since unix epoch as seed.
            //!
            //! **Warning**: will panic if system time is before Jan 1st, 1970 midnight.
            let s = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

            LCG {
                seed: s,
                value: s,
                modulus: MODULUS,
                increment: INCREMENT,
                multiplier: MULTIPLIER,
            }
        }

        pub fn from_seed(seed: u64) -> LCG {
            //! Generate a new LCG object using the given seed, and default mod, inc, and mul.
            LCG {
                seed,
                value: seed,
                modulus: MODULUS,
                increment: INCREMENT,
                multiplier: MULTIPLIER,
            }
        }

        pub fn from_parameters(seed: u64, modulus: u128, increment: u64, multiplier: u64) -> LCG {
            //! Generate an LCG object with all parameters overridden.
            LCG {
                seed,
                value: seed,
                modulus,
                increment,
                multiplier
            }
        }

        pub fn next(&mut self) -> u64 {
            //! Generate the next value and return it.
            self.value = (
                ((self.multiplier as u128) * (self.value as u128) + self.increment as u128) % self.modulus
            ) as u64;

            self.value
        }

        pub fn range(&mut self, start: u64, end: u64) -> u64 {
            //! Restrain the value of [next()](crate::rand::LCG::next) to given values.
            //!
            //! This is inclusive, so both the `start` and `end` parameters could be the result.
            let _ = self.next();

            (self.value % (end - start + 1)) + start
        }

        pub fn choose<'a, T>(&'a mut self, v: &'a [T]) -> &T {
            //! Take a slice and return a random item in it.
            //!
            //! **Warning**: Will panic if given empty slice.
            return &v[
                self.range(0, (v.len()-1) as u64) as usize
            ];
        }
    }
}