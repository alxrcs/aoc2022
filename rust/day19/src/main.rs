use rayon::prelude::*;
use regex::Regex;

const TIME_LIMIT: u32 = 32;

fn main() {
    let input = include_str!("../input.txt");
    let blueprints = parse(input.to_string());
    let initial_state = State::new();

    let mut results: Vec<u32> = vec![];

    for (i, blueprint) in blueprints.iter().take(3).enumerate() {
        println!("Blueprint {}", i + 1);
        // let geodes = max_geodes(&initial_state, &blueprint, 0, &mut HashMap::new());
        let geodes = max_geodes(&initial_state, blueprint, 0);

        println!("Geodes: {}", geodes);
        let quality = (i + 1) as u32 * geodes;
        println!("Quality for blueprint {}: {}", i + 1, quality);
        results.push(quality);
    }

    println!(
        "Total quality product: {}",
        results[0] * results[1] * results[2]
    );
}

#[test]
fn part_1() {
    let input = include_str!("../input.txt");

    let blueprints = parse(input.to_string());

    let initial_state = State::new();

    let mut total_quality = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        println!("Blueprint {}", i + 1);
        let geodes = max_geodes(&initial_state, blueprint, 0);

        println!("Geodes: {}", geodes);
        let quality = (i + 1) as u32 * geodes;
        println!("Quality for blueprint {}: {}", i + 1, quality);
        total_quality += quality;
    }

    println!("Total quality: {}", total_quality);
    assert_eq!(total_quality, 1389);
}

