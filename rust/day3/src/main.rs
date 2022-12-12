// use std::{collections::HashSet, fs::File, io::Read};

// fn main() {
//     // read from example.txt
//     // let mut file = File::open("example.txt").expect("File not found");
//     let mut file = File::open("input.txt").expect("File not found");

//     let mut contents = String::new();

//     // read the file into a string
//     file.read_to_string(&mut contents)
//         .expect("Something went wrong reading the file");

//     // initialize a vector
//     let mut vec: Vec<String> = Vec::new();

//     for line in contents.lines() {
//         // print the line
//         println!("{}", line);

//         // get the length of the string
//         let str_len = line.chars().count();
//         // slice the str by half
//         let half = str_len / 2;
//         // get the first half of the string
//         let first_half = &line[..half];
//         // get the second half of the string
//         let second_half = &line[half..];

//         // create a hashset to store the characters
//         let mut first_half_chars: HashSet<char> = HashSet::new();
//         let mut second_half_chars: HashSet<char> = HashSet::new();

//         // check if any char in the first half is in the second half by using the intersection method
//         for c in first_half.chars() {
//             first_half_chars.insert(c);
//         }
//         for c in second_half.chars() {
//             second_half_chars.insert(c);
//         }

//         let intersect = first_half_chars.intersection(&second_half_chars);

//         // print the result
//         println!("{} {}", first_half, second_half);
//         println!("Intersection: {:?}", intersect);

//         // add the first char from intersect to vec
//         for c in intersect {
//             vec.push(c.to_string());
//         }
//     }

//     // print the vector
//     println!("{:?}", vec);

//     // transform each char from vec to ascii using map and collect
//     // if the char is between a and z, assign it to 1 through 26
//     // if the char is between A and Z, assign it to 27 through 52
//     // if the char is not a letter, assign it to 0
//     let ascii_vec: Vec<i32> = vec
//         .iter()
//         .map(|c| match c.chars().next().unwrap() {
//             'a'..='z' => c.chars().next().unwrap() as i32 - 96,
//             'A'..='Z' => c.chars().next().unwrap() as i32 - 38,
//             _ => 0,
//         })
//         .collect();

//     // print the ascii vector
//     println!("{:?}", ascii_vec);

//     // sum the ascii vector
//     let sum: i32 = ascii_vec.iter().sum();

//     // print the sum
//     println!("{}", sum);
// }

use std::{collections::HashSet, fs::File, io::Read};

fn map_string_to_scores(c: &String) -> Vec<i32> {
    c.chars()
        .into_iter()
        .map(|c| match c {
            'a'..='z' => c as i32 - 96,
            'A'..='Z' => c as i32 - 38,
            _ => 0,
        })
        .collect()
}

fn main() {
    // read from example.txt
    // let mut file = File::open("example.txt").expect("File not found");
    let mut file = File::open("input.txt").expect("File not found");

    let mut contents = String::new();

    // read the file into a string
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    // initialize a vector that will contain tuples of 3 strings
    let mut vec: Vec<(String, String, String)> = Vec::new();

    let mut line_iter = contents.lines().into_iter();

    while let Some(line1) = line_iter.next() {
        let line2 = line_iter.next().unwrap();
        let line3 = line_iter.next().unwrap();

        vec.push((line1.to_string(), line2.to_string(), line3.to_string()));
    }

    // for each group, take the intersect of the three lines
    let mut intersect_vec: Vec<String> = Vec::new();
    for (line1, line2, line3) in vec {
        // create a hashset to store the characters
        let first_half_chars: HashSet<char> = line1.chars().collect();
        let second_half_chars: HashSet<char> = line2.chars().collect();
        let third_half_chars: HashSet<char> = line3.chars().collect();

        let intersect: HashSet<char> = first_half_chars
            .intersection(&second_half_chars)
            .cloned()
            .collect::<HashSet<char>>()
            .intersection(&third_half_chars)
            .cloned()
            .collect();

        // print the result
        println!("{} {} {}", line1, line2, line3);
        println!("Intersection: {:?}", intersect);

        // add the first char from intersect to vec
        for c in intersect {
            intersect_vec.push(c.to_string());
        }
    }

    // print the vector
    println!("{:?}", intersect_vec);

    // transform each char from vec to ascii using map and collect
    // if the char is between a and z, assign it to 1 through 26
    // if the char is between A and Z, assign it to 27 through 52
    // if the char is not a letter, assign it to 0
    let ascii_vec: Vec<i32> = intersect_vec
        .iter()
        .map(map_string_to_scores)
        .flatten()
        .collect();

    // print the ascii vector
    println!("{:?}", ascii_vec);

    // sum the ascii vector
    let sum: i32 = ascii_vec.iter().sum();

    // print the sum
    println!("{}", sum);
}
