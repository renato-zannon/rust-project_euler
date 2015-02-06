/* Problem 67: Maximum path sum II
 *
 * By starting at the top of the triangle below and moving to adjacent numbers on the row below, the
 * maximum total from top to bottom is 23.
 *
 *    3
 *   7 4
 *  2 4 6
 * 8 5 9 3
 *
 * That is, 3 + 7 + 4 + 9 = 23.
 *
 * Find the maximum total from top to bottom in triangle.txt, a 15K text file containing a triangle
 * with one-hundred rows.
 *
 * NOTE: This is a much more difficult version of Problem 18. It is not possible to try every route
 * to solve this problem, as there are 299 altogether! If you could check one trillion (1012) routes
 * every second it would take over twenty billion years to check them all. There is an efficient
 * algorithm to solve it. ;o) */

#![feature(core, io, path)]

extern crate shared;

use std::old_io::{File, BufferedReader};

use shared::triangle;

fn main() {
    let mut triangle = triangle::new(&deep_slice(&read_triangle())[]);
    let result = triangle.maximum_total();

    println!("{}", result);
}

fn read_triangle() -> Vec<Vec<u32>> {
    let path = &Path::new("./data/67-triangle.txt");
    let mut file = BufferedReader::new(File::open(path));

    let mut result = Vec::new();

    for line in file.lines() {
        let line_text = line.unwrap();
        let mut parsed_line = Vec::new();

        for atom in line_text.trim().split(' ') {
            match atom.parse::<u32>() {
                Ok(num) => parsed_line.push(num),
                Err(_)  => continue,
            }
        }

        result.push(parsed_line);
    }

    result
}

fn deep_slice(v: &Vec<Vec<u32>>) -> Vec<&[u32]> {
    let mut result: Vec<&[u32]> = Vec::new();

    for row in v.iter() {
        result.push(&row[]);
    }

    result
}
