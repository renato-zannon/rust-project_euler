/* Problem 22: Names scores
 *
 * Using names.txt, a 46K text file containing over five-thousand first names, begin by sorting it
 * into alphabetical order. Then working out the alphabetical value for each name, multiply this
 * value by its alphabetical position in the list to obtain a name score.
 *
 * For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 +
 * 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938  53 = 49714.
 *
 * What is the total of all the name scores in the file? */

#![feature(slicing_syntax)]

extern crate shared;

use shared::data_reader;
use std::iter::AdditiveIterator;

fn main() {
    let names: Vec<String> = get_name_list();

    let mut values: Vec<Vec<uint>> = names.into_iter().map(|name| {
        alphabetical_value(name[])
    }).collect();

    values.sort();

    let result = values.into_iter().enumerate().fold(0, |sum, (index, score)| {
        sum + ((index + 1) * score.into_iter().sum())
    });

    println!("{}", result);
}

fn alphabetical_value(name: &str) -> Vec<uint> {
    name.chars().map(|chr| {
        let result = (chr as u8) - ('A' as u8) + 1;

        result as uint
    }).collect::<Vec<uint>>()
}

fn get_name_list() -> Vec<String> {
    data_reader::for_path("./data/22-names.txt").collect()
}
