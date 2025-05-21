#[derive(Debug, Clone, PartialEq)]
pub struct Heights(usize, usize, usize, usize, usize);
impl Heights {
    fn new(height0: usize, height1: usize, height2: usize, height3: usize, height4: usize) -> Self {
        Heights(height0, height1, height2, height3, height4)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lock {
    pub heights: Heights,
}

impl Lock {
    pub fn new(heights: Heights) -> Self {
        Lock { heights }
    }

    pub fn get_heights(&self) -> &Heights {
        &self.heights
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Key {
    pub heights: Heights,
}

impl Key {
    pub fn new(heights: Heights) -> Self {
        Key { heights }
    }

    pub fn get_heights(&self) -> &Heights {
        &self.heights
    }

    fn try_key(&self, lock: &Lock) -> bool {
        // Check if the key can fit into the lock
        let key_heights = self.get_heights();
        let lock_heights = lock.get_heights();

        // Check if the key matches the lock
        (key_heights.0 + lock_heights.0 <= 5) &&
        (key_heights.1 + lock_heights.1 <= 5) &&
        (key_heights.2 + lock_heights.2 <= 5) &&
        (key_heights.3 + lock_heights.3 <= 5) &&
        (key_heights.4 + lock_heights.4 <= 5)
    }
}

const HEIGHT_GRID: usize = 7;
const WIDTH_GRID: usize = 5;

fn parse_lock(grid: &Vec<String>) -> Lock {
    let mut heights: Heights = Heights::new(0, 0, 0, 0, 0);
    for i in 0..HEIGHT_GRID {
        for j in 0..WIDTH_GRID {
            if grid[i].chars().nth(j).unwrap() == '#' {
                match j {
                    0 => heights.0 = i,
                    1 => heights.1 = i,
                    2 => heights.2 = i,
                    3 => heights.3 = i,
                    4 => heights.4 = i,
                    _ => {}
                }
            }
        }
    }
    Lock::new(heights)
}

fn parse_key(grid: &Vec<String>) -> Key {
    let mut heights: Heights = Heights::new(0, 0, 0, 0, 0);
    for i in (0..HEIGHT_GRID - 1).rev() {
        for j in 0..WIDTH_GRID {
            if grid[i].chars().nth(j).unwrap() == '#' {
                match j {
                    0 => heights.0 = HEIGHT_GRID - 1 - i,
                    1 => heights.1 = HEIGHT_GRID - 1 - i,
                    2 => heights.2 = HEIGHT_GRID - 1 - i,
                    3 => heights.3 = HEIGHT_GRID - 1 - i,
                    4 => heights.4 = HEIGHT_GRID - 1 - i,
                    _ => {}
                }
            }
        }
    }
    Key::new(heights)
}

fn main() {
    let input = include_str!("../input.txt");
    let mut locks: Vec<Lock> = Vec::new();
    let mut keys: Vec<Key> = Vec::new();
    let mut temp_grid: Vec<String> = Vec::new();
    for line in input.lines() {
        if !line.is_empty() {
            temp_grid.push(line.to_string());
        } else {
            if temp_grid[0] == "#####" {
                locks.push(parse_lock(&temp_grid));
            } else if temp_grid[0] == "....." {
                keys.push(parse_key(&temp_grid));
            }
            temp_grid.clear();
        }
    }
    // To cover also the last lock/key
    if temp_grid[0] == "#####" {
        locks.push(parse_lock(&temp_grid));
    } else if temp_grid[0] == "....." {
        keys.push(parse_key(&temp_grid));
    }

    let mut lock_key_set: Vec<(&Lock, &Key)> = Vec::new();
    for key in &keys {
        for lock in &locks {
            if key.try_key(lock) && !lock_key_set.contains(&(lock, key)) {
                lock_key_set.push((lock, key));
            }
        }
    }
    println!("Number of unique lock-key pairs: {}", lock_key_set.len());
}
