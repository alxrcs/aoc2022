use std::{collections::HashSet, fs::File, io::Read};

fn main() {
    // read from example.txt
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("File contents: {}", contents);

    let mut i: usize = 0;
    const WINDOW_LEN: usize = 14;
    // iterate through a string with a sliding window of size 4
    for window in contents.chars().collect::<Vec<_>>().windows(WINDOW_LEN) {
        let set: HashSet<char> = HashSet::from_iter(window.iter().cloned());
        if set.len() == WINDOW_LEN {
            println!("Found a window of 4 unique characters: {:?}", window);
            break;
        }
        i += 1;
    }
    println!("Window found at char: {}", i + WINDOW_LEN);
}
