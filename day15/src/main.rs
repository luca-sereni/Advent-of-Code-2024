use std::{fs::File, io::{BufRead, BufReader}};

fn explore_space(grid: &Vec<Vec<char>>, m: char, position: (usize, usize), is_first_part: bool) -> bool {
    if grid[position.0][position.1] == '.' {
        return true;
    }
    if grid[position.0][position.1] == '#' {
        return false;
    }

    match m {
        '^' => {
            // UP
            if grid[position.0][position.1] == '@' || is_first_part {
                return explore_space(grid, m, (position.0 - 1, position.1), is_first_part);
            } else if !is_first_part && grid[position.0][position.1] == '[' {
                return explore_space(grid, m, (position.0 - 1, position.1), is_first_part) && explore_space(grid, m, (position.0 - 1, position.1 + 1), is_first_part);
            } else if !is_first_part && grid[position.0][position.1] == ']' {
                return explore_space(grid, m, (position.0 - 1, position.1), is_first_part) && explore_space(grid, m, (position.0 - 1, position.1 - 1), is_first_part);
            }
        },
        'v' => {
            // DOWN
            if grid[position.0][position.1] == '@' || is_first_part {
                return explore_space(grid, m, (position.0 + 1, position.1), is_first_part);
            } else if !is_first_part && grid[position.0][position.1] == '[' {
                return explore_space(grid, m, (position.0 + 1, position.1), is_first_part) && explore_space(grid, m, (position.0 + 1, position.1 + 1), is_first_part);
            } else if !is_first_part && grid[position.0][position.1] == ']' {
                return explore_space(grid, m, (position.0 + 1, position.1), is_first_part) && explore_space(grid, m, (position.0 + 1, position.1 - 1), is_first_part);
            }
        },
        '<' => {
            // LEFT
            if grid[position.0][position.1] == '@' || is_first_part {
                return explore_space(grid, m, (position.0, position.1 - 1), is_first_part);
            } else if !is_first_part && grid[position.0][position.1] == ']' {
                return explore_space(grid, m, (position.0, position.1 - 2), is_first_part); // Exclude '[' parenthesis
            }
        },
        '>' => {
            // RIGHT
            if grid[position.0][position.1] == '@' || is_first_part {
                return explore_space(grid, m, (position.0, position.1 + 1), is_first_part);
            } else if !is_first_part && grid[position.0][position.1] == '[' {
                return explore_space(grid, m, (position.0, position.1 + 2), is_first_part); // Exclude ']' parenthesis
            }
        },
        _ => ()
    }
    false
}

