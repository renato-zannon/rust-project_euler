/* Problem 31: Coin sums
 *
 * In England the currency is made up of pound, £, and pence, p, and there are eight coins in
 * general circulation:
 *
 *     1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
 *
 * It is possible to make £2 in the following way:
 *
 *     1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
 *
 * How many different ways can £2 be made using any number of coins? */

const DENOMINATIONS: &'static [u32] = &[200, 100, 50, 20, 10, 5, 2, 1];

fn main() {
    println!("{}", ways_to_make(200, DENOMINATIONS));
}

fn ways_to_make(value: u32, denominations: &[u32]) -> u32 {
    match denominations.split_first() {
        None => 0,
        Some((_, &[])) => 1,

        Some((denom, remaining_denoms)) => (value % denom..value + 1)
            .step_by(*denom as usize)
            .fold(0, |acc, rest_val| {
                if rest_val == 0 {
                    acc + 1
                } else {
                    acc + ways_to_make(rest_val, remaining_denoms)
                }
            }),
    }
}
