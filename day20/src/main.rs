use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

pub const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn parse_file(reader: BufReader<File>, grid: &mut Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
    let mut start_end: ((usize, usize), (usize, usize)) = ((0, 0), (0,0));
    for (i, line) in reader.lines().enumerate() {
        if let Ok(s) = line {
            grid.push(Vec::new());
            for (j, c) in s.chars().enumerate() {
                grid[i].push(c);
                if c == 'S' {
                    start_end.0 = (i, j);
                } else if c == 'E' {
                    start_end.1 = (i, j);
                }
            }
        }
    }
    return start_end;
}

fn dfs(grid: &Vec<Vec<char>>, curr_pos: (usize, usize), end: (usize, usize), visited: &mut Vec<(usize, usize)>, path: &mut HashMap<(usize, usize), usize>, pico: usize) {
    path.insert(curr_pos, pico);
    visited.push(curr_pos);
    if grid[curr_pos.0 as usize][curr_pos.1 as usize] == 'E' {
        return;
    }

    for dir in DIRECTIONS {
        let (dx, dy) = dir;
        let new_x = (curr_pos.0 as isize + dx) as usize;
        let new_y = (curr_pos.1 as isize + dy) as usize;

        if grid[new_x][new_y] != '#' && !visited.contains(&(new_x, new_y)) {
            dfs(grid, (new_x, new_y), end, visited, path, 1 + pico);
        }
    }
}

fn find_cheats(grid: &Vec<Vec<char>>, path: &HashMap<(usize, usize), usize>) -> usize {
    let mut count: usize = 0;
    for (key, value) in path.iter() {
        let curr_pico = *value;
        for (nx, ny) in DIRECTIONS {
            let new_x = key.0 as isize + nx;
            let new_y = key.1 as isize + ny;

            if new_x >= 0 && (new_x as usize) < grid.len() && new_y >= 0 && (new_y as usize) < grid[0].len() {
                if grid[new_x as usize][new_y as usize] == '#' {
                    let next_x = new_x + nx;
                    let next_y = new_y + ny;
                    if next_x >= 0 && (next_x as usize) < grid.len() && next_y >= 0 && (next_y as usize) < grid[0].len() {
                        let new_pico_opt = path.get(&(next_x as usize, next_y as usize));
                        if let Some(new_pico) = new_pico_opt {
                            if curr_pico < *new_pico && (*new_pico - curr_pico) > 100 {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

fn count_cheats(grid: &mut Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut path: HashMap<(usize, usize), usize> = HashMap::new();
    dfs(grid, start, end, &mut Vec::new(), &mut path, 0);
    let count = find_cheats(grid, &path);
    return count;
}

fn main() {
    let file = File::open("input.txt").unwrap();

    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();

    let (start, end) = parse_file(reader, &mut grid);

    // Part 1
    println!("{}", count_cheats(&mut grid, start, end))
}
