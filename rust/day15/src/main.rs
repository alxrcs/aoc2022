use regex::Regex;
use std::{
    cmp::{max, min},
    collections::{HashSet, VecDeque},
    fs::File,
    io::Read,
};

#[derive(Debug)]
struct Reading {
    sensor_pos: (i32, i32),
    closest_beacon_pos: (i32, i32),
}

fn main() {
    // let (input, bound) = (read_from_file("ex.txt"), 20);
    let (input, bound) = (read_from_file("input.txt"), 4000000);
    let x_range = (0, bound);

    let lines = input.lines();

    let regex: Regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let parsed = lines
        .map(|line| parse_line(line, &regex))
        .collect::<Vec<_>>();

    for y in 0..=bound {
        let mut segments: HashSet<(i32, i32)> = HashSet::new();
        for reading in &parsed {
            calc_cover_for_row(reading, y, &mut segments);
        }

        let beacons: HashSet<(i32, i32)> = parsed.iter().map(|r| r.closest_beacon_pos).collect();
        let sensors: HashSet<(i32, i32)> = parsed.iter().map(|r| r.sensor_pos).collect();

        let mut segments: Vec<(i32, i32)> = segments.into_iter().collect();
        merge_overlapping_segments(&mut segments);

        // println!("Segments: {:?}", segments);

        let count = count_positions_with_no_beacon_or_sensor_and_in_range(
            &segments, &beacons, &sensors, y, x_range,
        );
        // println!("Positions ({}): {:?}", y, count);

        println!("\ry: {}", y);

        if count == x_range.1 - x_range.0 {
            println!("Segments: {:?}", segments);
            println!("Found y?: {}", y);

            break;
        }
    }
}

#[test]
fn calc_part_2() {
    // Segments: [(-1184066, 3138880), (3138882, 4497514)]
    let y: i128 = 3364986;
    let x: i128 = 3138881;

    println!("Part 2: {}", x * 4000000 + y)
}

#[test]
fn ex_part1() {
    let input: String = read_from_file("ex.txt");
    let y = 10;
    let lines = input.lines();

    let regex: Regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let parsed = lines
        .map(|line| parse_line(line, &regex))
        .collect::<Vec<_>>();

    let mut segments: HashSet<(i32, i32)> = HashSet::new();
    for reading in &parsed {
        calc_cover_for_row(reading, y, &mut segments);
    }

    let beacons: HashSet<(i32, i32)> = parsed.iter().map(|r| r.closest_beacon_pos).collect();
    let sensors: HashSet<(i32, i32)> = parsed.iter().map(|r| r.sensor_pos).collect();

    let mut segments: Vec<(i32, i32)> = segments.into_iter().collect();
    merge_overlapping_segments(&mut segments);

    println!("Segments: {:?}", segments);

    let count = count_positions_with_no_beacon_or_sensor(&segments, &beacons, &sensors, y);
    println!("Positions: {:?}", count);
}

fn merge_overlapping_segments(segments: &mut Vec<(i32, i32)>) {
    let mut merged: Vec<(i32, i32)> = Vec::new();

    segments.sort_by(|(a1, _a2), (b1, _b2)| a1.cmp(b1));
    let mut q = VecDeque::from(segments.clone());

    let (mut a, mut b) = segments[0];

    while let Some(next) = q.pop_front() {
        let (c, d) = next;

        if c <= b && b <= d {
            b = d;
        } else if d <= b {
            // Do nothing, it's contained
        } else {
            merged.push((a, b));
            a = c;
            b = d;
        }

        if q.is_empty() {
            merged.push((a, b));
        }
    }

    segments.clear();
    segments.extend(merged);
}

#[test]
fn test_merge_overlapping_segments() {
    let mut segments: Vec<(i32, i32)> = vec![(1, 3), (2, 4), (5, 6), (7, 8), (7, 10), (8, 9)];
    merge_overlapping_segments(&mut segments);
    assert_eq!(segments, vec![(1, 4), (5, 6), (7, 10)]);
}

fn count_positions_with_no_beacon_or_sensor(
    segments: &Vec<(i32, i32)>,
    known_beacons: &HashSet<(i32, i32)>,
    sensors: &HashSet<(i32, i32)>,
    y: i32,
) -> i32 {
    let beacons: Vec<&(i32, i32)> = known_beacons.iter().filter(|(_, y2)| *y2 == y).collect();
    let sensors: Vec<&(i32, i32)> = sensors.iter().filter(|(_, y2)| *y2 == y).collect();

    // calc total length of intervals
    let mut total_count = 0;
    for (x1, x2) in segments {
        total_count += x2 - x1 + 1;
    }

    // for each beacon, remove the interval from the total length
    for seg in segments {
        for beacon in beacons.iter() {
            let (x1, x2) = seg;
            let (bx, _) = beacon;

            if bx >= x1 && bx <= x2 {
                total_count -= 1;
            }
        }
        for sensor in sensors.iter() {
            let (x1, x2) = seg;
            let (bx, _) = sensor;

            if bx >= x1 && bx <= x2 {
                total_count -= 1;
            }
        }
    }

    total_count
}