fn move_robot(grid: &mut Vec<Vec<char>>, m: char, position: &mut(usize, usize), is_first_part: bool) {
    if grid[position.0][position.1] == '.' {
        return;
    }
    match m {
        '^' => {
            // UP
            if !is_first_part && grid[position.0][position.1] == '[' {
                move_robot(grid, m, &mut(position.0 - 1, position.1), is_first_part);
                move_robot(grid, m, &mut(position.0 - 1, position.1 + 1), is_first_part);
                if grid[position.0 - 1][position.1] == '.' && grid[position.0 - 1][position.1 + 1] == '.' {
                    let char_to_move1 = grid[position.0][position.1];
                    let char_to_move2 = grid[position.0][position.1 + 1];
                    grid[position.0][position.1] = '.';
                    grid[position.0][position.1 + 1] = '.';
                    grid[position.0 - 1][position.1] = char_to_move1;
                    grid[position.0 - 1][position.1 + 1] = char_to_move2;
                }
            } else if !is_first_part && grid[position.0][position.1] == ']' {
                move_robot(grid, m, &mut(position.0 - 1, position.1), is_first_part);
                move_robot(grid, m, &mut(position.0 - 1, position.1 - 1), is_first_part);
                if grid[position.0 - 1][position.1] == '.' && grid[position.0 - 1][position.1 - 1] == '.' {
                    let char_to_move1 = grid[position.0][position.1];
                    let char_to_move2 = grid[position.0][position.1 - 1];
                    grid[position.0][position.1] = '.';
                    grid[position.0][position.1 - 1] = '.';
                    grid[position.0 - 1][position.1] = char_to_move1;
                    grid[position.0 - 1][position.1 - 1] = char_to_move2;
                }
            } else {
                // Char is '@'
                move_robot(grid, m, &mut(position.0 - 1, position.1), is_first_part);
                if grid[position.0 - 1][position.1] == '.' {
                    let char_to_move = grid[position.0][position.1];
                    grid[position.0][position.1] = '.';
                    grid[position.0 - 1][position.1] = char_to_move;
                    if char_to_move == '@' {
                        position.0 = position.0 - 1;
                    }
                }
            }
        },
        'v' => {
            // DOWN
            if !is_first_part && grid[position.0][position.1] == '[' {
                move_robot(grid, m, &mut(position.0 + 1, position.1), is_first_part);
                move_robot(grid, m, &mut(position.0 + 1, position.1 + 1), is_first_part);
                if grid[position.0 + 1][position.1] == '.' && grid[position.0 + 1][position.1 + 1] == '.' {
                    let char_to_move1 = grid[position.0][position.1];
                    let char_to_move2 = grid[position.0][position.1 + 1];
                    grid[position.0][position.1] = '.';
                    grid[position.0][position.1 + 1] = '.';
                    grid[position.0 + 1][position.1] = char_to_move1;
                    grid[position.0 + 1][position.1 + 1] = char_to_move2;
                }
            } else if !is_first_part && grid[position.0][position.1] == ']' {
                move_robot(grid, m, &mut(position.0 + 1, position.1), is_first_part);
                move_robot(grid, m, &mut(position.0 + 1, position.1 - 1), is_first_part);
                if grid[position.0 + 1][position.1] == '.' && grid[position.0 + 1][position.1 - 1] == '.' {
                    let char_to_move1 = grid[position.0][position.1];
                    let char_to_move2 = grid[position.0][position.1 - 1];
                    grid[position.0][position.1] = '.';
                    grid[position.0][position.1 - 1] = '.';
                    grid[position.0 + 1][position.1] = char_to_move1;
                    grid[position.0 + 1][position.1 - 1] = char_to_move2;
                }
            } else {
                move_robot(grid, m, &mut(position.0 + 1, position.1), is_first_part);
                if grid[position.0 + 1][position.1] == '.' {
                    let char_to_move = grid[position.0][position.1];
                    grid[position.0][position.1] = '.';
                    grid[position.0 + 1][position.1] = char_to_move;
                    if char_to_move == '@' {
                        position.0 = position.0 + 1;
                    }
                }
            }
        },
        '<' => {
            // LEFT
            move_robot(grid, m, &mut(position.0, position.1 - 1), is_first_part);
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
            move_robot(grid, m, &mut (position.0, position.1 + 1), is_first_part);
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

fn apply_moves(grid: &mut Vec<Vec<char>>, moves: &Vec<String>, init_position: &mut (usize, usize), is_first_part: bool) {
    for m_string in moves.iter() {
        for m in m_string.chars() {
            if explore_space(grid, m, *init_position, is_first_part) {
                move_robot(grid, m, init_position, is_first_part);
            }
        }
    }
}

fn compute_lanternfish(grid: &Vec<Vec<char>>, symbol: char) -> usize {
    let mut lanternfish = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == symbol {
                let temp = 100 * i + j;
                lanternfish += temp;
            }
        }
    }
    lanternfish
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid.iter() {
        for c in line.iter() {
            print!("{}", c);
        }
        print!("\n");
    }
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

    let grid_copy = grid.clone();

    // First part
    apply_moves(&mut grid, &moves, &mut starting_position, true);
    println!("{}", compute_lanternfish(&grid, 'O'));

    // Second part
    let mut new_grid: Vec<Vec<char>> = Vec::new();

    for i in 0..grid_copy.len() {
        new_grid.push(Vec::new());
        for j in 0..grid_copy[0].len() {
            match grid_copy[i][j] {
                '#' => {
                    new_grid[i].push('#');
                    new_grid[i].push('#');
                },
                'O' => {
                    new_grid[i].push('[');
                    new_grid[i].push(']');
                },
                '.' => {
                    new_grid[i].push('.');
                    new_grid[i].push('.');
                },
                '@' => {
                    new_grid[i].push('@');
                    new_grid[i].push('.');
                },
                _ => ()
            }
        }
    }

    let mut starting_position_v2: (usize, usize) = {
        let mut x: usize = 0;
        let mut y: usize = 0;
        for i in 0..new_grid.len() {
            for j in 0..new_grid[0].len() {
                if new_grid[i][j] == '@' {
                    x = i;
                    y = j;
                    break;
                }
            }
        }
        (x, y)
    };

    apply_moves(&mut new_grid, &moves, &mut starting_position_v2, false);

    println!("{}", compute_lanternfish(&new_grid, '['));
}
