use std::io::{self, BufRead};
use std::collections::HashMap;

fn code1() {
    let stdin = io::stdin();

    let mut lines = stdin.lock().lines().fuse();
    let input = match lines.next()  {
        Some(Ok(a))  => a,
        _ => panic!("Couldn t read input.")
    };
    
    let info = input.trim();
    let splitted = info.split(",");
    let iter = splitted.map(|a| a.to_string().parse().unwrap_or(0));
    
    let mut result  = 0;
    for num in iter {
        result += num;
    }
    println!("{:?}", result); 
}


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

fn paint_matrix(matrix: &mut [[&str; 10]; 10]) {
    for (i, row) in  matrix.iter_mut().enumerate() {
        for (y, col) in row.iter_mut().enumerate() {
             print!("{}", col);  
        }
        println!();
    }
}

fn code3() {
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
    let mut matrix = [["."; 10];10];
    paint_matrix(&mut matrix);  
}


fn main() {
    //code1();
    //code2();
    code3();
}
