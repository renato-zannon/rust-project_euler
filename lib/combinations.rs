pub struct Combinations<A> {
  values: Vec<A>,
  lastPair: Option<(uint, uint)>
}

pub fn new<A>(values: Vec<A>) -> Combinations<A> {
  Combinations {
    values: values,
    lastPair: None
  }
}

impl<A: Clone> Iterator<(A, A)> for Combinations<A> {
  fn next(&mut self) -> Option<(A, A)> {
    let perm = self.nextPair();
    self.lastPair = perm;

    perm.map(|(nextLeft, nextRight)| {
      let left_value  = self.values.get(nextLeft);
      let right_value = self.values.get(nextRight);

      (left_value.clone(), right_value.clone())
    })
  }
}

impl<A> Combinations<A> {
  fn nextPair(&self) -> Option<(uint, uint)> {
    return match self.lastPair {
      None => Some((0, 0)),

      Some((lastLeft, lastRight)) => {
        let max = self.values.len() - 1;

        if lastRight < max {
          Some((lastLeft, lastRight + 1))
        } else if lastLeft < max {
          Some((lastLeft + 1, 0))
        } else {
          None
        }
      }
    };
  }
}
