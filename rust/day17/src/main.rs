use core::panic;
use indicatif::ProgressBar;
use std::cmp::Ord;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

struct Piece {
    shape: Vec<(i64, i64)>,
}

impl Piece {
    fn new(piece_no: i64) -> Self {
        match piece_no % 5 {
            0 => Piece {
                /* #### */
                shape: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            },
            1 => Piece {
                /*
                .#.
                ###
                .#.
                */
                shape: vec![(1, 0), (0, -1), (1, -1), (2, -1), (1, -2)],
            },
            2 => Piece {
                /*
                ..#
                ..#
                ###
                */
                shape: vec![(2, 0), (2, -1), (0, -2), (1, -2), (2, -2)],
            },
            3 => Piece {
                /*
                #
                #
                #
                #
                */
                shape: vec![(0, 0), (0, -1), (0, -2), (0, -3)],
            },
            4 => Piece {
                /*
                ##
                ##
                 */
                shape: vec![(0, 0), (1, 0), (0, -1), (1, -1)],
            },
            _ => panic!("Has Math Broken?"),
        }
    }

    fn get_coords(&self, left: i64, top: i64) -> Vec<(i64, i64)> {
        self.shape
            .iter()
            .map(|(x, y)| (left + x, top + y))
            .collect()
    }
}

struct Board {
    pieces: HashMap<(i64, i64), Piece>,
    falling_p: Option<((i64, i64), Piece)>,
    width: i64,
    highest: i64,
    collision_index: HashSet<(i64, i64)>,
    // collision_index: BTreeSet<OrderedPair>,
}

impl Board {
    fn new(width: i64) -> Board {
        Board {
            width,
            pieces: HashMap::new(),
            highest: -1,
            falling_p: None,
            collision_index: HashSet::new(),
        }
    }

    fn collides(&self, piece: &Piece, pos: (i64, i64)) -> bool {
        for (x, y) in piece.get_coords(pos.0, pos.1) {
            // Check for the floor, walls, and other pieces
            if x < 0 || x >= self.width || y < 0 {
                return true;
            }
            // for ((ox, oy), other_piece) in &self.pieces {
            //     if other_piece.get_coords(*ox, *oy).contains(&(x, y)) {
            //         return true;
            //     }
            // }
            if self.collision_index.contains(&(x, y)) {
                return true;
            }
        }

        false
    }

