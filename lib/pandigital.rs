pub enum PandigitalResult {
  IsPandigital,
  TooSmall,
  TooLarge,
  HasRepetitions,
}

impl PandigitalResult {
  pub fn to_bool(&self) -> bool {
    match *self {
      IsPandigital => true,
      _            => false,
    }
  }
}

pub fn is_9_pandigital(digits: &[uint]) -> PandigitalResult {
  match digits.len().cmp(&9) {
    Less    => return TooSmall,
    Greater => return TooLarge,
    Equal   => (),
  }

  let mut found_numbers = Vec::from_elem(9, false);

  let only_uniques = digits.iter().all(|digit| {
    let found = match *digit {
      0    => return false,
      1..9 => found_numbers.get_mut(digit - 1),
      _    => unreachable!(),
    };

    if *found {
      return false;
    } else {
      *found = true;
      return true;
    }
  });

  if only_uniques {
    IsPandigital
  } else {
    HasRepetitions
  }
}
