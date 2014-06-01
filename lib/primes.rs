use std::iter::range_step_inclusive;

// Adapted from the problem 07 overview PDF
pub fn is_prime(num: uint) -> bool {
  if num == 1           { return false }
  else if num < 4       { return true  }
  else if num % 2 == 0  { return false }
  else if num < 9       { return true  }
  else if num % 3 == 0  { return false }

  let r = num.to_f64()
    .map(|as_float| as_float.sqrt())
    .and_then(|result| result.ceil().to_uint())
    .unwrap();

  range_step_inclusive(5, r, 6).all(|f| {
    num % f != 0 && num % (f + 2) != 0
  })
}
