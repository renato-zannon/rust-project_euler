extern crate num;
use self::num::Integer;
use std::num::pow;

pub struct Digits<A> {
  remaining: A,
  remaining_digits: uint,
}

impl<A: Integer + FromPrimitive + ToPrimitive> Iterator<A> for Digits<A> {
  fn next(&mut self) -> Option<A> {
    let divisor: A = FromPrimitive::from_uint(self.current_divisor()).unwrap();
    let zero:    A = FromPrimitive::from_uint(0u).unwrap();

    if self.remaining == zero {
      None
    } else {
      let (digit, remainder) = self.remaining.div_rem(&divisor);
      self.remaining = remainder;
      self.remaining_digits -= 1;

      Some(digit)
    }
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

  fn len(&mut self) -> uint {
    let prev_len = self.remaining_digits;

    self.remaining_digits = 0;
    self.remaining = FromPrimitive::from_uint(0u).unwrap();

    prev_len
  }
}

pub fn new<A: ToPrimitive>(number: A) -> Digits<A> {
  Digits {
    remaining_digits: number_of_digits(&number),
    remaining: number,
  }
}

impl<A: ToPrimitive> Digits<A> {
  fn current_divisor(&self) -> uint {
    pow(10u, self.remaining_digits - 1)
  }
}

fn number_of_digits<A: ToPrimitive>(number: &A) -> uint {
  let as_float = number.to_f64().expect("Number not convertible to float!");
  (as_float.log10().floor() as uint) + 1
}
