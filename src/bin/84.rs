/* Problem 84: Monopoly odds
 *
 * In the game, Monopoly, the standard board is set up in the following way:
 *
 * A player starts on the GO square and adds the scores on two 6-sided dice to determine the number
 * of squares they advance in a clockwise direction. Without any further rules we would expect to
 * visit each square with equal probability: 2.5%. However, landing on G2J (Go To Jail), CC
 * (community chest), and CH (chance) changes this distribution.
 *
 * GO  A1  CC1 A2  T1  R1  B1  CH1 B2  B3  JAIL
 * H2                                      C1
 * T2                                      U1
 * H1                                      C2
 * CH3                                     C3
 * R4                                      R2
 * G3                                      D1
 * CC3                                     CC2
 * G2                                      D2
 * G1                                      D3
 * G2J F3  U2  F2  F1  R3  E3  E2  CH2 E1  FP
 *
 * In addition to G2J, and one card from each of CC and CH, that orders the player to go directly
 * to jail, if a player rolls three consecutive doubles, they do not advance the result of their
 * 3rd roll. Instead they proceed directly to jail.
 *
 * At the beginning of the game, the CC and CH cards are shuffled. When a player lands on CC or CH
 * they take a card from the top of the respective pile and, after following the instructions, it
 * is returned to the bottom of the pile. There are sixteen cards in each pile, but for the purpose
 * of this problem we are only concerned with cards that order a movement; any instruction not
 * concerned with movement will be ignored and the player will remain on the CC/CH square.
 *
 * * Community Chest (2/16 cards):
 *   - Advance to GO
 *   - Go to JAIL
 * * Chance (10/16 cards):
 *   - Advance to GO
 *   - Go to JAIL
 *   - Go to C1
 *   - Go to E3
 *   - Go to H2
 *   - Go to R1
 *   - Go to next R (railway company)
 *   - Go to next R
 *   - Go to next U (utility company)
 *   - Go back 3 squares.
 *
 * The heart of this problem concerns the likelihood of visiting a particular square. That is, the
 * probability of finishing at that square after a roll. For this reason it should be clear that,
 * with the exception of G2J for which the probability of finishing on it is zero, the CH squares
 * will have the lowest probabilities, as 5/8 request a movement to another square, and it is the
 * final square that the player finishes at on each roll that we are interested in. We shall make
 * no distinction between "Just Visiting" and being sent to JAIL, and we shall also
 * ignore the rule about requiring a double to "get out of jail", assuming that they pay
 * to get out on their next turn.
 *
 * By starting at GO and numbering the squares sequentially from 00 to 39 we can concatenate these
 * two-digit numbers to produce strings that correspond with sets of squares.
 *
 * Statistically it can be shown that the three most popular squares, in order, are JAIL (6.24%) =
 * Square 10, E3 (3.18%) = Square 24, and GO (3.09%) = Square 00. So these three most popular
 * squares can be listed with the six-digit modal string: 102400.
 *
 * If, instead of using two 6-sided dice, two 4-sided dice are used, find the six-digit modal
 * string.
 **/

use hashbrown::HashMap;
use rand::prelude::*;
use rayon::prelude::*;
use std::collections::VecDeque;

const ROLLS_PER_SIMULATION: u32 = 10_000_000;
const SIMULATIONS: u32 = 10;
const DICE_SIZE: u32 = 4;

fn main() {
    let result = (0..SIMULATIONS)
        .into_par_iter()
        .map_init(
            || thread_rng(),
            |thread_rng, _| {
                let mut rng = SmallRng::from_rng(thread_rng).unwrap();
                let mut b = build_board(&mut rng);

                for _ in 0..ROLLS_PER_SIMULATION {
                    run(&mut b, &mut rng);
                }

                b.squares
            },
        )
        .fold(
            || HashMap::new(),
            |mut h, squares| {
                for BoardSquare { index, visits, .. } in squares {
                    h.entry(index)
                        .and_modify(|c| *c += visits)
                        .or_insert(visits);
                }
                h
            },
        )
        .reduce(
            || HashMap::new(),
            |mut acc, h| {
                for (square_index, visits) in h {
                    acc.entry(square_index)
                        .and_modify(|c| *c += visits)
                        .or_insert(visits);
                }
                acc
            },
        );

    let mut result: Vec<_> = result.into_iter().collect();
    result.par_sort_unstable_by_key(|(_, visits)| std::cmp::Reverse(*visits));

    for (index, _) in result.into_iter().take(3) {
        print!("{:02}", index);
    }
    println!();
}

fn run<R: Rng>(board: &mut Board, rng: &mut R) {
    let index = {
        let next_square = match roll_dice(board, rng) {
            DiceResult::TripleDouble => board.get_square(Square::JAIL),
            DiceResult::Normal(step) => board.square_after_current(step as i32),
        };

        next_square.visits += 1;
        next_square.index
    };

    board.current_square = index;
}

fn roll_dice<R: Rng>(board: &mut Board, rng: &mut R) -> DiceResult {
    use rand::distributions::Uniform;

    let dice_range = Uniform::new_inclusive(1, DICE_SIZE);

    let d1 = dice_range.sample(rng);
    let d2 = dice_range.sample(rng);

    board.double_streak = if d1 == d2 { board.double_streak + 1 } else { 0 };

    if board.double_streak == 3 {
        board.double_streak = 0;
        return DiceResult::TripleDouble;
    }

    DiceResult::Normal(d1 + d2)
}

