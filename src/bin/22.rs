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


use shared::data_reader;
use std::borrow::Borrow;

fn main() {
    let mut names: Vec<String> = get_name_list();
    names.sort();

    let values = names.into_iter().map(alphabetical_value);

    let result = values.enumerate().fold(0, |sum, (index, score)| {
        let index = index as u32;
        sum + ((index + 1) * score)
    });

    println!("{}", result);
}

fn alphabetical_value<S: Borrow<str>>(name: S) -> u32 {
    name.borrow()
        .chars()
        .map(|chr| (chr as u8) - ('A' as u8) + 1)
        .fold(0, |sum, char_value| sum + char_value as u32)
}

fn get_name_list() -> Vec<String> {
    data_reader::for_path("./data/22-names.txt").collect()
}
