use num::{BigUint, ToPrimitive};

use std::io::{self, Write};

pub trait DigitCount {
    fn number_of_digits(&self) -> u32;
}

macro_rules! float_impl(
    ($ty:ident) => (
        impl DigitCount for $ty {
            fn number_of_digits(&self) -> u32 {
                self.log10().floor().to_u32().unwrap_or(0) + 1
            }
        }
    )
);

macro_rules! int_impl(
    ($ty:ident) => (
        impl DigitCount for $ty {
            fn number_of_digits(&self) -> u32 {
                (*self as f64).number_of_digits()
            }
        }
    )
);

float_impl!(f32);
float_impl!(f64);

int_impl!(u8);
int_impl!(u16);
int_impl!(u32);
int_impl!(u64);
int_impl!(u128);
int_impl!(usize);
int_impl!(i8);
int_impl!(i16);
int_impl!(i32);
int_impl!(i64);
int_impl!(i128);
int_impl!(isize);

impl DigitCount for BigUint {
    fn number_of_digits(&self) -> u32 {
        let mut counter = DigitCounter { count: 0 };
        (write!(&mut counter, "{:?}", self)).unwrap();

        counter.count
    }
}

struct DigitCounter {
    count: u32,
}

impl Write for DigitCounter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.count += buf.len() as u32;
        return Ok(buf.len());
    }

    fn flush(&mut self) -> io::Result<()> {
        return Ok(());
    }
}
