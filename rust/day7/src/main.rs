use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct FileInfo {
    name: String,
    size: u64,
}

#[derive(Debug, Clone)]
struct DirInfo {
    name: String,
    files: Vec<FileInfo>,
    subdirs: Vec<String>,
}

fn get_full_path(dir_stack: Vec<String>) -> String {
    let mut full_path = String::new();
    for dir in dir_stack {
        full_path.push_str(&dir);
        full_path.push_str("/");
    }
    full_path
}

fn get_total_size_recursively(
    dir_info: DirInfo,
    dir_stack: Vec<String>,
    dir_path_to_info: &std::collections::HashMap<String, DirInfo>,
) -> u64 {
    let mut total_size = 0;
    for file in dir_info.files {
        total_size += file.size;
    }

    for subdir in dir_info.subdirs {
        let mut new_dir_stack = dir_stack.clone();
        new_dir_stack.push(subdir.clone());
        let idx = get_full_path(new_dir_stack.clone());
        let subdir_info = dir_path_to_info.get(&idx).unwrap();
        total_size += get_total_size_recursively(
            subdir_info.clone(),
            new_dir_stack.clone(),
            dir_path_to_info,
        );
    }
    return total_size;
}

fn main() {
    let output = read_to_string("input.txt").unwrap();

    let mut dir_stack: Vec<String> = vec![];

    let mut dir_path_to_info: std::collections::HashMap<String, DirInfo> =
        std::collections::HashMap::new();

    for line in output.lines().into_iter() {
        if line.starts_with("$ cd ") {
            // add the current dir to the path stack
            let path = line.split(" ").nth(2).unwrap().to_string();
            // if the path is ".." then pop the stack
            if path == ".." {
                dir_stack.pop();
            } else {
                dir_stack.push(path);
            }
        } else if line.starts_with("$ ls") {
            // Add the current dir info to the dir dict
            let idx = get_full_path(dir_stack.clone());

            dir_path_to_info.insert(
                idx,
                DirInfo {
                    name: dir_stack.last().unwrap().to_string(),
                    files: vec![],
                    subdirs: vec![],
                },
            );
        } else if line.starts_with("dir ") {
            // get the dir name
            let dir_name = line.split(" ").nth(1).unwrap().to_string();
            let idx = get_full_path(dir_stack.clone());

            // add the dir to the current dir
            let cur_dir = dir_path_to_info.get_mut(&idx).unwrap();

            cur_dir.subdirs.push(dir_name);
        } else if line.chars().nth(0).unwrap().is_digit(10) {
            // this is a file size, so add the file to the current dir
            let file_name = line.split(" ").nth(1).unwrap().to_string();
            let file_size = line.split(" ").nth(0).unwrap().to_string();
            let idx = get_full_path(dir_stack.clone());

            // add the dir to the current dir
            let cur_dir = dir_path_to_info.get_mut(&idx).unwrap();
            cur_dir.files.push(FileInfo {
                name: file_name,
                size: file_size.parse::<u64>().unwrap(),
            });
        }
    }

    println!("{:#?}", dir_path_to_info);

    let root_dir = dir_path_to_info.get("//").unwrap();
    let mut sum_size = 0;

    let mut sizes: Vec<u64> = vec![];

    // iterate through the keys of the hashmap
    for (key, value) in dir_path_to_info.iter() {
        // calc the dir stack from the key
        let mut dir_stack: Vec<String> = key
            .split("/")
            .filter(|x| x != &"")
            .map(|x| x.to_string())
            .collect();
        dir_stack.insert(0, String::from("/"));

        let dir_total_size =
            get_total_size_recursively(value.clone(), dir_stack, &dir_path_to_info);

        if dir_total_size < 100000 {
            println!("{}: {}", key, dir_total_size);
            sum_size += dir_total_size;
        }
        sizes.push(dir_total_size);
    }

    sizes.sort();
    println!("Total size: {}", sum_size);

    let free_space = 70000000 - sizes.iter().last().unwrap();

    // print the first size in sizes larger than 30000000
    for size in sizes {
        if free_space + size > 30000000 {
            println!("First size larger than 30000000: {}", size);
            break;
        }
    }
}
