fn main() {
    // Puzzle: https://adventofcode.com/2022/day/1
    // Input link: https://adventofcode.com/2022/day/1/input
    let input = get_input(); // "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    let list = input.split("\n");

    // let mut max = 0;
    let mut max_arr: [i32; 3] = [0, 0, 0];
    let mut current_elf = 0;

    for item in list { 
        if item.is_empty() {
            let max = max_arr[0];
            
            if max < current_elf { 
                max_arr[2] = max_arr[1]; 
                max_arr[1] = max_arr[0];
                max_arr[0] = current_elf;
                println!("New max: {}", current_elf);
            } 

            current_elf = 0;
            continue;
        } 

        current_elf += item.parse::<i32>().unwrap();
    }
    
    println!("The answer: {}", max_arr[0] + max_arr[1] + max_arr[2]);
}

fn get_input() -> String {
    let url = "https://adventofcode.com/2022/day/1/input";
    let cookie = "session=53616c7465645f5f994a1a4f1a1a92356f167b987682e556edeb917fbcbb14a95d12e55a9c665eca4e0a620328abbf68dfa3af45746a56e6bea3571b805b5f70";

    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(url)
        .header("cookie", cookie)
        .send();

    // let mut resp = reqwest::blocking::get(url).unwrap();
    resp.unwrap().text().unwrap()
}
