extern crate num;
use self::num::Integer;

pub struct Digits<A> {
  remaining: A,
}

impl<A: Integer + FromPrimitive + ToPrimitive> Iterator<A> for Digits<A> {
  fn next(&mut self) -> Option<A> {
    let ten:  A = FromPrimitive::from_uint(10u).unwrap();
    let zero: A = FromPrimitive::from_uint(0u).unwrap();

    if self.remaining == zero {
      None
    } else {
      let (remainder, digit) = self.remaining.div_rem(&ten);
      self.remaining = remainder;

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
}

pub fn new<A>(number: A) -> Digits<A> {
  Digits { remaining: number }
}
