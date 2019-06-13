/* Problem 9: Special Pythagorean triplet
 *
 * A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
 * a^2 + b^2 = c^2
 *
 * For example, 3^2 + 4^2 = 9 + 16 = 25 = 52.
 *
 * There exists exactly one Pythagorean triplet for which a + b + c = 1000.
 *
 * Find the product abc. */

use shared::pythagorean_triplets;

fn main() {
    let (result, (a, b, c)) = pythagorean_triplets::new()
        .find(|&(a, b, c)| a + b + c == 1000)
        .map(|(a, b, c)| (a * b * c, (a, b, c)))
        .unwrap();

    println!("{} ({}, {}, {})", result, a, b, c);
}
