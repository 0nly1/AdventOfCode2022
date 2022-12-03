use std::fs;

fn main() {
    let input = get_input(String::from("input.txt"));
    println!("{input}"); 

    // first_part(input.clone());
    second_part(input);
}

fn first_part(input: String) {
    let rucksacks = input.split("\n");
    let mut priority_sum: usize = 0;

    for rucksack in rucksacks {
        if rucksack.is_empty() {
            break;
        }

        let items: Vec<char> = rucksack.chars().collect();
        let length = items.len();
        // let (first, second) = rucksack.split_at(length / 2);

        let mut result: char = ' ';

        for i in 0..(length / 2) {
            let item = items[i];
            result = get_char_from_list(item, items.clone(), length);

            if result != ' ' {
                break;
            }
        }
        
        let priority: usize = get_priority(result); 
        priority_sum += priority;

        println!("Length: {length}, Result: {result}, Priority: {priority}");
    }

    println!("Sum: {priority_sum}");
}

fn get_char_from_list(to_find: char, items: Vec<char>, length: usize) -> char{
    for i in (length / 2)..length {
        if to_find == items[i] {
            return items[i];
        }
    }

    return ' ';
}

fn second_part(input: String) {
    let rucksacks: Vec<&str> = input.split("\n").collect();
    let len = rucksacks.len();
    println!("Length: {len}");
    let mut sum = 0;
    let mut count = 1; 

    for group in rucksacks.chunks(3) {
        if group.len() < 3 {
            let rucksack = group[0];
            println!("First rucksack: {rucksack}");
            break;
        }
        
        let badge = get_badge(group.to_vec());
        
        let priority = get_priority(badge);
        sum += priority;

        println!("{count}: Badge: {badge}, Priority: {priority}, Sum: {sum}");
        count += 1;

    }

    println!("Sum: {sum}");
}

fn get_badge(group: Vec<&str>) -> char {
    let length: usize = group.len();

    println!("{:02?}", group);
    
    let first_rucksack = group[0];
    let second_rucksack = group[1];

    let mut init_chars = get_initial_chars(
            group[0].chars().collect(), 
            group[1].chars().collect()
        );

    println!("Init chars: {:02?}", init_chars);

    for item in init_chars.clone() {
        if in_init_chars(item, group[2].chars().collect()) {
            return item
        }
    }

    return ' '; 
}

fn get_initial_chars(first_items: Vec<char>, second_items: Vec<char>) -> Vec<char> {
    let mut res = vec![];
    
    for to_find in first_items.clone() {
        for item in second_items.clone() {
            if to_find == item {
                if res.len() < 1 {
                    res.push(item);
                } else {
                    if !exists_in_vec(item, res.clone()) {
                        res.push(item);
                    }
                }
            }
        }
    }

    return res;
}

fn in_init_chars(to_find: char, items: Vec<char>) -> bool {
    for item in items {
        if to_find == item {
            return true;
        }
    }

    return false;
}

fn exists_in_vec(item: char, chars: Vec<char>) -> bool {
    let exists = false;

    for c in chars {
        if c == item {
            return true;
        }
    }

    return exists; 
}

fn get_priority(item: char) -> usize {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    let mut count: usize = 1;

    for letter in alphabet {
        if letter == item {
            return count;
        }

        count += 1;
    }

    return 0; 
}

fn get_input(file_path: String) -> String {
    fs::read_to_string(file_path).unwrap()
}
