use std::collections::hashmap::HashSet;

fn main() {
  let mut powers: HashSet<(uint, uint)> = HashSet::new();

  for a in range(2u, 101) {
    let (base_a, factor) = factor_for(a);

    for b in range(2u, 101) {
      powers.insert((base_a, factor * b));
    }
  }

  println!("{}", powers.len());
}

fn factor_for(num: uint) -> (uint, uint) {
  let fnum:       f64 = num as f64;

  let mut factor: f64 = 1.0;
  let mut base:   f64 = fnum;

  for potential_root in range(2f64, fnum) {
    let exp = fnum.log(potential_root);

    if exp.fract() == 0f64 {
      factor = exp;
      base   = potential_root;
      break;
    }
  }

  (base as uint, factor as uint)
}
