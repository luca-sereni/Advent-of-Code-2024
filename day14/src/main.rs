use std::{fs::File, io::{BufRead, BufReader}};

pub const GRID_WIDTH: usize = 101;
pub const GRID_HEIGHT: usize = 103;

#[derive(Debug, Clone)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position {
            x,
            y
        }
    }

    fn update_position(&mut self, incr_x: isize, incr_y: isize) {
        let grid_width = GRID_WIDTH as isize;
        let grid_height = GRID_HEIGHT as isize;
    
        self.x = (((((self.x as isize) + incr_x) % grid_width) + grid_width) % grid_width) as usize;
        self.y = (((((self.y as isize) + incr_y) % grid_height) + grid_height) % grid_height) as usize;
    }
}

#[derive(Debug, Clone)]
pub struct Velocity {
    vel_x: isize,
    vel_y: isize,
}

impl Velocity {
    fn new(vel_x: isize, vel_y: isize) -> Self {
        Velocity { vel_x, vel_y }
    }
}

#[derive(Debug, Clone)]
pub struct Robot {
    current_position: Position,
    velocity: Velocity,
}

impl Robot {
    fn new(starting_pos: Position, velocity: Velocity) -> Self {
        Robot { current_position: starting_pos, velocity }
    }

    fn update_position(&mut self, incr_x: isize, incr_y: isize) {
        self.current_position.update_position(incr_x, incr_y);
    }
}

fn parse_robot(line: String) -> Robot {
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut vel_x = 0;
    let mut vel_y = 0;
    if let Some(rest) = line.split_once(' ') {
        let position = rest.0.trim(); 

        for part in position.split(',') {
            let part = part.trim();
            if let Some(stripped) = part.strip_prefix('p') {
                if let Some(stripped) = stripped.strip_prefix('=') {
                    pos_x = stripped.parse::<usize>().unwrap();
                }
            } else {
                pos_y = part.parse::<usize>().unwrap();
            }
        }

        let velocity = rest.1.trim();
        for part in velocity.split(',') {
            let part = part.trim();
            if let Some(stripped) = part.strip_prefix('v') {
                if let Some(stripped) = stripped.strip_prefix('=') {
                    vel_x = stripped.parse::<isize>().unwrap();
                }
            } else {
                vel_y = part.parse::<isize>().unwrap();
            }
        }
    }
    Robot::new(Position::new(pos_x, pos_y), Velocity::new(vel_x, vel_y))
}

fn assign_to_quadrant(quadrants: &mut [usize; 4], robot: &Robot) {
    let middle_vertical_limit = GRID_HEIGHT / 2;
    let middle_horizontal_limit = GRID_WIDTH / 2;
    if robot.current_position.x != middle_horizontal_limit && robot.current_position.y != middle_vertical_limit {
        if robot.current_position.x < middle_horizontal_limit && robot.current_position.y < middle_vertical_limit {
            quadrants[0] += 1;
        } else if robot.current_position.x > middle_horizontal_limit && robot.current_position.y < middle_vertical_limit {
            quadrants[1] += 1;
        } else if robot.current_position.x < middle_horizontal_limit && robot.current_position.y > middle_vertical_limit {
            quadrants[2] += 1;
        } else {
            quadrants[3] += 1;
        }
    }
}

fn print_grid(grid: [[usize; GRID_WIDTH]; GRID_HEIGHT], iteration: usize) {
    use std::io::Write;
    let filename = format!("file{}.txt", iteration);
    let file = File::create(&filename);
    if let Ok(mut file) = file {
        for row in grid.iter() {
            for elem in row.iter() {
                if *elem == 0 {
                    write!(file, ".").unwrap();
                } else {
                    write!(file, "#").unwrap();
                }
            }
            write!(file, "\n").unwrap();
        }
    }
}

const NUM_SECONDS: isize = 100;

fn main() {
    let file_to_read = File::open("input.txt").unwrap();
    let reader = BufReader::new(file_to_read);

    let mut robots = Vec::<Robot>::new();

    // Create robot and add it to the vector
    for line in reader.lines() {
        if let Ok(s) = line {
            robots.push(parse_robot(s));
        }
    }

    let mut robots_part2: Vec<Robot> = Vec::new();
    robots_part2.extend(robots.iter().cloned());
    // Array representing the number of robots inside each quadrant
    let mut quadrants: [usize; 4] = [0, 0, 0, 0];
    
    // Move robot
    for robot in robots.iter_mut() {
        robot.update_position(robot.velocity.vel_x * NUM_SECONDS, robot.velocity.vel_y * NUM_SECONDS);
        assign_to_quadrant(&mut quadrants, robot);
    }

    // Compute safety factor
    let mut safety_factor = 1;
    for i in 0..quadrants.len() {
        safety_factor *= quadrants[i];
    }
    println!("{}", safety_factor);

    // PART2

    let mut grid: [[usize; GRID_WIDTH]; GRID_HEIGHT] = [[0; GRID_WIDTH]; GRID_HEIGHT];
    for robot in robots_part2.iter() {
        grid[robot.current_position.y][robot.current_position.x] += 1;
    }

    // Since no figure of the tree is provided, I checked the first 100 images to see if there is an interesting pattern.
    // Every 103 iterations, all the robots are near each other.
    // 7752 -> 7753 seconds because it starts from 0

    let indexes: [usize; 100] = (0..100).map(|i| 27 + 103 * i).collect::<Vec<_>>().try_into().unwrap();

    for i in 0..105*100 {
        for robot in robots_part2.iter_mut() {
            grid[robot.current_position.y][robot.current_position.x] -= 1;
            robot.update_position(robot.velocity.vel_x, robot.velocity.vel_y);
            grid[robot.current_position.y][robot.current_position.x] += 1;
        }
        if indexes.contains(&i) {
            print_grid(grid, i as usize);
        }
    }
}
