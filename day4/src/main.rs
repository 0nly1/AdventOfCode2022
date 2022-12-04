use std::fs;

fn main() {
    let input = get_input(String::from("input.txt"));
    println!("{input}"); 

    first_part(input.clone());
    second_part(input);
}

fn first_part(input: String) {
    let pairs = input.split("\n");    
    let mut count = 0;

    for pair in pairs {
        if pair.is_empty() {
            break;
        }

        let elves_in_pair = pair.split(","); 
        
        let mut start = usize::MAX; 
        let mut end = usize::MIN; 

        // try without for here? 
        for sections in elves_in_pair {
            let assigments: Vec<usize> = sections.split("-").map(|el| el.parse::<usize>().unwrap()).collect();

            if start == usize::MAX {
                start = assigments[0];
                end = assigments[1];
                continue;
            }

            let first = start <= assigments[0] && assigments[1] <= end;
            let second = assigments[0] <= start && end <= assigments[1]; 

            if first || second { 
                // println!("{start}..{end} - {}..{}: overlaping", assigments[0], assigments[1]);
                count += 1;
                break;
            }

        }
    }

    println!("\nFirst. Total pairs: {count}");
}

fn second_part(input: String) {
    let pairs = input.split("\n");    
    let mut count = 0;

    for pair in pairs {
        if pair.is_empty() {
            break;
        }

        let elves_in_pair: Vec<&str> = pair.split(",").collect(); 
        
        let mut start = usize::MAX; 
        let mut end = usize::MIN; 

        // ass stands for assignments 
        let ass1: Vec<usize> = elves_in_pair[0].split("-").map(|el| el.parse::<usize>().unwrap()).collect();
        let ass2: Vec<usize> = elves_in_pair[1].split("-").map(|el| el.parse::<usize>().unwrap()).collect();

        if ass1[1] < ass2[0] || ass1[0] > ass2[1] {
            continue;
        }
        
        count += 1; 
    }

    println!("\nSecond. Total pairs: {count}");
}

fn get_input(file_path: String) -> String {
    fs::read_to_string(file_path).unwrap()
}
