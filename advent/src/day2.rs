use std::io::{self, BufRead};
use std::collections::HashMap;

fn code2() {
    let mut threes = 0;
    let mut two = 0;
    let mut vec: Vec<String> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let str_line = line.unwrap().trim().to_string();
        if str_line == "0" {
            println!("exit!");
            break;
        }
        vec.push(str_line);
    }
        
    let mut saved_characters: HashMap<String, i32> = HashMap::new();
    for info in &vec { 
        for charact in info.to_string().chars()  {
            let value = saved_characters.entry(charact.to_string()).or_insert(0);
            *value += 1;
        }

        let temp_threes = saved_characters.values().filter(|val| **val == 3).count();
        let temp_two = saved_characters.values().filter(|val| **val == 2).count();

        threes += temp_threes;
        two += temp_two;
    }
    println!("------------------------------");
    println!("threes: {}", threes);
    println!("twos: {}", two);
}
