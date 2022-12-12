use std::fs::read_to_string;

#[derive(Debug)]
struct MonkeyState<'a> {
    starting_items: Box<Vec<i64>>,
    operation: &'a str,
    test_divide_by: i64,
    if_true_thr_to: usize,
    if_false_thr_to: usize,
}

fn eval_op(op: &str, old: i64) -> i64 {
    let new: i64;

    // op is a string in the form <term> <operator> <term>
    // term can also be `old`, which is the value passed as parameter
    // operator can be +, -, *, /
    // term can also be a number
    // example: "old * 19"
    // example: "old + 6"
    // example: "old * old"
    // example: "old + 3"

    // Parse the operation
    let op_parts: Vec<&str> = op.split(" ").collect();

    // Get the first term
    let term1: i64 = match op_parts[0] {
        "old" => old,
        _ => op_parts[0].parse().unwrap(),
    };

    // Get the operator
    let operator: &str = op_parts[1];

    // Get the second term
    let term2: i64 = match op_parts[2] {
        "old" => old,
        _ => op_parts[2].parse().unwrap(),
    };

    // Evaluate the operation
    match operator {
        "+" => new = term1 + term2,
        "-" => new = term1 - term2,
        "*" => new = term1 * term2,
        "/" => new = term1 / term2,
        _ => panic!("Invalid operator"),
    }

    return new;
}

fn main() {
    let example: String = read_to_string("input.txt").unwrap();

    let mut monkeys: Vec<Box<MonkeyState>> = Vec::new();

    // Parse the example
    let lines: Vec<&str> = example.split("\n").collect();
    let mut i: usize = 0;

    while i < lines.len() {
        let starting_items = Box::new(Vec::new());

        let mut monkey = MonkeyState {
            starting_items,
            operation: "",
            test_divide_by: 0,
            if_true_thr_to: 0,
            if_false_thr_to: 0,
        };

        // Parse the starting items
        let starting_items: Vec<&str> = lines[i + 1].split(": ").collect();
        let starting_items: Vec<&str> = starting_items[1].split(", ").collect();
        for item in starting_items {
            monkey.starting_items.push(item.parse().unwrap());
        }

        // Parse the operation
        let operation: Vec<&str> = lines[i + 2].split("new = ").collect();
        monkey.operation = operation[1].trim();

        // Parse the test
        let test: Vec<&str> = lines[i + 3].split("divisible by ").collect();
        monkey.test_divide_by = test[1].trim().parse().unwrap();

        // Parse the if_true
        let if_true: Vec<&str> = lines[i + 4].split("throw to monkey ").collect();
        monkey.if_true_thr_to = if_true[1].trim().parse().unwrap();

        // Parse the if_false
        let if_false: Vec<&str> = lines[i + 5].split("throw to monkey ").collect();
        monkey.if_false_thr_to = if_false[1].trim().parse().unwrap();

        // Add the monkey to the monkeys vector
        monkeys.push(Box::new(monkey));

        i += 7;
    }

    // print the parsed example
    for monkey in &monkeys {
        println!("Starting items: {:?}", monkey.starting_items);
        println!("Operation: {}", monkey.operation);
        println!("Test: divisible by {}", monkey.test_divide_by);
        println!("If true: throw to monkey {}", monkey.if_true_thr_to);
        println!("If false: throw to monkey {}", monkey.if_false_thr_to);
        println!("");
    }

    // Simulate 20 rounds
    // On each round, each monkey inspects its items
    // If the item is divisible by the test, it is thrown to the if_true monkey
    // If the item is not divisible by the test, it is thrown to the if_false monkey
    // The item is also replaced by the result of the operation

    let num_monkeys = monkeys.len();
    let mut monkey_inspections_counts = vec![0; num_monkeys];

    let common_divider: i64 = monkeys.iter().map(|monkey| monkey.test_divide_by).product();

    for round in 0..10000 {
        for i in 0..num_monkeys {
            // println!("Monkey {}: ", i);
            let monkey = monkeys.get_mut(i).unwrap();
            let mut new_items: Vec<(usize, i64)> = Vec::new();

            for item in &*monkey.starting_items {
                // println!(" Monkey inspects an item with a worry level of {}", item);
                monkey_inspections_counts[i] += 1;
                let new_item_worry_level = eval_op(monkey.operation, *item);
                // println!(
                //     "  Monkey performs {} and result is {}",
                //     monkey.operation, new_item_worry_level
                // );

                // new_item_worry_level /= 3;

                // println!(
                //     "  Monkey gets bored with item. Worry level is divided by 3 to {}",
                //     new_item_worry_level
                // );

                // println!(
                //     "  Monkey checks for divisibility by {}. If true, throw to monkey {}. If false, throw to monkey {}",
                //     monkey.test_divide_by, monkey.if_true_thr_to, monkey.if_false_thr_to
                // );

                let target_monkey_idx = match new_item_worry_level % monkey.test_divide_by {
                    0 => monkey.if_true_thr_to,
                    _ => monkey.if_false_thr_to,
                };

                // println!(
                //     "  Item with a worry level of {} is thrown to monkey {}",
                //     new_item_worry_level, target_monkey_idx
                // );

                new_items.push((target_monkey_idx, new_item_worry_level % common_divider));
            }

            monkey.starting_items.clear();
            for item in new_items {
                monkeys.get_mut(item.0).unwrap().starting_items.push(item.1);
            }
        }

        // println!("Round {}:", round + 1);
        // for (i, monkey) in monkeys.iter().enumerate() {
        //     println!("Monkey {} has {:?}", i, monkey.starting_items);
        // }
    }

    println!("Monkey inspections count: {:?}", monkey_inspections_counts);

    monkey_inspections_counts
        .iter()
        .enumerate()
        .for_each(|(i, count)| {
            println!("Monkey {} inspected {} items", i, count);
        });

    let mut counts_with_index: Vec<(usize, &i64)> =
        monkey_inspections_counts.iter().enumerate().collect();

    counts_with_index.sort_by(|a, b| a.1.cmp(b.1));

    let monkey_business = counts_with_index
        .iter()
        .rev()
        .take(2)
        .map(|x| x.1)
        .fold(1, |acc, x| acc * x);

    println!("Monkey business: {}", monkey_business);
}
