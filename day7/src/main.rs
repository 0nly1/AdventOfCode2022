use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = get_input(String::from("input.txt"));
    println!("{input}"); 

    first_part(&input);
    println!();
    second_part(&input);
}

fn first_part(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    let mut dir_to_size: HashMap<Vec<&str>, usize> = HashMap::new();
    let mut seen: HashSet<Vec<&str>> = HashSet::new(); 

    let mut path: Vec<&str> = vec![];
    let mut listed: Vec<&str> = vec![];

    let mut directory_processed = false;

    for line in lines {
        let line_data: Vec<&str> = line.split(' ').collect();
        println!("Processing line '{}'", line);

        match line_data[0] {
            "$" => {
                match line_data[1] {
                    "ls" => {
                        seen.insert(listed);
                        listed = path.clone();

                        directory_processed = seen.contains(&listed);

                        dir_to_size.entry(path.clone()).or_insert(0);
                    }

                    "cd" => {
                        process_path(&mut path, line_data[2]);
                        println!("{:?}", path);
                    }

                    _ => unreachable!()
                }
            }

            "dir" => {
                // ignore
            }

            // file with size
            _ => {
                if directory_processed {
                    continue;
                }
                
                let size = line_data[0].parse::<usize>().unwrap();

                dir_to_size.entry(listed.clone())
                    .and_modify(|total| *total += size);
            } 
        }
    }
    
    // sum all the directories
    let mut dirs: HashMap<Vec<&str>, usize> = HashMap::new();

    for (path, size) in dir_to_size.into_iter() {
        for to in 1..=path.len() {
            match dirs.get_mut(&path[..to]) {
                Some(total) => {
                    *total += size;
                }
                None => {
                    dirs.insert(path[..to].to_vec(), size);
                }
            }
        }
    }

    println!("{:?}", dirs);

    let sum: usize = dirs.into_values().filter(|&size| size <= 100_000).sum();
    println!("{sum}");
}

fn second_part(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    let mut dir_to_size: HashMap<Vec<&str>, usize> = HashMap::new();
    let mut seen: HashSet<Vec<&str>> = HashSet::new(); 

    let mut path: Vec<&str> = vec![];
    let mut listed: Vec<&str> = vec![];

    let mut directory_processed = false;

    for line in lines {
        let line_data: Vec<&str> = line.split(' ').collect();
        println!("Processing line '{}'", line);

        match line_data[0] {
            "$" => {
                match line_data[1] {
                    "ls" => {
                        seen.insert(listed);
                        listed = path.clone();

                        directory_processed = seen.contains(&listed);

                        dir_to_size.entry(path.clone()).or_insert(0);
                    }

                    "cd" => {
                        process_path(&mut path, line_data[2]);
                        println!("{:?}", path);
                    }

                    _ => unreachable!()
                }
            }

            "dir" => {
                // ignore
            }

            // file with size
            _ => {
                if directory_processed {
                    continue;
                }
                
                let size = line_data[0].parse::<usize>().unwrap();

                dir_to_size.entry(listed.clone())
                    .and_modify(|total| *total += size);
            } 
        }
    }
    
    // sum all the directories
    let mut dirs: HashMap<Vec<&str>, usize> = HashMap::new();

    for (path, size) in dir_to_size.into_iter() {
        for to in 1..=path.len() {
            match dirs.get_mut(&path[..to]) {
                Some(total) => {
                    *total += size;
                }
                None => {
                    dirs.insert(path[..to].to_vec(), size);
                }
            }
        }
    }

    println!("{:?}", dirs);

    let device_storage = 70_000_000_usize;
    let space_needed = 30_000_000_usize;
    let space_used = dirs.get(&vec!["/"]).unwrap();
    let available_space = device_storage - space_used; 
    
    println!("Available space: {}", available_space);

    let need_to_clear = space_needed - available_space;
    
    println!("Need to clear: {}", need_to_clear);

    let min_size = dirs.into_values()
        .filter(|&size| size >= need_to_clear).min().unwrap(); 
    println!("{}", min_size);
}

fn process_path<'a>(path: &mut Vec<&'a str>, argument: &'a str) {
    match argument {
        "/" => {
            path.clear();
            path.push(argument);
        }

        ".." => {
            path.pop();
        }

        _ => {
            path.push(argument);
        }
    }
}

fn old_output_tree(current_dir: &str, dirs: &HashMap<&str, Vec<&str>>, sizes: &HashMap<&str, usize>) {
    let size = sizes.get(current_dir).unwrap();
    println!("{}: {}", current_dir, size);

    let insides_wrap = dirs.get(current_dir);

    if insides_wrap == None || insides_wrap.unwrap().len() == 0 {
        return;
    } 

    println!("{{");
    for dir in insides_wrap.unwrap() {
        old_output_tree(dir, &dirs, &sizes);
    }

    println!("}}");
}

fn get_input(file_path: String) -> String {
    fs::read_to_string(file_path).unwrap()
}
