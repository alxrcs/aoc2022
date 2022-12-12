use std::{fs::File, io::Read};

fn main() {
    /*
    format for example.txt

    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
     */

    let mut file = File::open("input.txt").expect("File not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let mut fully_contained_count = 0;
    let mut partial_overlap = 0;

    for line in contents.lines() {
        let nums = line
            .split(',')
            .map(|x| {
                x.split('-')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .flatten()
            .collect::<Vec<u32>>();

        // unpack the four numbers in nums
        let (a, b, c, d) = (nums[0], nums[1], nums[2], nums[3]);

        // check if the a,b interval is fully contained within c, d or viceversa
        if (a >= c && b <= d) || (c >= a && d <= b) {
            println!("{}-{} is fully contained within {}-{}", a, b, c, d);
            fully_contained_count += 1;
        } else {
            println!("{}-{} is not fully contained within {}-{}", a, b, c, d);
        }

        // check for any kind of overlap
        if (a >= c && a <= d) || (b >= c && b <= d) || (c >= a && c <= b) || (d >= a && d <= b) {
            println!("{}-{} overlaps with {}-{}", a, b, c, d);
            partial_overlap += 1;
        } else {
            println!("{}-{} does not overlap with {}-{}", a, b, c, d);
        }
    }

    println!("Fully contained count: {}", fully_contained_count);
    println!("Partial overlap count: {}", partial_overlap);
}
