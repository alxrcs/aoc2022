use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Exp<'a> {
    Num(i64),
    Plus(&'a str, &'a str),
    Minus(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

fn main() {
    let input = parse(include_str!("../input.txt"));

    let left;
    let right;

    match input.get("root").unwrap() {
        Exp::Num(_) => panic!("Root is a number"),
        Exp::Plus(l, r) => {
            left = l;
            right = r;
        }
        Exp::Minus(l, r) => {
            left = l;
            right = r;
        }
        Exp::Mul(l, r) => {
            left = l;
            right = r;
        }
        Exp::Div(l, r) => {
            left = l;
            right = r;
        }
    }

    let (ans_left, ans_right) = (eval(&input, left), eval(&input, right));
    println!("Left: {}, Right: {}", ans_left, ans_right);

    let mut invs_map = create_inverse_map_from_node_humn(&input, "humn", "root");

    let target_val = if has_node(&input, "root", "humn") {
        println!("Target is in left subtree of root");
        ans_right
    } else {
        println!("Target is in right subtree of root");
        ans_left
    };

    invs_map.insert(
        if target_val == ans_left { right } else { left },
        Exp::Num(target_val),
    );

    let new_input: HashMap<&str, Exp> = input
        .iter()
        .filter(|(k, _)| !invs_map.contains_key(*k))
        .map(|(k, v)| (*k, v.clone()))
        .collect();

    let ans = eval_with_inverse(&new_input, &invs_map, "humn");
    println!("Answer: {}", ans);
}

fn has_node<'a>(opers: &'a HashMap<&str, Exp>, src_node: &'a str, target_node: &'a str) -> bool {
    if src_node == target_node {
        return true;
    }

    match opers[src_node] {
        Exp::Num(_) => return false,
        Exp::Plus(l, r) => {
            return has_node(opers, l, target_node) || has_node(opers, r, target_node)
        }
        Exp::Minus(l, r) => {
            return has_node(opers, l, target_node) || has_node(opers, r, target_node)
        }
        Exp::Mul(l, r) => {
            return has_node(opers, l, target_node) || has_node(opers, r, target_node)
        }
        Exp::Div(l, r) => {
            return has_node(opers, l, target_node) || has_node(opers, r, target_node)
        }
    }
}

fn eval_with_inverse<'a>(
    opers: &'a HashMap<&str, Exp>,
    inv: &'a HashMap<&str, Exp>,
    src: &str,
) -> i64 {
    // Search in inv first, then in opers

    let mut exp = inv.get(src);
    if exp.is_none() {
        exp = opers.get(src);
        if exp.is_none() {
            panic!("No such node: {}", src);
        }
    }

    match exp.unwrap() {
        Exp::Num(n) => *n,
        Exp::Plus(l, r) => eval_with_inverse(&opers, &inv, l) + eval_with_inverse(&opers, &inv, r),
        Exp::Minus(l, r) => eval_with_inverse(&opers, &inv, l) - eval_with_inverse(&opers, &inv, r),
        Exp::Mul(l, r) => eval_with_inverse(&opers, &inv, l) * eval_with_inverse(&opers, &inv, r),
        Exp::Div(l, r) => eval_with_inverse(&opers, &inv, l) / eval_with_inverse(&opers, &inv, r),
    }
}

#[test]
fn example() {
    let ans = eval(&parse(include_str!("../ex.txt")), "root");
    assert_eq!(ans, 152);
}

#[test]
fn part_1() {
    let ans = eval(&parse(include_str!("../input.txt")), "root");
    assert_eq!(ans, 169525884255464);
}

fn eval(opers: &HashMap<&str, Exp>, to_string: &str) -> i64 {
    let exp = opers.get(to_string).unwrap();
    match exp {
        Exp::Num(n) => *n,
        Exp::Plus(l, r) => eval(opers, l) + eval(opers, r),
        Exp::Minus(l, r) => eval(opers, l) - eval(opers, r),
        Exp::Mul(l, r) => eval(opers, l) * eval(opers, r),
        Exp::Div(l, r) => eval(opers, l) / eval(opers, r),
    }
}

