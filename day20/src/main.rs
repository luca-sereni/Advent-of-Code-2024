use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

pub const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

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

fn dfs(grid: &Vec<Vec<char>>, curr_pos: (usize, usize), end: (usize, usize), visited: &mut Vec<(usize, usize)>, path: &mut Vec<(usize, usize)>) {
    path.push(curr_pos);
    visited.push(curr_pos);
    if grid[curr_pos.0 as usize][curr_pos.1 as usize] == 'E' {
        return;
    }

    for dir in DIRECTIONS {
        let (dx, dy) = dir;
        let new_x = (curr_pos.0 as isize + dx) as usize;
        let new_y = (curr_pos.1 as isize + dy) as usize;

        if grid[new_x][new_y] != '#' && !visited.contains(&(new_x, new_y)) {
            dfs(grid, (new_x, new_y), end, visited, path);
        }
    }
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

/// Finds the number of valid cheats that save at least `min_saving` units.
fn find_cheats<const CHEAT_RADIUS: usize>(
    grid: &Vec<Vec<char>>,
    path: &Vec<(usize, usize)>,
    min_to_save: usize,
) -> usize {
    let base_time = path.len();

    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    for (i, &pos) in path.iter().enumerate() {
        distances.insert(pos, path.len() - 1 - i);
    }

    let mut cheat_endpoints: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    let mut count = 0;

    for (time_so_far, &pos) in path.iter().enumerate() {
        let (x, y) = (pos.0 as isize, pos.1 as isize);

        for dx in -(CHEAT_RADIUS as isize)..=(CHEAT_RADIUS as isize) {
            for dy in -(CHEAT_RADIUS as isize)..=(CHEAT_RADIUS as isize) {
                if dx.abs() + dy.abs() > CHEAT_RADIUS as isize {
                    continue;
                }

                let new_x = x + dx;
                let new_y = y + dy;

                if new_x < 0 || new_y < 0 {
                    continue;
                }

                let new_pos = (new_x as usize, new_y as usize);

                if new_pos.0 >= grid.len() || new_pos.1 >= grid[0].len() {
                    continue;
                }

                if grid[new_pos.0][new_pos.1] != '.' && grid[new_pos.0][new_pos.1] != 'E' {
                    continue;
                }

                // Ensure the cheat target is on the path to the end
                if let Some(&to_end) = distances.get(&new_pos) {
                    let cheat_cost = manhattan_distance(pos, new_pos);
                    let total_time = time_so_far + cheat_cost + to_end;

                    if total_time < base_time && base_time - total_time >= min_to_save {
                        if !cheat_endpoints.contains(&(pos, new_pos)) {
                            count += 1;
                            cheat_endpoints.insert((pos, new_pos));
                            cheat_endpoints.insert((new_pos, pos));
                        }
                    }
                }
            }
        }
    }

    count
}

pub const PICOSECONDS_TO_SAVE: usize = 100;

fn count_cheats(grid: &mut Vec<Vec<char>>, start: (usize, usize), end: (usize, usize), is_part_1: bool) -> usize {
    let mut path: Vec<(usize, usize)> = Vec::new();
    dfs(grid, start, end, &mut Vec::new(), &mut path);
    let count = if is_part_1 {
        find_cheats::<2>(grid, &path, PICOSECONDS_TO_SAVE)
    } else {
        find_cheats::<20>(grid, &path, PICOSECONDS_TO_SAVE)
    };
    count
}

fn main() {
    let file = File::open("input.txt").unwrap();

    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();

    let (start, end) = parse_file(reader, &mut grid);

    // Part 1
    println!("{}", count_cheats(&mut grid, start, end, true));

    // Part 2
    println!("{}", count_cheats(&mut grid, start, end, false));
}
