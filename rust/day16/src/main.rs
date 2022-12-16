use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

type Pump = String;
type Flow = i32;

type FlowMap = HashMap<Pump, Flow>;

type DistanceMap = HashMap<(Pump, Pump), i32>;

fn main() {
    let (flow_map, tunnels) = parse("input.txt");

    let pos = "AA".to_string();
    let time_bound = 30;
    let cur_time = 0;

    let distances = reduce_graph(&flow_map, &tunnels);

    let relevant_valves: HashSet<&String> = flow_map
        .iter()
        .filter(|(_, f)| **f > 0)
        .map(|(v, _)| v)
        .collect();

    println!("Distances calculated.");

    let open_pumps = HashSet::new();

    let max_flow = explore(
        pos,
        0,
        0,
        cur_time,
        time_bound,
        &open_pumps,
        &relevant_valves,
        &flow_map,
        &distances,
    );

    println!("Max flow: {}", max_flow);
}

#[test]
fn example() {
    let (flow_map, tunnels) = parse("ex.txt");

    let pos = "AA".to_string();
    let time_bound = 30;
    let cur_time = 0;

    let distances = reduce_graph(&flow_map, &tunnels);
    println!("Distances calculated.");

    let relevant_valves: HashSet<&String> = flow_map
        .iter()
        .filter(|(_, f)| **f > 0)
        .map(|(v, _)| v)
        .collect();

    let open_pumps = HashSet::new();

    let max_flow = explore(
        pos,
        0,
        0,
        cur_time,
        time_bound,
        &open_pumps,
        &relevant_valves,
        &flow_map,
        &distances,
    );

    println!("Max flow: {}", max_flow);
}

fn explore(
    pos: String,
    cur_flow_per_minute: i32,
    total_flow: i32,
    minute: i32,
    time_bound: i32,
    open_pumps: &HashSet<&Pump>,
    relevant_valves: &HashSet<&Pump>,
    flow_map: &FlowMap,
    distances: &DistanceMap,
) -> i32 {
    // There is no time left, return the current flow
    if minute >= time_bound {
        assert!(minute == time_bound);
        return total_flow;
    }

    // There are no more relevant valves, return the current flow times the time left
    let unopened_valves: HashSet<&String> = relevant_valves - open_pumps;
    if unopened_valves.is_empty() {
        return total_flow + cur_flow_per_minute * (time_bound - minute);
    }

    let mut best_flow = 0;

    // Try moving to valve and opening it
    // Consume the time it takes to get there
    // Explore recursively from there
    for valve in unopened_valves {
        let cost = distances[&(pos.clone(), valve.to_string())];

        let mut new_open_pumps = open_pumps.clone();
        new_open_pumps.insert(valve);

        let val;
        if minute + cost <= time_bound {
            val = explore(
                valve.to_string(),
                cur_flow_per_minute + flow_map[valve],
                total_flow + cur_flow_per_minute * cost,
                minute + cost,
                time_bound,
                &new_open_pumps,
                relevant_valves,
                flow_map,
                distances,
            );
        } else {
            val = total_flow + cur_flow_per_minute * (time_bound - minute);
        }

        if val > best_flow {
            best_flow = val;
        }
    }

    best_flow
}

fn reduce_graph(flows: &FlowMap, tunnels: &HashMap<Pump, Vec<Pump>>) -> DistanceMap {
    // Reduce the graph to a single node for each flow
    // This is done by finding the shortest path between each pair of nodes
    // and summing the flows along the path
    let mut reduced_graph = HashMap::new();

    for a in flows.keys() {
        for b in flows.keys() {
            if a == b {
                continue;
            }

            let path = shortest_path_between(a, b, tunnels);
            reduced_graph.insert((a.to_string(), b.to_string()), path.len() as i32);
        }
    }

    reduced_graph
}

fn parse(filename: &str) -> (FlowMap, HashMap<Pump, Vec<Pump>>) {
    let mut all_flows = FlowMap::new();
    let mut all_tunnels = HashMap::<String, Vec<String>>::new();

    let file = File::open(filename).unwrap();

    // Examples:
    // "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"
    // "Valve BB has flow rate=13; tunnels lead to valves CC, AA"

    let re = Regex::new(
        r"Valve (\w{2}) has flow rate=(\d+); tunnels? leads? to valves? (\w{2}[, \w{2}]*)",
    )
    .unwrap();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let caps = re.captures(&line).unwrap();
        let name = caps.get(1).unwrap().as_str();
        let flow = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let tunnels = caps
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        println!("Name: {}, Flow: {}, Tunnels: {:?}", name, flow, tunnels);

        all_flows.insert(name.to_string(), flow);
        all_tunnels.insert(name.to_string(), tunnels);
    }

    (all_flows, all_tunnels)
}

fn shortest_path_between(
    pos_a: &str,
    pos_b: &str,
    tunnels: &HashMap<Pump, Vec<Pump>>,
) -> Vec<String> {
    // Do a BFS to find the shortest path between two points
    let mut queue: VecDeque<Vec<String>> = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();
    queue.push_back(vec![pos_a.to_string()]);
    visited.insert(pos_a.to_string());

    while !queue.is_empty() {
        let path = queue.pop_front().unwrap();
        let last_pos = path[path.len() - 1].clone();

        if last_pos == pos_b {
            return path;
        }

        for tunnel in &tunnels[&last_pos] {
            if !visited.contains(tunnel) {
                let mut new_path = path.clone();
                new_path.push(tunnel.clone());
                queue.push_back(new_path);
                visited.insert(tunnel.clone());
            }
        }
    }

    // If we get here, there is no path between the two points
    panic!("No path between {} and {}", pos_a, pos_b);
}

#[test]
fn test_parse() {
    let (all_flows, all_tunnels) = parse("ex.txt");
    assert_eq!(all_flows.len(), 10);
    assert_eq!(all_tunnels.len(), 10);
}
