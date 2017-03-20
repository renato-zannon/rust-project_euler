#![crate_name = "shared"]
#![crate_type = "lib"]

#![feature(step_trait, step_by)]

extern crate num;

pub use integer_extensions::IntegerExtensions;
pub use permutations::Permutations;

pub mod combinations;
pub mod triangle;
pub mod sieve;
pub mod digits;
pub mod pandigital;
pub mod primes;
pub mod data_reader;
pub mod integer_extensions;
pub mod continued_fraction;
pub mod permutations;