fn create_inverse_map_from_node_humn<'a>(
    opers: &'a HashMap<&str, Exp>,
    src_node: &'a str,
    target_node: &'a str,
) -> HashMap<&'a str, Exp<'a>> {
    let mut src_node = src_node;
    let mut inverse_map = HashMap::<&str, Exp>::new();

    while src_node != target_node {
        let mut found = false;
        for (p, exp) in opers {
            match exp {
                Exp::Num(_) => continue,
                Exp::Plus(l, r) => {
                    if *l == src_node {
                        inverse_map.insert(src_node, Exp::Minus(p, r));
                        println!("Since {p} = {l} + {r}, adding {} = {} - {}", src_node, p, r);
                        src_node = p;
                        found = true;
                        break;
                    } else if *r == src_node {
                        println!("Since {p} = {l} + {r}, adding {} = {} - {}", src_node, p, l);
                        inverse_map.insert(src_node, Exp::Minus(p, l));
                        src_node = p;
                        found = true;
                        break;
                    }
                }
                Exp::Minus(l, r) => {
                    if *l == src_node {
                        println!("Since {p} = {l} - {r}, adding {} = {} + {}", src_node, p, r);
                        inverse_map.insert(src_node, Exp::Plus(p, r));
                        src_node = p;
                        found = true;
                        break;
                    } else if *r == src_node {
                        println!("Since {p} = {l} - {r}, adding {} = {} - {}", src_node, l, p);
                        inverse_map.insert(src_node, Exp::Minus(l, p));
                        src_node = p;
                        found = true;
                        break;
                    }
                }
                Exp::Mul(l, r) => {
                    if *l == src_node {
                        println!("Since {p} = {l} * {r}, adding {} = {} / {}", src_node, p, r);
                        inverse_map.insert(src_node, Exp::Div(p, r));
                        src_node = p;
                        found = true;
                        break;
                    } else if *r == src_node {
                        println!("Since {p} = {l} * {r}, adding {} = {} / {}", src_node, p, l);
                        inverse_map.insert(src_node, Exp::Div(p, l));
                        src_node = p;
                        found = true;
                        break;
                    }
                }
                Exp::Div(l, r) => {
                    if *l == src_node {
                        println!("Since {p} = {l} / {r}, adding {} = {} * {}", src_node, p, r);
                        inverse_map.insert(src_node, Exp::Mul(p, r));
                        src_node = p;
                        found = true;
                        break;
                    } else if *r == src_node {
                        println!("Since {p} = {l} / {r}, adding {} = {} / {}", src_node, l, p);
                        inverse_map.insert(src_node, Exp::Div(l, p));
                        src_node = p;
                        found = true;
                        break;
                    }
                }
            }
        }
        if !found {
            panic!("Could not find node");
        }
    }
    inverse_map
}

fn parse(input: &str) -> HashMap<&str, Exp> {
    // Examples:
    // lzvm: ptgl * wvjc
    // jlbw: 5
    let mut vec = HashMap::<&str, Exp>::new();

    for line in input.lines() {
        let mut parts = line.split(": ");
        let name = parts.next().unwrap();
        let exp = parts.next().unwrap();

        if exp.contains(" ") {
            let mut parts = exp.split(" ");
            let l = parts.next().unwrap();
            let op = parts.next().unwrap();
            let r = parts.next().unwrap();

            let op = match op {
                "+" => Exp::Plus(l, r),
                "-" => Exp::Minus(l, r),
                "*" => Exp::Mul(l, r),
                "/" => Exp::Div(l, r),
                _ => panic!("Unknown operator"),
            };

            vec.insert(name, op);
        } else {
            vec.insert(name, Exp::Num(exp.parse().unwrap()));
        }
    }
    vec
}
