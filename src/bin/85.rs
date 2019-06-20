/* Problem 85: Counting rectangles
 *
 * By counting carefully it can be seen that a rectangular grid measuring 3 by 2 contains eighteen
 * rectangles.
 *
 * Although there exists no rectangular grid that contains exactly two million rectangles, find the
 * area of the grid with the nearest solution.
 **/

const TARGET: u32 = 2_000_000;

fn main() {
    let max_width = (1..)
        .take_while(|&width| rectangles_in_area(width, 1) <= TARGET)
        .last()
        .unwrap();

    let ((width, height), _) = (1..=max_width)
        .map(|width| {
            (1..=width)
                .map(move |height| ((width, height), rectangles_in_area(width, height)))
                .take_while(|(_, rectangles)| *rectangles <= TARGET)
                .last()
                .unwrap()
        })
        .max_by_key(|(_, rectangles)| *rectangles)
        .unwrap();

    println!("{}", width * height);
}

fn rectangles_in_area(width: u32, height: u32) -> u32 {
    let mut total = 0;

    for r_width in 1..=width {
        let complete_widths = width - r_width + 1;

        for r_height in 1..=height {
            let complete_heights = height - r_height + 1;

            total += complete_widths * complete_heights;
        }
    }

    total
}