enum DiceResult {
    TripleDouble,
    Normal(u32),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Square {
    A(u8),
    B(u8),
    C(u8),
    CC(u8),
    CH(u8),
    D(u8),
    E(u8),
    F(u8),
    FP,
    G(u8),
    G2J,
    GO,
    H(u8),
    JAIL,
    R(u8),
    T(u8),
    U(u8),
}

#[derive(Debug, Clone, Copy)]
enum SpecialCard {
    AdvanceToGO,
    GoBack3Squares,
    GoToC1,
    GoToE3,
    GoToH2,
    GoToJAIL,
    GoToNextR,
    GoToNextU,
    GoToR1,
    Noop,
}

#[derive(Debug)]
struct Board {
    squares: Vec<BoardSquare>,
    current_square: usize,
    community_chest_cards: VecDeque<SpecialCard>,
    chance_cards: VecDeque<SpecialCard>,
    double_streak: u32,
}

impl Board {
    fn get_square(&mut self, square: Square) -> &mut BoardSquare {
        self.squares
            .iter_mut()
            .find(|bs| bs.square == square)
            .unwrap()
    }

    fn square_after_current(&mut self, step: i32) -> &mut BoardSquare {
        self.square_after(self.current_square, step)
    }

    fn square_after(&mut self, base_square: usize, step: i32) -> &mut BoardSquare {
        use SpecialCard::*;

        let (square_index, square_type) = self.advance_step(base_square, step);

        let special_card = match square_type {
            Square::CC(_) => pop_special_card(&mut self.community_chest_cards),
            Square::CH(_) => pop_special_card(&mut self.chance_cards),
            Square::G2J => GoToJAIL,
            _ => Noop,
        };

        match special_card {
            Noop => &mut self.squares[square_index],
            AdvanceToGO => self.get_square(Square::GO),
            GoBack3Squares => self.square_after(square_index, -3),
            GoToC1 => self.get_square(Square::C(1)),
            GoToE3 => self.get_square(Square::E(3)),
            GoToH2 => self.get_square(Square::H(2)),
            GoToJAIL => self.get_square(Square::JAIL),
            GoToR1 => self.get_square(Square::R(1)),
            GoToNextR => self.next_r_square(square_index),
            GoToNextU => self.next_u_square(square_index),
        }
    }

    fn advance_step(&mut self, base_square: usize, step: i32) -> (usize, Square) {
        let square_count = self.squares.len();
        let step = if step >= 0 {
            step as usize
        } else {
            square_count - (-step as usize)
        };

        let square_index = (base_square + step) % square_count;
        let square_type = self.squares[square_index].square.clone();

        (square_index, square_type)
    }

    fn next_r_square(&mut self, base_square: usize) -> &mut BoardSquare {
        self.find_next_square(base_square, |bs| match bs.square {
            Square::R(_) => true,
            _ => false,
        })
    }

    fn next_u_square(&mut self, base_square: usize) -> &mut BoardSquare {
        self.find_next_square(base_square, |bs| match bs.square {
            Square::U(_) => true,
            _ => false,
        })
    }

    fn find_next_square(
        &mut self,
        base_square: usize,
        f: impl Fn(&BoardSquare) -> bool,
    ) -> &mut BoardSquare {
        let index = self
            .squares
            .iter()
            .cycle()
            .skip_while(|bs| bs.index != base_square)
            .skip(1)
            .position(f)
            .unwrap();
        &mut self.squares[index]
    }
}

fn pop_special_card(cards: &mut VecDeque<SpecialCard>) -> SpecialCard {
    let card = cards.pop_front().unwrap();
    cards.push_back(card.clone());
    card
}

#[derive(Debug)]
struct BoardSquare {
    index: usize,
    square: Square,
    visits: u32,
}

fn build_board<R: Rng>(rng: &mut R) -> Board {
    use SpecialCard::*;
    use Square::*;

    let mut bs_count = 0;
    let mut bs = move |square| {
        bs_count += 1;
        BoardSquare {
            square,
            visits: 0,
            index: bs_count - 1,
        }
    };

    #[rustfmt::skip]
    let squares = vec![
        bs(GO),    bs(A(1)),  bs(CC(1)), bs(A(2)),  bs(T(1)), bs(R(1)),  bs(B(1)),
        bs(CH(1)), bs(B(2)),  bs(B(3)),  bs(JAIL),  bs(C(1)), bs(U(1)),  bs(C(2)),
        bs(C(3)),  bs(R(2)),  bs(D(1)),  bs(CC(2)), bs(D(2)), bs(D(3)),  bs(FP),
        bs(E(1)),  bs(CH(2)), bs(E(2)),  bs(E(3)),  bs(R(3)), bs(F(1)),  bs(F(2)),
        bs(U(2)),  bs(F(3)),  bs(G2J),   bs(G(1)),  bs(G(2)), bs(CC(3)), bs(G(3)),
        bs(R(4)),  bs(CH(3)), bs(H(1)),  bs(T(2)),  bs(H(2)),
    ];

    let community_chest_cards = init_special_cards(rng, vec![AdvanceToGO, GoToJAIL], 16);
    let chance_cards = init_special_cards(
        rng,
        vec![
            AdvanceToGO,
            GoToJAIL,
            GoToC1,
            GoToE3,
            GoToH2,
            GoToR1,
            GoToNextR,
            GoToNextR,
            GoToNextU,
            GoBack3Squares,
        ],
        16,
    );

    Board {
        squares,
        current_square: 0,
        double_streak: 0,
        community_chest_cards,
        chance_cards,
    }
}

fn init_special_cards<R: Rng>(
    rng: &mut R,
    mut v: Vec<SpecialCard>,
    total: usize,
) -> VecDeque<SpecialCard> {
    v.resize(total, SpecialCard::Noop);
    v.shuffle(rng);

    v.into_iter().collect()
}
