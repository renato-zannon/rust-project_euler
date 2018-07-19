extern crate shared;

use shared::totient;

fn main() {
    let result: u64 = totient::up_to(1_000_000).into_iter().sum();
    println!("{}", result);
}
