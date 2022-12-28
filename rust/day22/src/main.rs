use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let state = parse(input);
    execute_instructions(state);
}

#[derive(Debug)]
enum Instruction {
    Walk(i32),
    TurnLeft,
    TurnRight,
}

struct State {
    bounds: Vec<(i32, i32)>,
    obstacles: HashSet<(i32, i32)>,
    pos: (i32, i32),
    dir: usize,
    path: Vec<Instruction>,
}

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn parse(input: &str) -> State {
    let map_lines: Vec<&str> = input.lines().take_while(|l| !l.is_empty()).collect();

    let mut bounds: Vec<(i32, i32)> = vec![];
    let mut obstacles = HashSet::new();
    for (line_num, line) in map_lines.iter().enumerate() {
        let min_c = line.chars().position(|c| c != ' ').unwrap();
        let max_c = line.len();

        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                obstacles.insert((i as i32, line_num as i32));
            }
        }

        bounds.push((min_c as i32, max_c as i32));
    }

    // Parse the path with a regex
    let mut path: Vec<Instruction> = Vec::new();
    let re = regex::Regex::new(r"(\d+|[RL])").unwrap();
    for cap in re.captures_iter(input) {
        let cap = cap.get(1).unwrap().as_str();
        if let Ok(n) = cap.parse::<i32>() {
            path.push(Instruction::Walk(n));
        } else if cap == "R" {
            path.push(Instruction::TurnRight);
        } else if cap == "L" {
            path.push(Instruction::TurnLeft);
        }
    }

    // Find the leftmost open tile in the first row
    let mut pos = (0, 0);
    for (i, c) in map_lines.into_iter().next().unwrap().chars().enumerate() {
        if c == '.' {
            pos = (i as i32, 0);
            break;
        }
    }

    State {
        bounds,
        obstacles,
        pos,
        dir: 0,
        path,
    }
}

fn execute_instructions(state: State) {
    let mut state = state;
    for instruction in &state.path {
        match instruction {
            Instruction::Walk(n) => {
                println!(
                    "Walking {:?} steps in dir {}, from {:?}",
                    n, state.dir, state.pos
                );
                for _ in 0..*n {
                    let (dx, dy) = DIRS[state.dir];
                    let mut new_pos = (state.pos.0 + dx, state.pos.1 + dy);

                    // If outside the board, walk in the opposite direction until we hit a wall
                    if !in_bounds(&state, new_pos) {
                        let (dx, dy) = DIRS[(state.dir + 2) % 4];
                        let mut wrapped = state.pos;
                        while in_bounds(&state, wrapped) {
                            wrapped = (wrapped.0 + dx, wrapped.1 + dy);
                        }
                        new_pos = (wrapped.0 - dx, wrapped.1 - dy);
                        println!("Wrapped around from {:?} to {:?}", state.pos, new_pos)
                    }

                    if state.obstacles.contains(&new_pos) {
                        println!("Hit an obstacle at {:?}", new_pos);
                        break;
                    }
                    println!(
                        "[{}, {}] -> [{}, {}]",
                        state.pos.1, state.pos.0, new_pos.1, new_pos.0
                    );
                    state.pos = new_pos;
                }
            }
            Instruction::TurnLeft => {
                state.dir = (state.dir + 3) % 4;
            }
            Instruction::TurnRight => {
                state.dir = (state.dir + 1) % 4;
            }
        }
    }
    println!("Final position: {:?}", state.pos);
    println!("Final direction: {:?}", state.dir);

    let final_row = state.pos.1 + 1;
    let final_col = state.pos.0 + 1;
    let final_dir = state.dir;
    let password = 1000 * final_row + 4 * final_col + final_dir as i32;
    println!("Password: {}", password);
}

fn in_bounds(state: &State, pos: (i32, i32)) -> bool {
    let (x, y) = pos;

    if y < 0 || y >= state.bounds.len() as i32 {
        return false;
    }

    let (min_x, max_x) = state.bounds[y as usize];
    x >= min_x && x < max_x
}
