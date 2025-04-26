use std::{fs::File, io::{BufRead, BufReader}};

pub const MAX_BUTTON_PRESSES: usize = 100;
pub const BUTTON_A_PRICE: usize = 3;
pub const BUTTON_B_PRICE: usize = 1;

pub struct Movement {
    pub mov_x: usize,
    pub mov_y: usize,
}

pub struct Machine {
    pub button_a: Movement,
    pub button_b: Movement,
    pub prize_x: usize,
    pub prize_y: usize,
}

fn load_movement(line: &String) -> Option<Movement> {
    if let Some(rest) = line.split_once(':') {
        let coords = rest.1.trim(); // take the part after ":"
        let mut x = 0;
        let mut y = 0;

        for part in coords.split(',') {
            let part = part.trim();
            if let Some(stripped) = part.strip_prefix('X') {
                if let Some(stripped) = stripped.strip_prefix('+') {
                    x = stripped.parse::<usize>().unwrap();
                }
            } else if let Some(stripped) = part.strip_prefix('Y') {
                if let Some(stripped) = stripped.strip_prefix('+') {
                    y = stripped.parse::<usize>().unwrap();
                }
            }
        }
        return Some( Movement { mov_x: x, mov_y: y } );
    }
    None
}

fn load_prize(line: &String) -> (usize, usize) {
    let mut x = 0;
    let mut y = 0;
    if let Some(rest) = line.split_once(':') {
        let coords = rest.1.trim(); // take the part after ":"

        for part in coords.split(',') {
            let part = part.trim();
            if let Some(stripped) = part.strip_prefix('X') {
                if let Some(stripped) = stripped.strip_prefix('=') {
                    x = stripped.parse::<usize>().unwrap();
                }
            } else if let Some(stripped) = part.strip_prefix('Y') {
                if let Some(stripped) = stripped.strip_prefix('=') {
                    y = stripped.parse::<usize>().unwrap();
                }
            }
        }
    }
    (x, y)
}

fn load_machine(button_a_str: &String, button_b_str: &String, prize: &String) -> Option<Machine> {
    if let Some(mov_a) = load_movement(button_a_str) {
        if let Some(mov_b) = load_movement(button_b_str) {
            let prize = load_prize(prize);
            return Some(Machine {
                button_a: mov_a,
                button_b: mov_b,
                prize_x: prize.0,
                prize_y: prize.1,
            });
        }
    }
    None
}

// Linear equations -> Cramer method to find the number of time we press button A and B
fn solve_equation(machine: &Machine, is_first_part: bool) -> Option<usize> {
    let determinant = (machine.button_a.mov_x * machine.button_b.mov_y) as isize - (machine.button_b.mov_x * machine.button_a.mov_y) as isize;
    let determinant_a = (machine.prize_x * machine.button_b.mov_y) as isize - (machine.prize_y * machine.button_b.mov_x) as isize;
    let determinant_b = (machine.prize_y * machine.button_a.mov_x) as isize - (machine.prize_x * machine.button_a.mov_y) as isize;

    if determinant_a % determinant == 0 && determinant_b % determinant == 0 {
        let num_button_a = (determinant_a / determinant) as usize;
        let num_button_b = (determinant_b / determinant) as usize;
        if is_first_part {
            if num_button_a <= MAX_BUTTON_PRESSES && num_button_b <= MAX_BUTTON_PRESSES {
                let num_tokens = num_button_a * BUTTON_A_PRICE + num_button_b * BUTTON_B_PRICE;
                return Some(num_tokens);
            } else {
                return None;
            }
        } else {
            let num_tokens = num_button_a * BUTTON_A_PRICE + num_button_b * BUTTON_B_PRICE;
            return Some(num_tokens);
        }
    }
    None
}

fn main() {
    // Read file and save the content in the buffer
    let file_to_read = File::open("input.txt").unwrap();
    let reader = BufReader::new(file_to_read);
    let mut buffer = Vec::new();
    
    for line in reader.lines() {
        if let Ok(s) = line {
            if !s.is_empty() {
                buffer.push(s);
            }
        }
    }

    let mut i = 0;
    let mut machines: Vec<Machine> = Vec::new();
    while i < buffer.len() {
        let machine_opt = load_machine(&buffer[i], &buffer[i+1], &buffer[i+2]);
        if let Some(machine) = machine_opt {
            machines.push(machine);
        }
        i += 3
    }

    // First part
    let mut total_num_tokens = 0;
    for machine in machines.iter() {
        if let Some(num_tokens_machine) = solve_equation(machine, true) {
            total_num_tokens += num_tokens_machine;
        }
    }
    println!("{}", total_num_tokens);

    // Second part
    pub const INCREMENT: usize = 10000000000000;
    total_num_tokens = 0;
    for machine in machines.iter_mut() {
        machine.prize_x += INCREMENT;
        machine.prize_y += INCREMENT;

        if let Some(num_tokens_machine) = solve_equation(machine, false) {
            total_num_tokens += num_tokens_machine;
        }
    }
    println!("{}", total_num_tokens);
}
