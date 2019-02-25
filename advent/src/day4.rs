

pub mod day4 {

use std::io::{self, BufRead};
use std::collections::HashMap;
use std::fmt;
use chrono::prelude::*;

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

pub fn code4 () {
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
}
}
