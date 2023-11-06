use std::io::{stdin, Stdin};

fn sum_first(mut tuple: (u32, u32)) -> (u32, u32) {
    tuple.0 = tuple.0 + tuple.1;
    tuple
}
fn sum_second(mut tuple: (u32, u32)) -> (u32, u32) {
    tuple.1 = tuple.0 + tuple.1;
    tuple
}

#[derive(Clone, Debug)]
pub struct Solution {
    steps: Vec<(u32, u32)>,
}

impl Solution {
    pub fn new(initial_position: (u32, u32)) -> Self {
        Solution {
            steps: vec![initial_position],
        }
    }
    pub fn add_position(&mut self, position: (u32, u32)) {
        self.steps.push(position);
    }
}

fn isPossible(a: i32, b: i32, c: i32, d: i32) -> String {
    let mut elem = (a as u32, b as u32);
    let mut goal = (c as u32, d as u32);

    let mut solutions: Vec<Solution> = Vec::new();

    "Yes".to_string()
}

pub struct Game {
    result: (u32, u32),
    visited: Vec<(u32, u32)>,
}

impl Game {
    pub fn new(result: (u32, u32)) -> Self {
        Game {
            result,
            visited: Vec::new(),
        }
    }

    fn display_board(&self, solution: &Solution) -> bool {
        println!("partial sols = {:?}", solution);
        println!("result = {:?}", self.result);
        for row in (0..self.result.0 + 1).rev() {
            for col in 0..(self.result.1 + 1) {
                if row == self.result.0 && col == self.result.1 {
                    print!("[#]");
                    continue;
                }
                if solution.steps.iter().any(|&x| x == (col, row)) {
                    print!("[+]");
                } else {
                    print!("[ ]");
                }
            }
            println!();
        }
        return self.read_quit();
    }

    pub fn is_valid(&self, solution: &Solution) -> bool {
        if let Some(value) = solution.steps.last() {
            if self.visited.iter().any(|&x| x == *value) {
                return false;
            }
            *value == self.result
        } else {
            false
        }
    }

    pub fn is_a_candidate(&self, movement: (u32, u32)) -> bool {
        if movement.1 > self.result.1 || movement.0 > self.result.0 {
            return false;
        }
        true
    }

    pub fn read_quit(&self) -> bool {
        use std::io;
        let mut buffer = String::new();
        if io::stdin().read_line(&mut buffer).is_ok() {
            if buffer.starts_with("quit") {
                return true;
            }
        }
        false
    }

    pub fn play(&mut self, input: (u32, u32)) -> Vec<Solution> {
        let mut solutions: Vec<Solution> = Vec::new();
        let initial_solution = Solution::new(input);

        //Initial step , solution (the set of steps are importants to define a solution, if not,
        //you don-t need to save all the steps)
        let mut stack: Vec<Solution> = vec![initial_solution];

        while !stack.is_empty() {
            // Get first available possible solution
            let candidate_solution: Solution = stack.remove(0);
            //if self.display_board(&candidate_solution) {
            //    return solutions;
            //}

            // Is solution, save and discard, use other candidates for more solution
            // If you only needs one.. stop the program
            if self.is_valid(&candidate_solution) {
                println!("> It is a solution <");
                solutions.push(candidate_solution.clone());
                continue;
            }

            // Calculate next movements
            let possible_movements = [
                sum_first(*candidate_solution.steps.last().expect("last doesn t exist")),
                sum_second(*candidate_solution.steps.last().expect("last doesn t exist")),
            ];

            // Evaluate next movements (It can be done just before) and save them for next
            // iterations
            for possible_movement in possible_movements {
                if self.is_a_candidate(possible_movement) {
                    let mut new_possible_solution = candidate_solution.clone();
                    new_possible_solution.add_position(possible_movement);
                    stack.push(new_possible_solution);
                } else {
                    // For avoiding loops.... we save if we visited this step.
                    self.visited.push(possible_movement);
                }
            }
        }
        solutions
    }

    fn find_one_result(&mut self, input: (u32, u32)) -> String {
        let mut stack: Vec<(u32, u32)> = vec![input];

        while !stack.is_empty() {
            let candidate_solution: (u32, u32) = stack.remove(0);
            if candidate_solution == self.result {
                return "Yes".to_string();
            }
            let possible_movements = [
                sum_first(candidate_solution),
                sum_second(candidate_solution),
            ];
            for possible_movement in possible_movements {
                if self.is_a_candidate(possible_movement) {
                    stack.push(possible_movement);
                } else {
                    self.visited.push(possible_movement);
                }
            }
        }
        "No".to_string()
    }

    fn find_one_result_with_heuristic(&mut self, input: (u32, u32)) -> String {
        let mut stack: Vec<(u32, u32)> = vec![input];

        while !stack.is_empty() {
            let candidate_solution: (u32, u32) = stack.remove(0);
            if candidate_solution == self.result {
                return "Yes".to_string();
            }
            let possible_movements = [
                sum_first(candidate_solution),
                sum_second(candidate_solution),
            ];
            for possible_movement in possible_movements {
                if self.is_a_candidate(possible_movement) {
                    stack.push(possible_movement);
                    self.sort_by_heuristic(&mut stack)
                } else {
                    self.visited.push(possible_movement);
                }
            }
        }
        "No".to_string()
    }

    pub fn euclidean_distance(&self, p1: (u32, u32), p2: (u32, u32)) -> f64 {
        let dx = p2.0 - p1.0;
        let dy = p2.1 - p1.1;
        ((dx * dx + dy * dy) as f64).sqrt()
    }

    //TODO check this heuristic
    pub fn sort_by_heuristic(&mut self, possible_movements: &mut Vec<(u32, u32)>) {
        possible_movements.sort_by(|&a, &b| {
            self.euclidean_distance(a, self.result)
                .partial_cmp(&self.euclidean_distance(b, self.result))
                .unwrap()
        });
    }
}

fn main() {
    let input1 = (1, 4);
    let output1 = (5, 9);

    let mut game = Game::new(output1);
    let solutions = game.play(input1);

    println!("Solutions = {:?}", solutions);
    println!("{:}", game.find_one_result(input1));

    let input2 = (1, 1);
    let output2 = (5, 2);

    let mut game2 = Game::new(output2);
    let solutions2 = game2.play(input2);

    println!("Solutions = {:?}", solutions2);
    println!("{:}", game2.find_one_result(input2));

    let input3 = (1, 2);
    let output3 = (3, 6);
    let mut game3 = Game::new(output3);
    let solutions3 = game3.play(input3);
    println!("Solutions = {:?}", solutions3);
    println!("{:}", game3.find_one_result(input3));
}
