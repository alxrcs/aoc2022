use serde_json::Value::{Array, Number};
use serde_json::{from_str, to_string, Value};
use std::cmp::Ordering;
use std::fs::read_to_string;

fn main() {
    // load example.txt
    let txt = read_to_string("input.txt").unwrap();

    let mut lines_it = txt.lines();
    let mut orders: Vec<Ordering> = vec![];

    while let Some(line1) = lines_it.next() {
        let line2 = lines_it.next().unwrap();

        println!("Line 1: {}", line1);
        println!("Line 2: {}", line2);

        let line1: Value = from_str(&line1).unwrap();
        let line2: Value = from_str(&line2).unwrap();

        let order = are_in_right_order(&line1, &line2);
        orders.push(order);

        println!("Order: {:?}\n", order);

        // skip empty line
        lines_it.next();
    }

    println!("Orders: {:?}", orders);

    // sum the indices of the -1s
    let sum: i32 = orders
        .iter()
        .enumerate()
        .map(|(i, &x)| (i + 1, x))
        .filter(|(_, x)| *x == Ordering::Less)
        .map(|(i, _)| i as i32)
        .sum();

    println!("Sum: {}", sum);

    let mut all_lines: Vec<Value> = txt
        .lines()
        .filter(|line| !line.is_empty())
        .map(|l| from_str(l).unwrap())
        .collect();

    // print all lines
    for line in &all_lines {
        println!("{:?}", to_string(line).unwrap());
    }

    // add some additional packets
    let additional_packets = ["[[2]]", "[[6]]"];
    for packet in additional_packets {
        let value: Value = from_str(packet).unwrap();
        all_lines.push(value)
    }

    let packet_i = all_lines.len() - 1;
    let packet_j = all_lines.len();

    let mut numbered_lines = all_lines
        .iter()
        .enumerate()
        .map(|(i, x)| (i + 1, x))
        .collect::<Vec<_>>();

    numbered_lines.sort_by(|(_, v1), (_, v2)| are_in_right_order(v1, v2));

    // print all lines
    for (i, val) in numbered_lines.clone() {
        println!("{}:{:?}", i, to_string(val).unwrap());
    }

    // search for packet_i and packet_j in the sorted list
    let mut i = 0;
    let mut j = 0;
    for (new_index, (index, _)) in numbered_lines.iter().enumerate() {
        if *index == packet_i {
            i = new_index + 1;
        }
        if *index == packet_j {
            j = new_index + 1;
        }
    }

    // println!("All lines sorted: {:#?}", all_lines);
    println!("Packet i: {}", i);
    println!("Packet j: {}", j);
    println!("Product: {}", i * j);
}

fn are_in_right_order(t1: &Value, t2: &Value) -> Ordering {
    match (t1, t2) {
        (Number(a), Number(b)) => {
            if a.as_i64() < b.as_i64() {
                Ordering::Less
            } else if a.as_i64() > b.as_i64() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
        (Number(n), Array(a)) => {
            // Wrap the number in an array
            are_in_right_order(
                &Array(vec![Value::Number(n.to_owned().into())]),
                &Array(a.to_owned()),
            )
        }

        (Array(a), Number(n)) => are_in_right_order(
            &Array(a.to_owned()),
            &Array(vec![Value::Number(n.to_owned().into())]),
        ),
        (Array(a1), Array(a2)) => {
            let mut a1_it = a1.iter();
            let mut a2_it = a2.iter();

            loop {
                match (a1_it.next(), a2_it.next()) {
                    (Some(v1), Some(v2)) => {
                        let res = are_in_right_order(v1, v2);
                        if res != Ordering::Equal {
                            return res;
                        }
                    }
                    (Some(_), None) => return Ordering::Greater,
                    (None, Some(_)) => return Ordering::Less,
                    (None, None) => return Ordering::Equal,
                }
            }

            // If the left list runs out of items first return -1
            // If the right list runs out of items first return 1
            // If both lists run out of items at the same time return 0
        }
        _ => panic!("Unexpected value"),
    }
}
