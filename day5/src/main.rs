use std::fs;

fn main() {
    let input = get_input(String::from("input.txt"));
    println!("{input}"); 

    first_part(&input);
    println!();
    second_part(&input);
}

fn first_part(input: &str) {
    let input_parts: Vec<&str> = input.split("\n\n").collect();
    
    let crates_raw: Vec<&str> = input_parts[0].lines().collect();
    let instructions = input_parts[1].split("\n");

    let mut matrix = parse_crates(crates_raw);
    println!("Starting point:");
    output_crates(&matrix);
    println!();

    for instruction in instructions {
        if instruction.is_empty() {
            break;
        }

        let vec_raw: Vec<&str> = instruction.split(' ').collect();

        let amount = vec_raw[1].parse::<usize>().unwrap();
        let from_row = vec_raw[3].parse::<usize>().unwrap() - 1;
        let to_row = vec_raw[5].parse::<usize>().unwrap() - 1;
        
        for iter in 0..amount {
            let temp = matrix[from_row].pop().unwrap();
            matrix[to_row].push(temp); 
        }    
    }

    output_crates(&matrix);
    
    let mut letters: Vec<char> = vec![];

    for i in 0..matrix.len() {
        letters.push(matrix[i].pop().unwrap()); 
    }

    println!("Answer: {:?}", letters);
}

fn second_part(input: &str) {
    let input_parts: Vec<&str> = input.split("\n\n").collect();
    
    let crates_raw: Vec<&str> = input_parts[0].lines().collect();
    let instructions = input_parts[1].split("\n");

    let mut matrix = parse_crates(crates_raw);
    println!("Starting point:");
    output_crates(&matrix);
    println!();

    for instruction in instructions {
        if instruction.is_empty() {
            break;
        }

        let vec_raw: Vec<&str> = instruction.split(' ').collect();

        let amount = vec_raw[1].parse::<usize>().unwrap();
        let from_row = vec_raw[3].parse::<usize>().unwrap() - 1;
        let to_row = vec_raw[5].parse::<usize>().unwrap() - 1;
        
        let from_col = matrix[from_row].len() - amount;

        let mut temp: Vec<char> = matrix[from_row].drain(from_col..).collect();
        matrix[to_row].append(&mut temp);
    }
    
    output_crates(&matrix);

    let mut letters: Vec<char> = vec![];

    for i in 0..matrix.len() {
        letters.push(matrix[i].pop().unwrap()); 
    }

    println!("res: {:?}", letters);
}

fn output_crates(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        println!("{:?}", row);
    }
}

fn parse_crates(crates: Vec<&str>) -> Vec<Vec<char>> {
    let len: usize = crates.last().unwrap().len();
    let x = crates.len() - 1;
    let y: usize = (len + 1) / 4; 

    // let matrix = [[char; x]; y];
    let mut matrix: Vec<Vec<char>> = vec![];
     
    let mut char_index: usize = 1;

    for i in 0..y {
        if i != 0 {
            char_index += 4;
        }

        let mut row: Vec<char> = vec![];
        
        for j in (0..x).rev() {
            let crates_row: Vec<char> = crates[j].chars().collect();
            let data = crates_row[char_index];

            if data == ' ' {
                break;
            }

            row.push(crates_row[char_index]);
        }

        matrix.push(row);
    }

    matrix
}

fn get_input(file_path: String) -> String {
    fs::read_to_string(file_path).unwrap()
}
