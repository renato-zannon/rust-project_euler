extern crate num;
use self::num::Integer;
use std::num::{pow, ToStrRadix};

pub struct Digits<A> {
  remaining: A,
  remaining_digits: uint,
}

impl<A: Integer + FromPrimitive + ToPrimitive> Iterator<A> for Digits<A> {
  fn next(&mut self) -> Option<A> {
    let divisor: A = FromPrimitive::from_uint(self.current_divisor()).unwrap();

    self.consume_remaining(divisor).map(|(digit, remainder)| {
      self.remaining = remainder;
      self.remaining_digits -= 1;

      digit
    })
  }

  fn size_hint(&self) -> (uint, Option<uint>) {
    match self.remaining.to_f64() {
      Some(as_float) => {
        let log = as_float.log10();
        (log.floor() as uint, Some(log.ceil() as uint))
      },

      None => (0, None)
    }
  }

  fn count(&mut self) -> uint {
    let prev_count = self.remaining_digits;

    self.remaining_digits = 0;
    self.remaining = FromPrimitive::from_uint(0u).unwrap();

    prev_count
  }
}

impl<A: Integer + FromPrimitive + ToPrimitive> DoubleEndedIterator<A> for Digits<A> {
  fn next_back(&mut self) -> Option<A> {
    let ten: A = FromPrimitive::from_uint(10u).unwrap();

    self.consume_remaining(ten).map(|(remainder, digit)| {
      self.remaining = remainder;
      self.remaining_digits -= 1;

      digit
    })
  }
}

pub fn new<A: ToPrimitive + ToStrRadix>(number: A) -> Digits<A> {
  Digits {
    remaining_digits: number_of_digits(&number),
    remaining: number,
  }
}

impl<A: Integer + FromPrimitive + ToPrimitive> Digits<A> {
  fn consume_remaining(&mut self, divisor: A) -> Option<(A, A)> {
    if self.remaining_digits == 0 {
      None
    } else {
      Some(self.remaining.div_rem(&divisor))
    }
  }

  fn current_divisor(&self) -> uint {
    pow(10u, self.remaining_digits - 1)
  }
}

fn number_of_digits<A: ToPrimitive + ToStrRadix>(number: &A) -> uint {
  match number.to_f64() {
    Some(as_float) => (as_float.log10().floor() as uint) + 1,
    None           => number.to_str_radix(10).len()
  }
}

#[cfg(test)]
mod tests {
  use super::new;

  #[test]
  fn test_digits_in_order() {
    let digits = new(12345u).collect::<Vec<uint>>();
    assert_eq!(digits.as_slice(), &[1, 2, 3, 4, 5]);
  }

  #[test]
  fn test_digits_in_reverse() {
    let digits = new(12345u).rev().collect::<Vec<uint>>();
    assert_eq!(digits.as_slice(), &[5, 4, 3, 2, 1]);
  }

  #[test]
  fn test_digits_in_order_with_zero() {
    let digits = new(123450u).collect::<Vec<uint>>();
    assert_eq!(digits.as_slice(), &[1, 2, 3, 4, 5, 0]);
  }

  #[test]
  fn test_digits_in_reverse_with_zero() {
    let digits = new(123450u).rev().collect::<Vec<uint>>();
    assert_eq!(digits.as_slice(), &[0, 5, 4, 3, 2, 1]);
  }
}
