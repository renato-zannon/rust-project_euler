/* Problem 39: Integer right triangles
 *
 * If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are
 * exactly three solutions for p = 120.
 *
 * {20,48,52}, {24,45,51}, {30,40,50}
 *
 * For which value of p ≤ 1000, is the number of solutions maximised? */

static MAX_PERIMETER: f64 = 1_000.0;

// Alternative implementation for problem 39. Much faster
fn main() {
  let mut counts = [0u, ..(MAX_PERIMETER as uint)];

  let max_perimeter_by_3 = MAX_PERIMETER / 3.0;
  let two_thirds_of_max  = 2.0 * max_perimeter_by_3;

  for a in range(1.0, max_perimeter_by_3) {
    let a_squared = a * a;

    for b in range(a + 1.0, two_thirds_of_max - a) {
      let b_squared = b * b;

      let c = (a_squared + b_squared).sqrt();
      let is_integer = c == c.floor();

      if !is_integer { continue; }

      let perimeter = a + b + c;

      if perimeter < MAX_PERIMETER {
        counts[(perimeter as uint) - 1] += 1;
      }
    }
  }

  let mut max_count = 0;
  let mut max = 0;

  for (index, &count) in counts.iter().enumerate() {
    if count > max_count {
      max_count = count;
      max = index + 1;
    }
  }

  println!("{}", max);
}
