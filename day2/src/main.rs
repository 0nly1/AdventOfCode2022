use std::fs;
use std::collections::HashMap;

fn main() {
    let input = get_input(String::from("input.txt"));

    println!("{}", input); 

    first_part(input.clone());
    second_part(input);
}

fn first_part(input: String) {
    let hand_scores: HashMap<&str, i32> = HashMap::from([
        // Rock   Paper     Scissors
        ("A", 1), ("B", 2), ("C", 3),
        ("X", 1), ("Y", 2), ("Z", 3)
    ]);
   
    let rounds = input.split("\n");
    let mut total_score = 0;

    for round in rounds {
        if round.is_empty() {
           break; 
        }

        let hands: Vec<&str> = round.split(" ").collect();
        let opponent = hand_scores.get(hands[0]).unwrap();
        let me = hand_scores.get(hands[1]).unwrap(); 

        print!("Opponent: {opponent} vs Me {me}");

        let winner: i32 = get_winner(*opponent, *me);  
        
        let score: i32 = match winner {
            // draw
            0 => 3,
            // lost
            1 => 0,
            // win
            2 => 6,
            _ => panic!("Save my soul")
        };
        
        total_score += score + me; 
        
        // output the result
        if score == 0 {
            print!(" I lost...");
        } else if score == 3 {
            print!(" draw.");
        } else {
            print!(" I won!");
        }
        print!(" Score: {score} + {me}. Total: {total_score} \n");
    }

    println!("{}", total_score);
}

fn second_part(input: String) {
     let hand_scores: HashMap<&str, i32> = HashMap::from([
        // Rock   Paper     Scissors
        ("A", 1), ("B", 2), ("C", 3),
    ]);
   
    let rounds = input.split("\n");
    let mut total_score = 0;

    for round in rounds {
        if round.is_empty() {
           break; 
        }

        let data: Vec<&str> = round.split(" ").collect();
        let opponent = hand_scores.get(data[0]).unwrap();
        
        let score: i32 = match data[1] {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,

            _ => panic!("What am I")
        };

        let me = get_my_hand(*opponent, score); 

        print!("{round}({score}): Opponent: {opponent} vs Me {me}");
        
        // output the result
        if score == 0 {
            print!(" I lost...");
        } else if score == 3 {
            print!(" draw.");
        } else {
            print!(" I won!");
        }

        total_score += score + me; 

        print!(" Score: {score} + {me}. Total: {total_score} \n");
    }

    println!("{}", total_score);    
}

fn get_my_hand(opponent: i32, score: i32) -> i32 {
    let mut res: i32;

    if score == 3 {
        res = opponent;
    } else if score == 6 {
        res = opponent + 1;

        if res > 3 {
            res = 1;
        }
    } else {
        res = opponent - 1;

        if res == 0 {
            res = 3;
        }
    }

    return res; 
}

fn get_winner(opponent: i32, me: i32) -> i32 {
    if opponent == me {
        return 0;
    // Rock vs Scissors = OPPONENT won 
    } else if opponent == 1 && me == 3 {
        // I lost
        return 1;
    // Scissors vs Rock = I won
    } else if opponent == 3 && me == 1 {
        // I won
        return 2;
    } else if opponent < me {
        // I won 
        return 2;
    } else if opponent > me {
        // I lost
        return 1;
    }

    return 2;
}

fn get_input(file_path: String) -> String {
    fs::read_to_string(file_path).unwrap()
}
