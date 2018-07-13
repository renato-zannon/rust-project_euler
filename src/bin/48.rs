/* Problem 48: Self powers
 *
 * The series, 1¹ + 2² + 3³ + ... + 10¹⁰ = 10405071317.
 *
 * Find the last ten digits of the series, 1¹ + 2² + 3³ + ... + 1000¹⁰⁰⁰ */

fn main() {
    let ten_to_ten = 10u64.pow(10);
    let take_10_digits = |number: u64| number % ten_to_ten;

    let mut result: u64 = 0;

    for number in 1u64..1001 {
        let mut number_result: u64 = 1;

        for _ in 0u64..number {
            number_result = take_10_digits(number_result) * number;
        }

        result = take_10_digits(number_result) + take_10_digits(result);
    }

    println!("{}", take_10_digits(result));
}
