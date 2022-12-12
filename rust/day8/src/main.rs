use std::fs::read_to_string;

fn main() {
    let file_name = "input.txt";
    let contents = read_to_string(file_name).expect("Something went wrong reading the file");

    // Interpret contents as a height map
    // Create a bidimensional array of integers
    let mut height_map: Vec<Vec<i32>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<i32> = Vec::new();
        for number in line.chars() {
            row.push(number.to_string().parse::<i32>().unwrap());
        }
        height_map.push(row);
    }

    // print the board
    for row in height_map.iter() {
        for number in row.iter() {
            print!("{} ", number);
        }
        println!();
    }

    // n is the number of rows
    let n: i32 = height_map.len().try_into().unwrap();
    // m is the number of columns
    let m: i32 = height_map.first().unwrap().len().try_into().unwrap();

    // These will guide the traversals of the height map
    // From top to bottom, right to left, bottom to top, left to right
    let directions: Vec<(i32, i32)> = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];
    let axis_increments: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let starting_points: Vec<(i32, i32)> = vec![(0, 0), (0, m - 1), (n - 1, m - 1), (n - 1, 0)];

    // Create a bidimensional array of booleans
    let mut visibility_bool_map =
        vec![vec![false; height_map.len()]; height_map.first().unwrap().len()];

    for (dir_i, (dx, dy)) in directions.iter().enumerate() {
        let (ax, ay) = axis_increments[dir_i];
        let (mut x0, mut y0) = starting_points[dir_i];

        while in_bounds(x0, n.try_into().unwrap(), y0, m.try_into().unwrap()) {
            let mut max_height = -1;
            let (mut i, mut j) = (x0, y0);
            while in_bounds(i, n.try_into().unwrap(), j, m.try_into().unwrap()) {
                let (x, y) = (i as usize, j as usize);
                if height_map[x][y] > max_height {
                    max_height = height_map[x][y];
                    visibility_bool_map[x][y] = true;
                }
                i += dx;
                j += dy;
            }

            x0 += ax;
            y0 += ay;
        }
    }

    // print the board
    for row in visibility_bool_map.iter() {
        for number in row.iter() {
            print!("{} ", number);
        }
        println!();
    }

    // Count the number of visible cells
    let mut visible_cells = 0;
    for row in visibility_bool_map.iter() {
        for number in row.iter() {
            if *number {
                visible_cells += 1;
            }
        }
    }

    // get the max scenic score
    let mut scenic_score = 0;
    for i in 0..n {
        for j in 0..m {
            let score = get_scenic_score(&height_map, (i, j));
            println!("Scenic score for cell ({}, {}): {}", i, j, score);
            if score > scenic_score {
                scenic_score = score;
            }
        }
    }

    println!("Visible cells: {}", visible_cells);
    println!("Max scenic score: {}", scenic_score);
}

fn get_scenic_score(height_map: &Vec<Vec<i32>>, (i, j): (i32, i32)) -> i32 {
    let (m, n) = (height_map.len(), height_map.first().unwrap().len());

    // Calculate the scenic score for the cell at (i, j)
    // Check the cells above, below, left and right
    let directions: Vec<(i32, i32)> = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut visible_cells: Vec<i32> = vec![];
    let max_height = height_map[i as usize][j as usize];

    for (dx, dy) in directions.iter() {
        let (mut x, mut y) = (i + dx, j + dy);

        let mut count = 0;
        while in_bounds(x, n.try_into().unwrap(), y, m.try_into().unwrap()) {
            count += 1;
            if height_map[x as usize][y as usize] >= max_height {
                break;
            }
            x += dx;
            y += dy;
        }

        if count == 0 {
            return 0;
        }
        visible_cells.push(count);
    }

    visible_cells.iter().fold(1, |a, b| a * b)
}

fn in_bounds(x: i32, n: i32, y: i32, m: i32) -> bool {
    x >= 0 && x < n && y >= 0 && y < m
}
