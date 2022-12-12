use std::fs::read_to_string;

fn parse_board(filename: String) -> (Vec<Vec<i32>>, (usize, usize), (usize, usize)) {
    let board = read_to_string(filename).unwrap();

    let board_len_i = board.lines().count();
    let board_len_j = board.lines().into_iter().next().unwrap().chars().count();

    let res: Vec<(usize, usize, i32)> = board
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars().enumerate().map(move |(j, c)| {
                let h = match c {
                    'a'..='z' => c as i32 - 96,
                    'S' => 'a' as i32 - 96,
                    'E' => 'z' as i32 - 96,
                    _ => panic!("Invalid character in board file"),
                };
                (i, j, h)
            })
        })
        .flatten()
        .collect();

    let mut start_pos: (usize, usize) = (0, 0);
    let mut end_pos: (usize, usize) = (0, 0);

    for (i, row) in board.lines().enumerate() {
        for (j, elem) in row.chars().enumerate() {
            if elem == 'S' {
                start_pos = (i, j);
            } else if elem == 'E' {
                end_pos = (i, j);
            }
        }
    }

    let mut board = vec![vec![0; board_len_j]; board_len_i];

    for (i, j, h) in res {
        board[i][j] = h;
    }

    (board, start_pos, end_pos)
}

fn in_bounds((i, j): (i32, i32), board: &Vec<Vec<i32>>) -> bool {
    i >= 0 && j >= 0 && i < board.len() as i32 && j < board[0].len() as i32
}

fn main() {
    let (board, start_pos, end_pos) = parse_board("input.txt".to_string());

    let res = part_1(board.clone(), start_pos, end_pos);
    println!("Part 1: {}", res);

    let mut squares_with_a: Vec<(usize, usize)> = vec![];
    for (i, row) in board.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            if *elem == 1 {
                squares_with_a.push((i, j));
            }
        }
    }

    let distances: Vec<i32> = squares_with_a
        .iter()
        .map(|&pos| part_1(board.clone(), pos, end_pos))
        .collect();

    let res = distances.iter().filter(|x| **x > 0).min().unwrap();
    println!("Part 2: {}", res);
}

fn part_1(board: Vec<Vec<i32>>, start_pos: (usize, usize), end_pos: (usize, usize)) -> i32 {
    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    let valid_moves = |(i, j): (i32, i32)| {
        dirs.iter()
            .map(|(di, dj)| (i + di, j + dj))
            .filter(|&pos| {
                in_bounds(pos, &board)
                    && (board[pos.0 as usize][pos.1 as usize] - board[i as usize][j as usize]) <= 1
            })
            .collect::<Vec<(i32, i32)>>()
    };

    let mut queue = vec![start_pos];
    let mut visited = vec![vec![false; board[0].len()]; board.len()];
    let mut dist = vec![vec![-1; board[0].len()]; board.len()];
    dist[start_pos.0][start_pos.1] = 0;

    while !queue.is_empty() {
        let pos = queue.remove(0);
        if visited[pos.0][pos.1] {
            continue;
        }
        visited[pos.0][pos.1] = true;
        for next_pos in valid_moves((pos.0 as i32, pos.1 as i32)) {
            if !visited[next_pos.0 as usize][next_pos.1 as usize] {
                queue.push((next_pos.0 as usize, next_pos.1 as usize));
                let height = board[next_pos.0 as usize][next_pos.1 as usize] - board[pos.0][pos.1];

                assert!(height <= 1);

                dist[next_pos.0 as usize][next_pos.1 as usize] = dist[pos.0][pos.1] + 1;
            }
        }
    }

    dist[end_pos.0][end_pos.1]
}
