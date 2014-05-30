/* Problem 39: Integer right triangles
 *
 * If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are
 * exactly three solutions for p = 120.
 *
 * {20,48,52}, {24,45,51}, {30,40,50}
 *
 * For which value of p ≤ 1000, is the number of solutions maximised? */

#[cfg(not(test))]
fn main() {
  let result = range(1u, 1_000)
    .max_by(|&perimeter| solution_count(perimeter))
    .unwrap();

  println!("{}", result);
}

fn solution_count(perimeter: uint) -> uint {
  let mut count = 0;
  let perimeter_by_3 = perimeter / 3;

  for a in range(1, perimeter_by_3) {
    let a_squared = a * a;

    for b in range(a, perimeter_by_3 + a) {
      let c = perimeter - (a + b);

      let b_squared = b * b;
      let c_squared = c * c;

      if a_squared + b_squared == c_squared {
        count += 1;
      }
    }
  }

  return count;
}

#[cfg(test)]
mod tests {
  use super::solution_count;

  #[test]
  fn test_example_solution_count() {
    assert_eq!(solution_count(120), 3);
  }
}
