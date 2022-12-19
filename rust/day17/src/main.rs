use core::panic;
use indicatif::{ProgressBar, ProgressIterator};
use std::collections::HashMap;

struct Piece {
    shape: Vec<(i32, i32)>,
}

impl Piece {
    fn new(piece_no: i32) -> Self {
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

    fn get_coords(&self, left: i32, top: i32) -> Vec<(i32, i32)> {
        self.shape
            .iter()
            .map(|(x, y)| (left + x, top + y))
            .collect()
    }
}

struct Board {
    pieces: HashMap<(i32, i32), Piece>,
    falling_p: Option<((i32, i32), Piece)>,
    width: i32,
    highest: i32,
}

impl Board {
    fn new(width: i32) -> Board {
        Board {
            width,
            pieces: HashMap::new(),
            highest: -1,
            falling_p: None,
        }
    }

    fn collides(&self, piece: &Piece, pos: (i32, i32)) -> bool {
        // if the lowest point of this piece is higher than the highest point of the board,
        // just return false
        if piece.shape.iter().max_by_key(|(_, y)| y).unwrap().1 + pos.1 > self.highest {
            return false;
        }

        for (x, y) in piece.get_coords(pos.0, pos.1) {
            // Check for the floor, walls, and other pieces
            if x < 0 || x >= self.width || y < 0 {
                return true;
            }
            for ((ox, oy), other_piece) in &self.pieces {
                if other_piece.get_coords(*ox, *oy).contains(&(x, y)) {
                    return true;
                }
            }
        }

        false
    }

    fn run_simulation(&mut self, input_pattern: &str, max_rock_count: i32) {
        let mut piece_no = 0;
        let mut pat_iter = input_pattern.chars().cycle();

        // After a rock appears, it alternates between being pushed
        // by a jet of hot gas one unit (in the direction indicated
        // by the next symbol in the jet pattern) and then falling
        // one unit down.

        let bar = ProgressBar::new(max_rock_count as u64);

        while piece_no <= max_rock_count {
            bar.set_position(piece_no as u64);
            self.print();
            let falling_piece = self.falling_p.take();

            match falling_piece {
                Some(((mut x, mut y), p)) => {
                    // Alternate between pushing and falling
                    let nx = match pat_iter.next().unwrap() {
                        '>' => {
                            dbg!("Jet pushes rock right");
                            x + 1
                        }
                        '<' => {
                            dbg!("Jet pushes rock left");
                            x - 1
                        }
                        _ => panic!("Invalid input pattern"),
                    };

                    // If any movement would cause any part of the rock
                    // to move into the walls, floor, or a stopped rock,
                    // the movement instead does not occur.
                    if !self.collides(&p, (nx, y)) {
                        x = nx;
                    } else {
                        dbg!("...but rock collides with something, so it doesn't move");
                    }

                    // Check if the piece is colliding with something below
                    if !self.collides(&p, (x, y - 1)) {
                        // Keep falling
                        self.falling_p = Some(((x, y - 1), p));
                        dbg!("Rock falls 1 unit");
                    } else {
                        // Update the highest point
                        p.get_coords(x, y).iter().for_each(|(x, y)| {
                            self.highest = self.highest.max(*y);
                        });

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
                    dbg!("Rock {piece_no} begins falling");
                }
            }
        }
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
            println!();
        }
        for _ in 0..self.width + 2 {
            print!("-");
        }
        println!();
    }
}

fn main() {}

#[test]
fn test_example() {
    let input_pattern = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    let mut board = Board::new(7);
    let max_rock_count = 2022;

    board.run_simulation(input_pattern, max_rock_count);

    assert_eq!(board.pieces.len(), 2022);
    assert_eq!(board.highest, 3068);

    board.print();
}
