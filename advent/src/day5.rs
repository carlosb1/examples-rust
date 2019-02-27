pub mod day5 {
    use std::io::{self, BufRead};
    
    pub fn code5() {
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
        for line in vec {
            let mut updated_line = line.clone();
            let mut has_more_changes = true;

            while (has_more_changes) {
                for char_ind in  (0..updated_line.len()) {
                        //check if we can compare
                        if ((char_ind + 1) >= updated_line.len()) {
                            break;
                        }
                        let char_source = updated_line.chars().nth(char_ind).unwrap().to_uppercase().to_string();                
                        let char_cmp = updated_line.chars().nth(char_ind + 1).unwrap().to_uppercase().to_string();
                        if (char_source == char_cmp) {
                            updated_line.replace_range(char_ind..char_ind+2,"XX");
                        }
                }
                let my_chars: Vec<_> = updated_line.chars().collect();
                let cleaned_line: String = my_chars.iter().filter(|l| *l != &'X').collect();
                if (updated_line == cleaned_line) {
                    has_more_changes = false;
                }
                updated_line = cleaned_line;
            }
            println!("updated_line: {}", updated_line);
        }


    }

}
