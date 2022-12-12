use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
struct Movement {
    count: u32,
    from: u32,
    to: u32,
}

struct State {
    towers: Vec<Vec<char>>,
    moves: Vec<Movement>,
}

fn parse_file(filepath: &str) -> Result<State, String> {
    let contents = read_to_string(filepath).map_err(|err| err.to_string())?;

    let state_lines: Vec<&str> = contents
        .split("\n")
        .filter(|line| !line.starts_with("move"))
        .collect();

    // get the line for the base of the towers
    let base = state_lines[state_lines.len() - 2];

    // Map each letter in s to its index
    let tower_no_to_index: Vec<usize> = base
        .chars()
        .enumerate()
        .filter(|(_index, tower_no)| *tower_no != ' ')
        .map(|(index, _tower_no)| index)
        .collect();

    let number_of_towers = tower_no_to_index.len();

    println!("{:#?}", number_of_towers);

    let mut disks: Vec<Vec<char>> = vec![Vec::new(); number_of_towers];

    for line in state_lines.iter().rev().skip(2) {
        for (i, chr) in line.chars().enumerate() {
            if chr.is_ascii_uppercase() {
                let tower_index = tower_no_to_index
                    .iter()
                    .position(|&x| x == i)
                    .expect("Tower index not found");
                disks[tower_index].push(chr);
            }
        }
    }

    let moves: Vec<Movement> = contents
        .split("\n")
        .filter(|line| line.starts_with("move"))
        .map(|line| {
            let nums: Vec<u32> = line
                .split(" ")
                .into_iter()
                .filter(|word| word.chars().all(|c| c.is_numeric()))
                .map(|word| word.parse::<u32>().unwrap())
                .collect();
            let count = nums[0];
            let from = nums[1];
            let to = nums[2];
            Movement { count, from, to }
        })
        .collect();

    Ok(State {
        towers: disks,
        moves: moves,
    })
}

fn make_movements(state: &mut State) {
    for movement in state.moves.iter() {
        let from = movement.from as usize - 1;
        let to = movement.to as usize - 1;
        let count = movement.count as usize;

        let cur_tower = &mut state.towers[from];
        let cur_tower_len = cur_tower.len();

        let mut disks_to_move: Vec<char> = cur_tower.drain(cur_tower_len - count..).collect();

        // insert in reverse order on target tower
        // disks_to_move.reverse(); # Uncomment for part 1
        state.towers[to].append(&mut disks_to_move);
    }
}

fn main() {
    let mut state = parse_file("input.txt").unwrap();

    println!("{:?}", state.towers);
    println!("{:?}", state.moves);

    make_movements(&mut state);

    let top_letters = state
        .towers
        .iter()
        .map(|tower| tower[tower.len() - 1])
        .collect::<String>();

    println!("{:?}", top_letters);
}
