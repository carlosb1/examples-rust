extern crate regex;
extern crate chrono;

use std::io::{self, BufRead};
use std::collections::HashMap;
use std::fmt;
use chrono::prelude::*;

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


static DEFAULT: i32 = 0;
static MULTIPLE:  i32 = -1;

fn paint_matrix(matrix: &mut [[i32; 10]; 10]) {
    for (i, row) in  matrix.iter_mut().enumerate() {
        for (y, col) in row.iter_mut().enumerate() {
             if *col == 0 as i32 {
                print!(".");
             } else if *col == -1 as i32 {
                print!("X");
             } else {
                print!("{}", col);  
             }
        }
        println!();
    }
}

fn fill_matrix(matrix: &mut [[i32; 10]; 10], entry: &mut Entry) {
    for x in entry.index_x..entry.width{
        for y in entry.index_y..entry.height {
            let current_value = matrix[x as usize][y as usize];
            if current_value == DEFAULT {
                 matrix[x as usize][y as usize] = entry.num; 
                 //matrix[x as usize][y as usize] = "1"; 
            } else {
                if current_value != MULTIPLE {
                    matrix[x as usize][y as usize] = MULTIPLE;
                }
            }
        }
    }
}

pub struct Entry {
    num: i32,
    index_x: i32,
    index_y: i32,
    width: i32,
    height: i32
}

impl fmt::Display for Entry {
    fn fmt(&self,fmt:  &mut fmt::Formatter) -> fmt::Result {
        let str_num = "num=".to_string()+&self.num.to_string();
        let str_index_x = "index_x=".to_string() + &self.index_x.to_string(); 
        let str_index_y = "index_y=".to_string() + &self.index_y.to_string();
        let str_w = "w=".to_string() + &self.width.to_string();
        let str_h = "h=".to_string() + &self.height.to_string();
        let parsed = str_num + " " + str_index_x.as_str() + " " +  str_index_y.as_str() + " " +  str_w.as_str() + " " +  str_h.as_str();
        fmt.write_str(parsed.as_str())?;
        Ok(())
    }
}


fn code3() {
    use regex::Regex;
    let re = Regex::new(r"^#(\d) @ (\d),(\d): (\d)x(\d)").unwrap();
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
    // check input
    let mut entries: Vec<Entry> = Vec::new();
    for line in vec {
        println!("{:?}", re.is_match(line.as_str()));
        for cap in re.captures_iter(line.as_str()) {
            let num: i32 = cap[1].parse::<i32>().unwrap();
            let index_x: i32 = cap[2].parse::<i32>().unwrap();
            let index_y: i32 = cap[3].parse::<i32>().unwrap();
            let width: i32 = cap[4].parse::<i32>().unwrap();
            let height: i32 = cap[5].parse::<i32>().unwrap();
            let entry = Entry{num, index_x, index_y, width, height};
            entries.push(entry);
        }
    }

    let mut matrix = [[DEFAULT; 10];10];
    
    for mut ent in entries {
        println!("{}",ent);
        fill_matrix(&mut matrix, &mut ent);
    }

    paint_matrix(&mut matrix);

}



#[derive(PartialEq, Clone)]
enum Event {
    Shift,
    Asleep,
    Wakeup
}

#[derive(PartialEq, Clone)]
pub struct SleepEvent  {
    start_sleep: i64,
    stop_sleep: i64,
    quantity: i32
}

pub struct Soldier {
    num: i32,
    status: Event,
    start_sleep: Option<DateTime<Utc>>,
    events: Vec<SleepEvent>
}

impl Soldier {
    fn new(num: i32) -> Soldier {
        Soldier{ num: num, status: Event::Wakeup, start_sleep: None, events: Vec::new()}
    }

    fn add_event(& mut self, event: Event, year: i32, month: i32, day: i32, hour: i32, minutes: i32) {
        println!("id: {}", self.num);
        let dt = Utc.ymd(year, month as u32, day as u32).and_hms(hour as u32, minutes as u32, 0);
     //   self.events.push(event);
        match  event {
            Event::Asleep => {
                println!("Asleep {}", dt.timestamp());
                self.start_sleep = Some(dt);
            },
            Event::Wakeup => {
                let stop_sleep = dt;
                let quantity = (stop_sleep.timestamp() - self.start_sleep.unwrap().timestamp()) as i32;
                let sleepEvent: SleepEvent = SleepEvent{start_sleep: self.start_sleep.unwrap().timestamp(), stop_sleep: stop_sleep.timestamp(), quantity: quantity};
                self.events.push(sleepEvent);
            },
            _ => {
                println!("Other event");
            }
        }
    }
    fn contains_event(&mut self, moment_sleep: i32) -> Option<SleepEvent> {
        let bestSleep = self.events.iter().filter(|x| x.start_sleep >= moment_sleep as i64 || x.stop_sleep <= moment_sleep as i64).min_by_key(|x| x.quantity);
        match bestSleep {
            Some(x) => return Some(x.clone()),
            None => return None
        }
    }
}

fn code4 () {
    
    use regex::Regex;
    let mut re_shift = Regex::new(r"^\[(\d{4})\-(\d{2})\-(\d{2}) (\d{2}):(\d{2})\] Guard #(\d{2}) begins shift").unwrap();
    let mut re_fall_sleeps = Regex::new(r"^\[(\d{4})\-(\d{2})\-(\d{2}) (\d{2}):(\d{2})\] falls asleep").unwrap();
    let mut re_wakes_up = Regex::new(r"^\[(\d{4})\-(\d{2})\-(\d{2}) (\d{2}):(\d{2})\] wakes up").unwrap();

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
    // check input
    //let mut entries: Vec<SoldierEvent> = Vec::new();
    
    
    let mut soldiers: HashMap<i32, Soldier> = HashMap::new();

    let mut current_soldier = -1;
    for line in vec {
        let typ: Event;
        let re;
        if (re_shift.is_match(line.as_str())) {
           re = re_shift.clone();
           typ = Event::Shift;
        } else if (re_fall_sleeps.is_match(line.as_str())) {
           re = re_fall_sleeps.clone();  
           typ = Event::Asleep;
        } else {
           re = re_wakes_up.clone();  
           typ = Event::Wakeup;
        }
        
        for cap in re.captures_iter(line.as_str()) {    
            let year: i32 = cap[1].parse::<i32>().unwrap();
            let month: i32 = cap[2].parse::<i32>().unwrap();
            let day: i32 = cap[3].parse::<i32>().unwrap();
            let hour: i32 = cap[4].parse::<i32>().unwrap();
            let minutes: i32 = cap[5].parse::<i32>().unwrap(); 
            if(typ == Event::Shift) {
                current_soldier = cap[6].parse::<i32>().unwrap();
                soldiers.entry(current_soldier).or_insert(Soldier::new(current_soldier));
            }
            let soldier = soldiers.get_mut(&current_soldier).unwrap();
            soldier.add_event(typ.clone(), year, month, day, hour, minutes);
        }
    }

    //TODO Finish code to search best sentynel
}


fn main() {
    //code1();
    //code2();
    //code3();
    code4();
}
