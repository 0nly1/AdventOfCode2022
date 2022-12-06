use std::fs;

fn main() {
    let input = get_input(String::from("input.txt"));
    println!("{input}"); 

    first_part(&input);
    println!();
    second_part(&input);
}

fn first_part(input: &str) {
    let stream: Vec<char> = input.chars().collect();

    let mut window: Vec<char> = vec![];
    let mut count: usize = 0;

    for el in stream {
        count += 1;
        
        while window.contains(&el) {
            window.remove(0);
        }

        window.push(el);

        if window.len() == 4 {
            break;
        }
    }

    println!("The answer is {count}");
}

fn second_part(input: &str) {
    let stream: Vec<char> = input.chars().collect();

    let mut window: Vec<char> = vec![];
    let mut count: usize = 0;

    for el in stream {
        count += 1;
        
        while window.contains(&el) {
            window.remove(0);
        }

        window.push(el);

        if window.len() == 14 {
            break;
        }
    }

    println!("The answer is {count}");
}

fn get_input(file_path: String) -> String {
    fs::read_to_string(file_path).unwrap()
}
