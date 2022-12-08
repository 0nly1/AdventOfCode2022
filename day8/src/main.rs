use std::fs;

fn main() {
    let input = get_input(String::from("input.txt"));
    println!("{input}"); 

    first_part(&input);
    println!();
    second_part(&input);
}

fn first_part(input: &str) {
    let matrix: Vec<Vec<u32>> = create_matrix(input); 
    output_matrix(&matrix);

    let mut invisible: Vec<(usize, usize)> = vec![];

    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[i].len() - 1 {
            if !is_visible_horizontaly(&matrix, i, j) && 
               !is_visible_verticaly(&matrix, i, j) {
                invisible.push((i, j));
            }
        }
    }

    println!("{:?}", invisible);

    let square = matrix.len() * matrix.len();
    let res = square - invisible.len();

    println!("Visible: {res}");
}

fn second_part(input: &str) {
    let matrix: Vec<Vec<u32>> = create_matrix(input);
    output_matrix(&matrix);

    let mut scores: Vec<u32> = vec![];

    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[i].len() - 1{ 
            let horizontal = score_horizontaly(&matrix, i, j);
            let vertical = score_verticaly(&matrix, i, j);

            let tree_score = horizontal * vertical;

            scores.push(tree_score);
        }
    }

    println!("Scores: {:?}", scores);
    let max = scores.iter().max().unwrap();

    println!("Max: {max}");
}

fn score_horizontaly(matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut left = 0;
    let mut right = 0;

    for i in (0..y).rev() {
        left += 1;

        if matrix[x][y] <= matrix[x][i] {
            break;
        }
    }

    for i in (y+1)..matrix.len() {
        right += 1;

        if matrix[x][y] <= matrix[x][i] {
            break;
        }
    }

    return left * right;
}

fn score_verticaly(matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut up = 0; 
    let mut down = 0; 

    for i in (0..x).rev() {
        up += 1;

        if matrix[x][y] <= matrix[i][y] {
            break;
        }
    }

    for i in (x+1)..matrix.len() {
        down += 1; 

        if matrix[x][y] <= matrix[i][y] {
            break;
        }
    }

    return up * down; 
}

fn is_visible_horizontaly(matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let mut left = true;
    let mut right = true;

    // look by column
    for i in 0..y {
        // if there is a bigger tree, it's invisible
        if matrix[x][y] <= matrix[x][i] {
            left = false;
            break;
        }
    }

    for i in (y+1)..matrix.len() {
        // if there is a bigger tree, it's invisible
        if matrix[x][y] <= matrix[x][i] {
            right = false;
            break;
        }
    }

    return left || right; 
}

fn is_visible_verticaly(matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let mut up = true; 
    let mut down = true; 

    // look by column
    for i in 0..x {
        // if there is a bigger tree, it's invisible
        if matrix[x][y] <= matrix[i][y] {
            up = false;
            break;
        }
    }

    for i in (x+1)..matrix.len() {
        // if there is a bigger tree, it's invisible
        if matrix[x][y] <= matrix[i][y] {
            down = false;
            break;
        }
    }

    return up || down; 
}

fn output_matrix(matrix: &Vec<Vec<u32>>) {
    for el in matrix {
        println!("{:?}", el);
    }
}

fn create_matrix(input: &str) -> Vec<Vec<u32>>{
    let mut matrix = vec![];
    let radix: u32 = 10; 

    for line in input.lines() {
        matrix.push(line.chars()
            .map(|tree| tree.to_digit(radix).unwrap()).collect());
    }

    matrix
}

fn get_input(file_path: String) -> String {
    fs::read_to_string(file_path).unwrap()
}
