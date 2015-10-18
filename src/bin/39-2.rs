/* Problem 39: Integer right triangles
 *
 * If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are
 * exactly three solutions for p = 120.
 *
 * {20,48,52}, {24,45,51}, {30,40,50}
 *
 * For which value of p ≤ 1000, is the number of solutions maximised? */

const MAX_PERIMETER: f64 = 1_000.0;

// Alternative implementation for problem 39. Much faster
fn main() {
    let mut counts = [0usize; MAX_PERIMETER as usize];

    let max_perimeter_by_3 = MAX_PERIMETER / 3.0;
    let two_thirds_of_max  = 2.0 * max_perimeter_by_3;

    let mut a = 1.0;

    while a < max_perimeter_by_3 {
        let a_squared = a * a;

        let mut b = a + 1.0;
        while b < two_thirds_of_max - a {
            let b_squared = b * b;

            let c = (a_squared + b_squared).sqrt();
            let is_integer = c == c.floor();

            if !is_integer {
                b += 1.0;
                continue;
            }

            let perimeter = a + b + c;

            if perimeter < MAX_PERIMETER {
                counts[(perimeter as usize) - 1] += 1;
            }

            b += 1.0;
        }

        a += 1.0;
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