    fn run_simulation(&mut self, input_pattern: &str, max_rock_count: i64) {
        let mut piece_no = 0;
        let mut pat_iter = input_pattern.chars().cycle().enumerate();
        let total_jets = input_pattern.len() as i64;
        let bar = ProgressBar::new(max_rock_count as u64);

        // After a rock appears, it alternates between being pushed
        // by a jet of hot gas one unit (in the direction indicated
        // by the next symbol in the jet pattern) and then falling
        // one unit down.

        let INSPECT_LEN = 500_000;
        let TOTAL_LOOP_LEN = 10_000;
        let bound = max_rock_count.min(INSPECT_LEN);

        // let (div, rem) = (max_rock_count / loop_len, max_rock_count % loop_len);
        let mut delta_heights: Vec<i64> = vec![];

        while piece_no <= bound {
            bar.set_position(piece_no as u64);
            // self.print();
            let falling_piece = self.falling_p.take();

            match falling_piece {
                Some(((mut x, mut y), p)) => {
                    // Alternate between pushing and falling
                    let (jet_i, nx) = match pat_iter.next().unwrap() {
                        (i, '>') => {
                            // dbg!("Jet pushes rock right");
                            (i, x + 1)
                        }
                        (i, '<') => {
                            // dbg!("Jet pushes rock left");
                            (i, x - 1)
                        }
                        _ => panic!("Invalid input pattern"),
                    };

                    // If any movement would cause any part of the rock
                    // to move into the walls, floor, or a stopped rock,
                    // the movement instead does not occur.
                    if !self.collides(&p, (nx, y)) {
                        x = nx;
                    } else {
                        // dbg!("...but rock collides with something, so it doesn't move");
                    }

                    // Check if the piece is colliding with something below
                    if !self.collides(&p, (x, y - 1)) {
                        // Keep falling
                        self.falling_p = Some(((x, y - 1), p));
                        // dbg!("Rock falls 1 unit");
                    } else {
                        // Update the highest point
                        let old_height = self.highest;
                        p.get_coords(x, y).iter().for_each(|(x, y)| {
                            self.highest = self.highest.max(*y);
                            self.collision_index.insert((*x, *y));
                        });

                        delta_heights.push(self.highest - old_height);

                        // Stop falling
                        self.pieces.insert((x, y), p);

                        // Start a new piece
                        self.falling_p = None;
                    }
                }
                None => {
                    // Start a new piece
                    // Each rock appears so that its left edge is two units
                    // away from the left wall and its bottom edge is three
                    // units above the highest rock in the room (or the floor,
                    // if there isn't one).

                    let new_piece = Piece::new(piece_no);

                    let bottom_edge = new_piece.shape.iter().min_by_key(|(_, y)| y).unwrap().1;
                    let left_edge = new_piece.shape.iter().min_by_key(|(x, _)| x).unwrap().0;

                    self.falling_p =
                        Some(((left_edge + 2, self.highest - bottom_edge + 4), new_piece));

                    piece_no += 1;
                    // dbg!("Rock {piece_no} begins falling");
                }
            }
        }

        let mut turtle = 0;
        let mut hare = 0;

        loop {
            turtle += 1;
            hare += 2;

            if (0..TOTAL_LOOP_LEN).all(|i| delta_heights[turtle + i] == delta_heights[hare + i]) {
                println!("Found cycle at {} with length {}", turtle, hare - turtle);
                break;
            }
        }

        let mut answer = 0;

        for i in 0..turtle {
            answer += delta_heights[i];
        }

        let cycle_len = hare - turtle;
        let cycle_sum: i64 = delta_heights[turtle..hare].iter().sum();
        let full_cycle_times: i64 = (max_rock_count - turtle as i64) / cycle_len as i64;
        let remaining_steps: i64 = (max_rock_count - turtle as i64) % cycle_len as i64;

        answer += full_cycle_times * cycle_sum;

        for i in 0..remaining_steps as usize {
            answer += delta_heights[turtle + i];
        }

        println!("Answer: {}", answer);
    }

    fn print(&self) {
        for y in (0..self.highest + 7).rev() {
            print!("|");
            'row: for x in 0..self.width {
                // Check for the falling piece first
                match &self.falling_p {
                    Some(((px, py), piece)) => {
                        for (tx, ty) in &piece.shape {
                            if x == px + tx && y == py + ty {
                                print!("@");
                                continue 'row;
                            }
                        }
                    }
                    None => {}
                }

                // Check for the remaining pieces
                for ((px, py), piece) in &self.pieces {
                    for (tx, ty) in &piece.shape {
                        if x == px + tx && y == py + ty {
                            print!("#");
                            continue 'row;
                        }
                    }
                }

                print!(".");
            }
            print!("|");
            // println!();
        }
        for _ in 0..self.width + 2 {
            print!("-");
        }
        // println!();
    }
}

fn main() {
    let input_pattern = read_to_string("input.txt").unwrap();

    let mut board = Board::new(7);
    let max_rock_count: i64 = 1_000_000_000_000;

    board.run_simulation(&input_pattern, max_rock_count);
    println!("Highest point: {}", board.highest);
}

#[test]
fn test_example_p1() {
    let input_pattern = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    let mut board = Board::new(7);
    let max_rock_count = 2022;

    board.run_simulation(input_pattern, max_rock_count);

    assert_eq!(board.highest, 3068 - 1);
}

#[test]
fn test_example_p2() {
    let input_pattern = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    let mut board = Board::new(7);
    let max_rock_count = 1000000000000;

    board.run_simulation(input_pattern, max_rock_count);

    assert_eq!(board.highest, 1514285714288);

    board.print();
}
