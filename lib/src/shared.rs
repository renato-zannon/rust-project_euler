#![crate_name = "shared"]
#![crate_type = "lib"]

#![feature(zero_one, step_trait, step_by, core)]

extern crate num;

pub use integer_extensions::IntegerExtensions;

pub mod combinations;
pub mod triangle;
pub mod sieve;
pub mod digits;
pub mod pandigital;
pub mod primes;
pub mod data_reader;
pub mod integer_extensions;
pub mod continued_fraction;
