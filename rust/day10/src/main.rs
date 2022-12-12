use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, PartialEq, Eq, Hash)]
enum OpCode {
    AddX,
    NoOp,
}

#[derive(Debug)]
struct Op {
    opcode: OpCode,
    x: Option<i32>,
}

fn read_ops_from_file(path: String) -> Vec<Op> {
    read_to_string(path)
        .expect("Failed to read file")
        .lines()
        .map(|s| s.trim())
        .map(|s| {
            let mut ss = s.split(" ");
            let op = ss.next().unwrap();
            match op {
                "addx" => Op {
                    opcode: OpCode::AddX,
                    x: Some(ss.next().unwrap().parse().unwrap()),
                },
                "noop" => Op {
                    opcode: OpCode::NoOp,
                    x: None,
                },
                _ => panic!("Unknown op: {}", op),
            }
        })
        .collect()
}

fn get_char_for_cycle_and_pos(cycle: i32, register_state: i32, pos: (i32, i32)) -> char {
    // Register state is the horizontal pos of the sprite
    // Sprite has width 3
    // Cycle is the absolute number of cycles since the start of the program
    // The vertical position is the integer division of the cycle by 40

    let (x, y) = pos;

    let sprite_x = register_state;
    let sprite_y: i32 = cycle / 40;

    if y != sprite_y {
        return '.';
    }

    let sprite_width = 3;

    // Returns either a '.' or a '#' depending on whether the sprite is visible at the given position
    if x >= sprite_x - 1 && x < sprite_x - 1 + sprite_width {
        '#'
    } else {
        '.'
    }
}

fn main() {
    let ops = read_ops_from_file("input.txt".to_string());

    let cycle_lengths: HashMap<OpCode, i32> = HashMap::from([(OpCode::AddX, 2), (OpCode::NoOp, 1)]);
    let checkpoints = vec![20, 60, 100, 140, 180, 220];
    let _checkpoint_values = part_1(ops, cycle_lengths, checkpoints);

    // Part 2
    let ops2 = read_ops_from_file("input.txt".to_string());

    let max_cycle = 240;
    let checkpoints_2 = (0..=max_cycle).collect::<Vec<i32>>();
    let cycle_lengths2: HashMap<OpCode, i32> =
        HashMap::from([(OpCode::AddX, 2), (OpCode::NoOp, 1)]);

    let checkpoint_values_2 = part_1(ops2, cycle_lengths2, checkpoints_2);

    println!("Part 2:");
    for cycle in 0..max_cycle {
        let (x, y) = (cycle % 40, cycle / 40);
        print!(
            "{}",
            get_char_for_cycle_and_pos(cycle, checkpoint_values_2[(cycle + 1) as usize], (x, y))
        );
        if x == 39 {
            println!();
        }
    }
}

fn part_1(ops: Vec<Op>, cycle_lengths: HashMap<OpCode, i32>, checkpoints: Vec<i32>) -> Vec<i32> {
    let mut checkpoint_values: Vec<i32> = vec![];
    let mut next_checkpoint_idx = 0;
    let mut cycle = 0;
    let mut state = 1;
    for (_op_i, op) in ops.iter().enumerate() {
        if next_checkpoint_idx >= checkpoints.len() {
            // Stop if we've already covered all the checkpoints
            break;
        }

        // Increase cycle count
        cycle_lengths.get(&op.opcode).map(|cycle_length| {
            // Check if the current state is to be checkpointed
            while next_checkpoint_idx < checkpoints.len()
                && cycle + cycle_length >= checkpoints[next_checkpoint_idx]
            {
                checkpoint_values.push(state);
                next_checkpoint_idx += 1;
            }
            cycle += cycle_length;
        });

        match op.opcode {
            OpCode::AddX => {
                let num = op.x.unwrap();
                println!("Executing AddX {}", num);
                state += num;
            }
            OpCode::NoOp => {
                println!("Executing NoOp");
            }
        }

        println!("State at cycle {}: {}", cycle, state);
    }
    println!("Checkpoint values: {:?}", checkpoint_values);
    let signal_strength: i32 = checkpoints
        .iter()
        .zip(checkpoint_values.iter())
        .map(|(checkpoint, value)| checkpoint * value)
        .sum();
    println!("Signal strength: {}", signal_strength);
    checkpoint_values
}
