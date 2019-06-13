/* The nth term of the sequence of triangle numbers is given by, tₙ = ½n(n+1); so the first ten
 * triangle numbers are:
 *
 * 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
 *
 * By converting each letter in a word to a number corresponding to its alphabetical position and
 * adding these values we form a word value. For example, the word value for SKY is 19 + 11 + 25 =
 * 55 = t10. If the word value is a triangle number then we shall call the word a triangle word.
 *
 * Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly
 * two-thousand common English words, how many are triangle words? */

use num::ToPrimitive;
use shared::data_reader;

fn main() {
    let reader = data_reader::for_path("./data/42-words.txt");

    let result = reader.filter(|word| is_triangular_word(&word)).count();

    println!("{}", result);
}

fn is_triangular_word(word: &str) -> bool {
    let as_number = word.chars().fold(0, |total, chr| {
        let num_chr = (chr as u32) - ('A' as u32) + 1;
        total + num_chr
    });

    triangular_index(as_number).is_some()
}

// Deduced by solving x = n(n + 1)/2
fn triangular_index(number: u32) -> Option<u32> {
    if number == 1 {
        return Some(1);
    }

    let delta = 1 + 8 * number;

    return take_sqrt(delta)
        .and_then(ensure_integer)
        .and_then(ensure_divisible);

    fn take_sqrt(delta: u32) -> Option<f64> {
        delta.to_f64().map(|as_float| as_float.sqrt())
    }

    fn ensure_integer(delta_sqrt: f64) -> Option<u32> {
        let is_integer = delta_sqrt == delta_sqrt.floor();

        if is_integer {
            delta_sqrt.to_u32()
        } else {
            None
        }
    }

    fn ensure_divisible(integer_sqrt: u32) -> Option<u32> {
        let remainder = (integer_sqrt - 1) % 2;

        if remainder == 0 {
            let divided = (integer_sqrt - 1) / 2;
            Some(divided)
        } else {
            None
        }
    }
}

#[test]
fn test_triangular_index() {
    let examples = [1, 3, 6, 10, 15, 21, 28, 36, 45, 55];

    for (index, &example) in examples.iter().enumerate() {
        assert_eq!(triangular_index(example), Some(index as u32 + 1));
    }
}

#[test]
fn test_triangular_word() {
    assert!(is_triangular_word("SKY"));
}
