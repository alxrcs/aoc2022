use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct P3D {
    x: i32,
    y: i32,
    z: i32,
}

const LOWER: i32 = -1;
const UPPER: i32 = 22;

impl P3D {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn adjacent(p: P3D) -> Vec<P3D> {
        let mut adj = Vec::new();
        for x in -1..2i32 {
            for y in -1..2i32 {
                for z in -1..2i32 {
                    if (x == 0 && y == 0 && z == 0) || (x.abs() + y.abs() + z.abs() != 1) {
                        continue;
                    }
                    // Only account for points on [0..20]
                    if p.x + x < LOWER
                        || p.y + y < LOWER
                        || p.z + z < LOWER
                        || p.x + x > UPPER
                        || p.y + y > UPPER
                        || p.z + z > UPPER
                    {
                        continue;
                    }

                    adj.push(P3D::new(p.x + x, p.y + y, p.z + z));
                }
            }
        }
        adj
    }
}

fn main() {
    part_1(String::from("input.txt"));
    part_2(String::from("input.txt"));
}

fn reachable_points(start: P3D, blocks: &HashSet<P3D>) -> HashSet<P3D> {
    let mut queue = Vec::new();
    let mut visited = HashSet::new();

    queue.push(start);
    while !queue.is_empty() {
        let p = queue.pop().unwrap();
        visited.insert(p);
        for a in P3D::adjacent(p).iter() {
            if !blocks.contains(a) && !visited.contains(a) {
                queue.push(*a);
            }
        }
    }
    visited
}

fn part_1(filename: String) -> i32 {
    let input = read_to_string(filename).unwrap();
    let p_set: HashSet<P3D> = parse(input).iter().cloned().collect();
    part_1_inner(&p_set)
}

fn part_1_inner(points: &HashSet<P3D>) -> i32 {
    // Count covered sidesa
    // let mut count = 0;
    let mut total_count = 0;
    for p in points.iter() {
        let mut count = 0;
        let adj = P3D::adjacent(*p);
        for a in adj.iter() {
            if !points.contains(a) {
                count += 1;
            }
        }
        total_count += count;
    }

    // println!("Total covered sides: {count}");

    // let uncovered = p_set.len() * 6 - count;
    // println!("Total uncovered sides: {uncovered}");
    println!("total count: {total_count}");

    total_count
}

fn part_2(filename: String) -> i32 {
    let input = read_to_string(filename).unwrap();
    let p_set: HashSet<P3D> = parse(input).iter().cloned().collect();
    let result = part_2_inner(p_set);
    println!("Part 2: {result}");
    result
}

fn part_2_inner(points: HashSet<P3D>) -> i32 {
    // get the reachable points from (0,0,0)
    let reachable = reachable_points(P3D::new(0, 0, 0), &points);
    let all_points = (LOWER..UPPER)
        .flat_map(|x| (LOWER..UPPER).map(move |y| (x, y)))
        .flat_map(|(x, y)| (LOWER..UPPER).map(move |z| (x, y, z)))
        .map(|(x, y, z)| P3D::new(x, y, z))
        .collect::<HashSet<P3D>>();

    let unreachable: HashSet<P3D> = all_points.difference(&reachable).cloned().collect();
    let mut possibly_air: HashSet<P3D> = unreachable.difference(&points).cloned().collect();
    let total_side_area: i32 = part_1_inner(&points);

    let mut air_count = possibly_air.len();
    let mut air_side_area = 0;

    while air_count > 0 {
        let start_point = *possibly_air.iter().next().unwrap();
        let reachable_air = reachable_points(start_point, &points);
        air_side_area += part_1_inner(&reachable_air);

        possibly_air = possibly_air.difference(&reachable_air).cloned().collect();
        air_count = possibly_air.len();
    }
    total_side_area - air_side_area
}

#[test]
fn ex_small_p1() {
    assert_eq!(part_1(String::from("ex_small.txt")), 10);
}

#[test]
fn ex_p1() {
    assert_eq!(part_1(String::from("ex.txt")), 64);
}

fn parse(input: String) -> Vec<P3D> {
    let mut points = Vec::new();
    for line in input.lines() {
        let coords = line
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        points.push(P3D::new(coords[0], coords[1], coords[2]));
    }
    points
}

#[test]
fn ex_p2() {
    let result = part_2(String::from("ex.txt"));
    println!("Result: {result}");

    assert_eq!(result, 58);
}
