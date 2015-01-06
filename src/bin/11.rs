/*Problem 11: Largest product in a grid
 *
 * What is the greatest product of four adjacent numbers in the same direction (up, down, left,
 * right, or diagonally) in the 20×20 grid? */

#![feature(associated_types)]

static GRID: [[uint; 20]; 20] = [
    [08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08],
    [49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00],
    [81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65],
    [52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91],
    [22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80],
    [24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50],
    [32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70],
    [67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21],
    [24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72],
    [21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95],
    [78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92],
    [16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57],
    [86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58],
    [19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40],
    [04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66],
    [88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69],
    [04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36],
    [20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16],
    [20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54],
    [01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48],
];

const SLICE_LENGTH: uint = 4;

fn main() {
    let result = coordinates().iter().flat_map(|&(row, column)| {
        slices_for(row, column)
    }).map(|slice| slice.iter().fold(1, |acc, &num| acc * num)).max().unwrap();

    println!("{}", result);
}

fn coordinates() -> Vec<(uint, uint)>  {
    let mut result = vec!();

    for row in range(0u, 20) {
        for column in range(0u, 20) {
            result.push((row, column));
        }
    }

    return result;
}

fn slices_for(row: uint, column: uint) -> SlicesFromPoint<'static> {
    SlicesFromPoint::new(row, column, SLICE_LENGTH, &GRID)
}

struct SlicesFromPoint<'a> {
    grid: &'a [[uint; 20]; 20],
    row: uint,
    column: uint,
    length: uint,
    last_slice_type: Option<SliceType>
}

enum SliceType {
    Horizontal,
    Vertical,
    DiagonalFromLeft,
    DiagonalFromRight
}

impl<'a> SlicesFromPoint<'a> {
    fn new(row: uint, column: uint, length: uint, grid: &[[uint; 20]; 20]) -> SlicesFromPoint {
        SlicesFromPoint {
            grid: grid,
            row: row,
            column: column,
            length: length,
            last_slice_type: None,
        }
    }

    fn horizontal_slice(&mut self) -> Option<Vec<uint>> {
        let limit = 20 - self.length;
        self.last_slice_type = Some(SliceType::Horizontal);

        match (self.row, self.column) {
            (_, col) if col <= limit => {
                let slice = self.make_slice(|(r, c)| (r, c + 1));
                Some(slice)
            },

            _ => self.next()
        }
    }

    fn vertical_slice(&mut self) -> Option<Vec<uint>> {
        let limit = 20 - self.length;
        self.last_slice_type = Some(SliceType::Vertical);

        match (self.row, self.column) {
            (row, _) if row <= limit => {
                let slice = self.make_slice(|(r, c)| (r + 1, c));
                Some(slice)
            },

            _ => self.next()
        }
    }

    fn diagonal_from_left_slice(&mut self) -> Option<Vec<uint>> {
        let limit = 20 - self.length;
        self.last_slice_type = Some(SliceType::DiagonalFromLeft);

        match (self.row, self.column) {
            (row, col) if row <= limit && col <= limit => {
                let slice = self.make_slice(|(r, c)| (r + 1, c + 1));
                Some(slice)
            },

            _ => self.next()
        }
    }

    fn diagonal_from_right_slice(&mut self) -> Option<Vec<uint>> {
        let limit = 20 - self.length;
        self.last_slice_type = Some(SliceType::DiagonalFromRight);

        match (self.row, self.column) {
            (row, col) if row <= limit && col >= self.length => {
                let slice = self.make_slice(|(r, c)| (r + 1, c - 1));
                Some(slice)
            },

            _ => self.next()
        }
    }

    fn make_slice(&self, increment: |(uint, uint)| -> (uint, uint)) -> Vec<uint> {
        let mut result    = vec!(self.grid[self.row][self.column]);
        let mut prev_pair = (self.row, self.column);

        for _ in range(0, self.length - 1) {
            let next_pair = increment(prev_pair);

            let (row, col) = next_pair;
            result.push(self.grid[row][col]);

            prev_pair = next_pair;
        }

        result
    }
}

impl<'a> Iterator for SlicesFromPoint<'a> {
    type Item = Vec<uint>;

    fn next(&mut self) -> Option<Vec<uint>> {
        match self.last_slice_type {
            None                               => self.horizontal_slice(),
            Some(SliceType::Horizontal)        => self.vertical_slice(),
            Some(SliceType::Vertical)          => self.diagonal_from_left_slice(),
            Some(SliceType::DiagonalFromLeft)  => self.diagonal_from_right_slice(),
            Some(SliceType::DiagonalFromRight) => None
        }
    }
}
