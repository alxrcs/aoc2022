use std::{collections::HashSet, io::Read};

#[derive(Debug)]
enum Motion {
    Up,
    Down,
    Left,
    Right,
}

fn read_motions(filename: String) -> Vec<(Motion, usize)> {
    let mut motions = Vec::new();
    let mut input = String::new();

    std::fs::File::open(filename)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    for line in input.lines() {
        let mut chars = line.split(" ");
        let motion = match chars.next().unwrap() {
            "U" => Motion::Up,
            "D" => Motion::Down,
            "L" => Motion::Left,
            "R" => Motion::Right,
            _ => panic!("Invalid motion"),
        };
        let distance = chars.collect::<String>().parse::<usize>().unwrap();
        motions.push((motion, distance));
    }
    motions
}

fn main() {
    let motions = read_motions("input.txt".to_string());
    println!("{:?}", motions);

    let rope_length = 10;
    let mut rope_segments = vec![(0, 0); rope_length];

    let windowed_indices = (0..rope_segments.len())
        .zip(1..rope_segments.len())
        .collect::<Vec<_>>();

    let mut visited_by_tail: HashSet<(i32, i32)> = HashSet::new();
    visited_by_tail.insert((0, 0));

    for (motion, distance) in motions {
        for _ in 0..distance {
            let head_pos: &mut (i32, i32) = rope_segments.get_mut(0).unwrap();

            // Move the head in the direction of the motion.
            match motion {
                Motion::Up => head_pos.1 += 1,
                Motion::Down => head_pos.1 -= 1,
                Motion::Left => head_pos.0 -= 1,
                Motion::Right => head_pos.0 += 1,
            }

            // Move the remaining segments of the rope.
            // for t in rope_segments.windows(2) {
            // let sub_head: (i32, i32) = t[0];
            // let mut sub_tail: &mut (i32, i32) = t[1];

            for (sub_head_idx, sub_tail_idx) in windowed_indices.iter() {
                let sub_head = rope_segments[*sub_head_idx];
                let mut sub_tail = &mut rope_segments[*sub_tail_idx];

                // Calculate the difference vector between the head and tail.
                let (dx, dy): (i32, i32) = (
                    (sub_head.0 - sub_tail.0).signum(),
                    (sub_head.1 - sub_tail.1).signum(),
                );

                // Calculate the Chebyshev distance between the head and tail.
                let dist = (sub_head.0 - sub_tail.0)
                    .abs()
                    .max((sub_head.1 - sub_tail.1).abs());

                if dist > 1 {
                    // Move the tail in the direction of the head.
                    sub_tail.0 += dx;
                    sub_tail.1 += dy;
                }
            }

            let tail_pos = rope_segments[rope_length - 1];
            // Mark the position of the tail as visited.
            println!("({}, {})", tail_pos.0, tail_pos.1);
            visited_by_tail.insert(tail_pos);
        }
    }
    // Print the number of visited positions.
    println!("Visited {} unique positions", visited_by_tail.len());
}
