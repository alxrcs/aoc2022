use std::fs;

fn main() {
    // Read the input file
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    // let input = fs::read_to_string("example.txt").expect("Unable to read file");

    // Create an empty list of numbers
    let mut numbers: Vec<i32> = Vec::new();

    let mut index = 0;
    let mut curr_total = 0;
    let mut max_total = 0;
    let mut max_index = 0;

    // Loop over each line in the input
    for line in input.lines() {
        // Parse the line as an integer.
        // If the line is empty instead, add the curr_total to numbers and
        // move the index resetting curr_total
        let num = line.parse::<i32>();
        if num.is_ok() {
            curr_total += num.unwrap();
        } else {
            numbers.push(curr_total);
            index += 1;
            // update max_total if needed
            if curr_total > max_total {
                max_total = curr_total;
                max_index = index;
            }
            curr_total = 0;
        }
    }

    println!("Max total: {}", max_total);
    println!("Max index: {}", max_index);

    // sort numbers
    numbers.sort();

    // print all numbers
    for num in &numbers {
        println!("{}", num);
    }

    // print the sum of the last 3 numbers
    println!(
        "Sum of last 3: {}",
        numbers[numbers.len() - 1] + numbers[numbers.len() - 2] + numbers[numbers.len() - 3]
    );
}
