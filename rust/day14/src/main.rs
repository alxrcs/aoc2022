use std::{
    cmp::{max, min},
    collections::HashSet,
    fs::read_to_string,
};

struct RockStructure {
    coords: Vec<(i32, i32)>, // (x, y)*;
}

fn parse(text: String) -> Vec<RockStructure> {
    // Example format:
    // 498,4 -> 498,6 -> 496,6
    // 503,4 -> 502,4 -> 502,9 -> 494,9

    let mut structures = Vec::new();

    for line in text.lines() {
        let mut coords = Vec::new();

        for coord in line.split("->") {
            let coord = coord.trim();
            let coord: Vec<i32> = coord
                .split(",")
                .map(|x| x.parse::<i32>().expect("Unable to parse coordinate"))
                .collect();

            coords.push((coord[0], coord[1]));
        }

        structures.push(RockStructure { coords });
    }

    structures
}

fn check_for_collision(
    position: (i32, i32),
    rocks: &Vec<RockStructure>,
    stopped_sand: &HashSet<(i32, i32)>,
) -> bool {
    // Checks if the given position collides with any of the segments of the rock structures
    // Returns true if there is a collision, false otherwise

    // Check for collisions with the stopped sand
    if stopped_sand.contains(&position) {
        return true;
    }

    for rock in rocks {
        for i in 0..rock.coords.len() - 1 {
            let (x1, y1) = rock.coords[i];
            let (x2, y2) = rock.coords[i + 1];

            if x1 == x2 {
                // Vertical segment
                if x1 == position.0 && position.1 >= min(y1, y2) && position.1 <= max(y1, y2) {
                    return true;
                }
            } else {
                // Horizontal segment
                if y1 == position.1 && position.0 >= min(x1, x2) && position.0 <= max(x1, x2) {
                    return true;
                }
            }
        }
    }

    false
}

fn get_next_collision_point_for_grain(
    initial_position: (i32, i32),
    rocks: &Vec<RockStructure>,
    stopped_sand: &HashSet<(i32, i32)>,
    lower_bound: i32,
) -> Option<(i32, i32)> {
    // Simulates the falling sand from the given position
    // Returns the final position of the sand

    let mut pos = initial_position;

    loop {
        // Check if the sand has reached the bottom
        // (Part 1 )
        // if pos.1 >= lowest_horizontal_segment_height {
        //     return None;
        // }
        if pos.1 >= lower_bound {
            return Some(pos);
        }

        // Try moving the rock to the position just below
        let below = (pos.0, pos.1 + 1);

        // Try moving the rock to the position to the down-left
        let down_left = (pos.0 - 1, pos.1 + 1);

        // Try moving the rock to the position to the down-right
        let down_right = (pos.0 + 1, pos.1 + 1);

        let positions_to_try = vec![below, down_left, down_right];

        for pos_to_try in positions_to_try.iter() {
            if !check_for_collision(*pos_to_try, rocks, stopped_sand) {
                pos = *pos_to_try;
                break;
            }
        }

        if positions_to_try
            .iter()
            .all(|p| check_for_collision(*p, rocks, stopped_sand))
        {
            return Some(pos);
        }
    }
}

fn main() {
    let mut state = parse(read_to_string("input.txt").expect("Unable to read file"));

    println!("Found {} rock structures", state.len());
    for rock in &state {
        println!("Rock structure with {:?} coordinates", rock.coords);
    }

    let initial_position = (500, 0);
    let mut stopped_sand: HashSet<(i32, i32)> = HashSet::new();

    // Find the lowest horizontal segment that the sand can fall to
    // If there is no such segment, return None
    let lowest_horizontal_segment_height = state
        .iter()
        .flat_map(|r| r.coords.iter())
        .map(|c| c.1)
        .max()
        .unwrap()
        + 2;

    // Part 1 - Count the number of sand grains that reach the bottom before going to infinity
    // let mut sand_count = 0;
    // loop {
    //     let sand_pos = get_next_collision_point_for_grain(initial_position, &state, &stopped_sand);

    //     match sand_pos {
    //         Some(pos) => {
    //             println!("Sand stopped at {:?}", pos);
    //             stopped_sand.insert(pos);
    //         }
    //         None => {
    //             println!("Sand reached the bottom");
    //             break;
    //         }
    //     }

    //     sand_count += 1;
    // }

    // Part 2 - Count the number of sand grains before one stops at (500, 0)
    // Basically brute force and takes a while, but hey! It works! :)
    let mut sand_count = 0;

    // minimum value for i32
    let neg_inf = std::i32::MIN;
    let max_inf = std::i32::MAX;

    let floor = RockStructure {
        coords: vec![
            (neg_inf, lowest_horizontal_segment_height),
            (max_inf, lowest_horizontal_segment_height),
        ],
    };

    state.push(floor);

    loop {
        let sand_pos = get_next_collision_point_for_grain(
            initial_position,
            &state,
            &stopped_sand,
            lowest_horizontal_segment_height,
        );

        match sand_pos {
            Some(pos) => {
                println!("Sand stopped at {:?}", pos);
                if pos == (500, 0) {
                    break;
                }
                stopped_sand.insert(pos);
            }
            None => {
                panic!("Shouldn't happen!")
            }
        }

        sand_count += 1;
    }

    println!("Sand count: {}", sand_count + 1);
}
