/* Problem 76: Counting summations
 *
 * It is possible to write five as a sum in exactly six different ways:
 *
 * 4 + 1
 * 3 + 2
 * 3 + 1 + 1
 * 2 + 2 + 1
 * 2 + 1 + 1 + 1
 * 1 + 1 + 1 + 1 + 1
 *
 * How many different ways can one hundred be written as a sum of at least two positive integers?
 **/

// dynamic programming solution based on solution 31-2
fn main() {
    println!("{}", ways_to_make(100));
}

fn ways_to_make(n: i32) -> i32 {
    let n = n as usize;

    let mut ways = vec![0; n + 1];
    ways[0] = 1;

    // when part = X, it means we're evaluating
    // in which X can be a part of the other numbers
    for part in 1..n {
        // target is each number which `part` can be part of
        for target in part..=n {
            ways[target] += ways[target - part];
        }
    }

    ways[n]
}
