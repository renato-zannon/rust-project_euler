/* Problem 79: Passcode derivation
 *
 * A common security method used for online banking is to ask the user for three random characters
 * from a passcode. For example, if the passcode was 531278, they may ask for the 2nd, 3rd, and 5th
 * characters; the expected reply would be: 317.
 *
 * The text file, keylog.txt, contains fifty successful login attempts.
 *
 * Given that the three characters are always asked for in order, analyse the file so as to
 * determine the shortest possible secret passcode of unknown length.
 **/

use std::collections::HashSet;

const KEYLOG: &'static str = include_str!("../../data/79-keylog.txt");

fn main() {
    let seen: HashSet<Vec<u32>> = KEYLOG
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    let different_digits = seen
        .iter()
        .flat_map(|entry| entry.iter())
        .collect::<HashSet<_>>()
        .len();

    let hyp = hypothesis(different_digits as u32)
        .find(|hyp| seen.iter().all(|followers| fits_followers(hyp, followers)))
        .unwrap();

    for num in hyp {
        print!("{}", num);
    }
    println!("");
}

fn hypothesis(min_size: u32) -> impl Iterator<Item = Vec<u32>> {
    let mut current = vec![0; min_size as usize];

    std::iter::from_fn(move || {
        let maxed_count = current.iter().rev().take_while(|i| **i == 9).count();

        let first_to_turn = current.len() - maxed_count;
        for num in &mut current[first_to_turn..] {
            *num = 0;
        }

        if first_to_turn >= 1 {
            current[first_to_turn - 1] += 1;
        } else {
            current.push(0);
        }

        Some(current.clone())
    })
}

fn fits_followers(hyp: &[u32], followers: &[u32]) -> bool {
    let mut remaining = &hyp[..];

    for &follower in followers {
        loop {
            match remaining.split_first() {
                Some((head, tail)) => {
                    remaining = tail;

                    if *head == follower {
                        break;
                    }
                }

                None => return false,
            }
        }
    }

    true
}