#[test]
fn example() {
    let input = include_str!("../ex.txt");

    let blueprints = parse(input.to_string());

    let initial_state = State::new();

    let mut total_quality = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        println!("Blueprint {}", i + 1);
        // let geodes = max_geodes(&initial_state, &blueprint, 0, &mut HashMap::new());
        let geodes = max_geodes(&initial_state, blueprint, 0);

        println!("Geodes: {}", geodes);
        let quality = (i + 1) as u32 * geodes;
        println!("Quality for blueprint {}: {}", i + 1, quality);
        total_quality += quality;
        // break; // TODO: Remove this line to run all blueprints
    }

    println!("Total quality: {}", total_quality);
    assert_eq!(total_quality, 33);
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct State {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,

    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

impl State {
    fn new() -> Self {
        State {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,

            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
}

fn max_geodes(state: &State, blueprint: &Blueprint, minute: u32) -> u32 {
    if minute == TIME_LIMIT {
        return state.geode;
    }

    // TODO: Prune the tree with an optimistic estimation of the max geodes

    possible_actions(&state, &blueprint)
        .par_iter()
        .map(|act| (try_building(state, act, &blueprint), act))
        .map(|(ns, act)| (produce(&ns), act))
        .map(|(ns, act)| finish_building_robot(&ns, act))
        .map(|ns| max_geodes(&ns.clone(), &blueprint, minute + 1))
        .max()
        .unwrap()
}

fn produce(s: &State) -> State {
    // Pass time first
    // Each robot produces 1 ore, 1 clay, 1 obsidian, or 1 geode per minute.
    State {
        ore: s.ore + s.ore_robots,
        clay: s.clay + s.clay_robots,
        obsidian: s.obsidian + s.obsidian_robots,
        geode: s.geode + s.geode_robots,
        ..*s
    }
}

fn try_building(s: &State, action: &Action, blueprint: &Blueprint) -> State {
    match action {
        Action::MakeOreBot => State {
            ore: s.ore - blueprint.ore_bot_ore_cost,
            ..*s
        },
        Action::MakeClayBot => State {
            ore: s.ore - blueprint.clay_bot_ore_cost,
            ..*s
        },
        Action::MakeObsidianBot => State {
            ore: s.ore - blueprint.obsidian_bot_ore_cost,
            clay: s.clay - blueprint.obsidian_robot_clay_cost,
            ..*s
        },
        Action::MakeGeodeBot => State {
            ore: s.ore - blueprint.geode_bot_ore_cost,
            obsidian: s.obsidian - blueprint.geode_robot_obsidian_cost,
            ..*s
        },
        Action::Wait => s.clone(),
    }
}

fn finish_building_robot(s: &State, action: &Action) -> State {
    match action {
        Action::MakeOreBot => State {
            ore_robots: s.ore_robots + 1,
            ..*s
        },
        Action::MakeClayBot => State {
            clay_robots: s.clay_robots + 1,
            ..*s
        },
        Action::MakeObsidianBot => State {
            obsidian_robots: s.obsidian_robots + 1,
            ..*s
        },
        Action::MakeGeodeBot => State {
            geode_robots: s.geode_robots + 1,
            ..*s
        },
        Action::Wait => s.clone(),
    }
}

fn possible_actions(s: &State, bp: &Blueprint) -> Vec<Action> {
    // Given the current state of the factory, determine
    // what actions are possible.

    let mut actions: Vec<Action> = Vec::new();

    let max_ore_cost = bp
        .ore_bot_ore_cost
        .max(bp.clay_bot_ore_cost)
        .max(bp.obsidian_bot_ore_cost)
        .max(bp.geode_bot_ore_cost);
    let max_clay_cost = bp.clay_bot_ore_cost.max(bp.obsidian_robot_clay_cost);

    if s.ore >= bp.geode_bot_ore_cost && s.obsidian >= bp.geode_robot_obsidian_cost {
        actions.push(Action::MakeGeodeBot);
        return actions; // prioritize always making a geode robot
    }

    if s.ore >= bp.obsidian_bot_ore_cost
        && s.clay >= bp.obsidian_robot_clay_cost
        && s.obsidian_robots < bp.geode_robot_obsidian_cost
    {
        actions.push(Action::MakeObsidianBot);
    }
    if s.ore >= bp.ore_bot_ore_cost && s.ore_robots < max_ore_cost {
        actions.push(Action::MakeOreBot);
    }

    if s.ore >= bp.clay_bot_ore_cost // If we have enough ore to make a clay robot
    && s.clay_robots < max_clay_cost
    // and we don't have too many clay robots
    {
        actions.push(Action::MakeClayBot);
    }

    if s.ore < max_ore_cost
        || (s.clay < bp.obsidian_robot_clay_cost && s.clay_robots > 0)
        || (s.obsidian < bp.geode_robot_obsidian_cost && s.obsidian_robots > 0)
    {
        actions.push(Action::Wait);
    }

    if actions.len() == 0 {
        actions.push(Action::Wait);
    }

    actions
}

enum Action {
    MakeOreBot,
    MakeClayBot,
    MakeObsidianBot,
    MakeGeodeBot,
    Wait,
}

fn parse(filename: String) -> Vec<Blueprint> {
    // Parse the file and return a vector of Blueprints.

    // Format examples:
    // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    // Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay.  Each geode robot costs 3 ore and 12 obsidian.

    let r: Regex = Regex::new(r"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

    let mut blueprints: Vec<Blueprint> = Vec::new();

    for line in filename.lines() {
        let caps = r.captures(line).unwrap();
        blueprints.push(Blueprint {
            ore_bot_ore_cost: caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            clay_bot_ore_cost: caps.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            obsidian_bot_ore_cost: caps.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            obsidian_robot_clay_cost: caps.get(4).unwrap().as_str().parse::<u32>().unwrap(),
            geode_bot_ore_cost: caps.get(5).unwrap().as_str().parse::<u32>().unwrap(),
            geode_robot_obsidian_cost: caps.get(6).unwrap().as_str().parse::<u32>().unwrap(),
        });
    }
    blueprints
}
struct Blueprint {
    ore_bot_ore_cost: u32,
    clay_bot_ore_cost: u32,
    obsidian_bot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_bot_ore_cost: u32,
    geode_robot_obsidian_cost: u32,
}
