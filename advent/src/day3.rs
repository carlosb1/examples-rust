use std::io::{self, BufRead};
use std::fmt;

static DEFAULT: i32 = 0;
static MULTIPLE:  i32 = -1;

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

