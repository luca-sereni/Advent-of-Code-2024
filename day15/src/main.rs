use std::{fs::File, io::{BufRead, BufReader}};

fn move_robot(grid: &mut Vec<Vec<char>>, m: char, position: &mut(usize, usize)) {
    if grid[position.0][position.1] == '.' {
        return;
    }
    match m {
        '^' => {
            // UP
            if grid[position.0 - 1][position.1] == '#' {
                return;
            }
            move_robot(grid, m, &mut(position.0 - 1, position.1));
            if grid[position.0 - 1][position.1] == '.' {
                let char_to_move = grid[position.0][position.1];
                grid[position.0][position.1] = '.';
                grid[position.0 - 1][position.1] = char_to_move;
                if char_to_move == '@' {
                    position.0 = position.0 - 1;
                }
            }
        },
        'v' => {
            // DOWN
            if grid[position.0 + 1][position.1] == '#' {
                return;
            }
            move_robot(grid, m, &mut(position.0 + 1, position.1));
            if grid[position.0 + 1][position.1] == '.' {
                let char_to_move = grid[position.0][position.1];
                grid[position.0][position.1] = '.';
                grid[position.0 + 1][position.1] = char_to_move;
                if char_to_move == '@' {
                    position.0 = position.0 + 1;
                }
            }
        },
        '<' => {
            // LEFT
            if grid[position.0][position.1 - 1] == '#' {
                return;
            }
            move_robot(grid, m, &mut(position.0, position.1 - 1));
            if grid[position.0][position.1 - 1] == '.' {
                let char_to_move = grid[position.0][position.1];
                grid[position.0][position.1] = '.';
                grid[position.0][position.1 - 1] = char_to_move;
                if char_to_move == '@' {
                    position.1 = position.1 - 1;
                }
            }
        },
        '>' => {
            // RIGHT
            if grid[position.0][position.1 + 1] == '#' {
                return;
            }
            move_robot(grid, m, &mut (position.0, position.1 + 1));
            if grid[position.0][position.1 + 1] == '.' {
                let char_to_move = grid[position.0][position.1];
                grid[position.0][position.1] = '.';
                grid[position.0][position.1 + 1] = char_to_move;
                if char_to_move == '@' {
                    position.1 = position.1 + 1;
                }
            }
        },
        _ => ()
    } 
}

fn apply_moves(grid: &mut Vec<Vec<char>>, moves: &Vec<String>, init_position: &mut (usize, usize)) {
    for m_string in moves.iter() {
        for m in m_string.chars() {
            move_robot(grid, m, init_position);
        }
    }
}

fn compute_lanternfish(grid: &Vec<Vec<char>>) -> usize {
    let mut lanternfish = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                let temp = 100 * i + j;
                lanternfish += temp;
            }
        }
    }
    lanternfish
}

fn main() {
    let file_to_read = File::open("input.txt").unwrap();
    let reader = BufReader::new(file_to_read);

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<String> = Vec::new();

    let mut in_moves_part = false;
    for line_result in reader.lines() {
        if let Ok(line) = line_result {

            if !in_moves_part && line.trim().is_empty() {
                // Switch to part2 after encountering the first blank line
                in_moves_part = true;
                continue; // skip the blank line itself
            }

            if in_moves_part {
                moves.push(line.trim().to_string());
            } else {
                let mut line_vec: Vec<char> = Vec::new();
                for c in line.trim().chars() {
                    line_vec.push(c);
                }
                grid.push(line_vec);
            }
        }
    }

    let mut starting_position: (usize, usize) = {
        let mut x: usize = 0;
        let mut y: usize = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '@' {
                    x = i;
                    y = i;
                    break;
                }
            }
        }
        (x, y)
    };

    apply_moves(&mut grid, &moves, &mut starting_position);
    println!("{}", compute_lanternfish(&grid));
}