fn count_positions_with_no_beacon_or_sensor_and_in_range(
    segments: &Vec<(i32, i32)>,
    known_beacons: &HashSet<(i32, i32)>,
    sensors: &HashSet<(i32, i32)>,
    y: i32,
    x_range: (i32, i32),
) -> i32 {
    let beacons: Vec<&(i32, i32)> = known_beacons.iter().filter(|(_, y2)| *y2 == y).collect();
    let sensors: Vec<&(i32, i32)> = sensors.iter().filter(|(_, y2)| *y2 == y).collect();

    // calc total length of intervals
    let mut total_count = 0;
    for (x1, x2) in segments {
        let x1 = max(*x1, x_range.0);
        let x2 = min(*x2, x_range.1);
        total_count += x2 - x1 + 1;
    }

    // for each beacon, remove the interval from the total length
    // for seg in segments {
    //     for beacon in beacons.iter() {
    //         let (x1, x2) = seg;
    //         let (bx, _) = beacon;

    //         if bx >= x1 && bx <= x2 {
    //             total_count -= 1;
    //         }
    //     }
    //     for sensor in sensors.iter() {
    //         let (x1, x2) = seg;
    //         let (bx, _) = sensor;

    //         if bx >= x1 && bx <= x2 {
    //             total_count -= 1;
    //         }
    //     }
    // }

    total_count
}

#[test]
fn test_calc_cover_for_row() {
    let ys = vec![10, 15, 16];
    let expecteds: Vec<usize> = vec![13, 3, 1];

    for (y, expected) in ys.iter().zip(expecteds.iter()) {
        let mut marks: HashSet<(i32, i32)> = HashSet::new();

        calc_cover_for_row(
            &Reading {
                sensor_pos: (8, 7),
                closest_beacon_pos: (2, 10),
            },
            *y,
            &mut marks,
        );

        assert_eq!(marks.len(), *expected);
    }
}

#[test]
fn test_first_try() {
    // This unfortunately times out
    // let mut visited = HashMap::new();
    // for reading in &parsed {
    //     let dist = manhattan_distance(reading.sensor_pos, reading.closest_beacon_pos);
    //     bfs_up_to_n_steps(reading.sensor_pos, dist, &mut visited);
    // }

    // println!("Visited: {:?}", visited);

    // // count the number of visited nodes where the y value is equal to m
    // let count = visited.iter().filter(|((_, y), _)| *y == 2000000).count();
    // println!("Count: {}", count);
}

fn calc_cover_for_row(reading: &Reading, y: i32, segments: &mut HashSet<(i32, i32)>) {
    let (x1, y1) = reading.sensor_pos;

    let dist = manhattan_distance(reading.sensor_pos, reading.closest_beacon_pos);

    // If row is out of range, do nothing
    if y > y1 + dist || y < y1 - dist {
        return;
    }

    let y_from_s_dist = (y - y1).abs();
    let extra_dist = dist - y_from_s_dist;

    let (n_lo, n_up) = (x1 - extra_dist, x1 + extra_dist);

    segments.insert((n_lo, n_up));
}

// fn bfs_up_to_n_steps(start: (i32, i32), n: i32, visited: &mut HashMap<(i32, i32), i32>) -> bool {
//     if n == 0 {
//         return false;
//     }

//     let mut queue = Vec::new();
//     queue.push(start);

//     while !queue.is_empty() {
//         let current = queue.pop().unwrap();

//         let neighbors = get_neighbors(current);
//         for neighbor in neighbors {
//             if !visited.contains_key(&neighbor) {
//                 let distance_from_origin = manhattan_distance(start, neighbor);
//                 if distance_from_origin > n {
//                     continue;
//                 } else {
//                     queue.push(neighbor);
//                     visited.insert(neighbor, distance_from_origin);
//                 }
//             }
//         }
//     }

//     false
// }

// fn get_neighbors(current: (i32, i32)) -> Vec<(i32, i32)> {
//     let (x, y) = current;
//     vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
// }

fn manhattan_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn parse_line(line: &str, regex: &Regex) -> Reading {
    // Example line format:
    // "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
    // "Sensor at x=9, y=16: closest beacon is at x=10, y=16"

    let caps = regex
        .captures(line)
        .unwrap_or_else(|| panic!("Line '{}' does not match the expected format", line));

    // Extract the values you need from the captures
    let (x1, y1) = (
        caps[1].parse::<i32>().unwrap(),
        caps[2].parse::<i32>().unwrap(),
    );
    let (x2, y2) = (
        caps[3].parse::<i32>().unwrap(),
        caps[4].parse::<i32>().unwrap(),
    );

    Reading {
        sensor_pos: (x1, y1),
        closest_beacon_pos: (x2, y2),
    }
}

fn read_from_file(arg: &str) -> String {
    let mut file = File::open(arg).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
