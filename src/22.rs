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

use std::iter::AdditiveIterator;
use std::io::{File, BufferedReader};
use std::str;

fn main() {
  let names: Vec<~str> = get_name_list();

  let mut values: Vec<Vec<uint>> = names.move_iter().map(|name| {
    alphabetical_value(name.as_slice())
  }).collect();

  values.sort();

  let result = values.move_iter().enumerate().fold(0, |sum, (index, score)| {
    sum + ((index + 1) * score.move_iter().sum())
  });

  println!("{}", result);
}

fn alphabetical_value(name: &str) -> Vec<uint> {
  name.chars().map(|chr| {
    let result = (chr as u8) - ('A' as u8) + 1;

    result as uint
  }).collect::<Vec<uint>>()
}

fn get_name_list() -> Vec<~str> {
  let path = &Path::new("./data/22-names.txt");
  let mut file = BufferedReader::new(File::open(path));

  let mut result: Vec<~str> = Vec::new();

  loop {
    let maybe_name = file.read_until(',' as u8).ok().and_then(|vec| {
      str::from_utf8(trim_markup(vec).as_slice()).map(|str_slice| {
        str_slice.to_owned()
      })
    });

    match maybe_name {
      Some(name) => { result.push(name); },
      None       => break
    }
  }

  return result;

  fn trim_markup(bytes: Vec<u8>) -> Vec<u8> {
    bytes.move_iter().filter(|&byte| {
      byte >= ('A' as u8) && byte <= ('Z' as u8)
    }).collect::<Vec<u8>>()
  }
}
