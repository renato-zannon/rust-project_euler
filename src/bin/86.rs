/* Problem 86: Cuboid route
 *
 * A spider, S, sits in one corner of a cuboid room, measuring 6 by 5 by 3, and a fly, F, sits in
 * the opposite corner. By travelling on the surfaces of the room the shortest "straight line"
 * distance from S to F is 10 and the path is shown on the diagram.
 *
 * However, there are up to three "shortest" path candidates for any given cuboid and the shortest
 * route doesn't always have integer length.
 *
 * It can be shown that there are exactly 2060 distinct cuboids, ignoring rotations, with integer
 * dimensions, up to a maximum size of M by M by M, for which the shortest route has integer length
 * when M = 100. This is the least value of M for which the number of solutions first exceeds two
 * thousand; the number of solutions when M = 99 is 1975.
 *
 * Find the least value of M such that the number of solutions first exceeds one million.
 **/

const TARGET: u32 = 1_000_000;
const BASE_M: u32 = 100;

fn main() {
    let mut count = int_solutions_until(BASE_M);

    for m in BASE_M.. {
        count += int_solutions(m);

        if count >= TARGET {
            println!("{}", m);
            break;
        }
    }
}

fn int_solutions_until(m: u32) -> u32 {
    let mut count = 0;

    for a in 1..=m {
        count += int_solutions(a);
    }

    count
}

fn int_solutions(a: u32) -> u32 {
    let mut count = 0;

    for b in 1..=a {
        for c in 1..=b {
            let path = shortest_path(a as f32, b as f32, c as f32);
            if is_integer(path) {
                count += 1;
            }
        }
    }

    count
}

fn shortest_path(a: f32, b: f32, c: f32) -> f32 {
    // d² = a² + (b + c)²
    let d2 = (a * a) + (b * b) + (c * c) + (2.0 * b * c);

    d2.sqrt()
}

fn is_integer(n: f32) -> bool {
    n - n.trunc() == 0.0
}
