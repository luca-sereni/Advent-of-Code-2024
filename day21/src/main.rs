use std::{collections::{HashMap, VecDeque}, fs::File, io::{BufRead, BufReader}};
use cached::proc_macro::cached;
use itertools::Itertools;

pub struct Keypad {
    layout: HashMap<char, Vec<char>>,
    positions: HashMap<char, (isize, isize)>,
}

impl Keypad {
    fn new(layout: HashMap<char, Vec<char>>, positions: HashMap<char, (isize, isize)>) -> Self {
        Keypad { layout, positions }
    }

    fn find_shortest_paths(&self, start: char, end: char) -> Vec<Vec<char>> {
        let mut results: Vec<Vec<char>> = Vec::new();
        let mut queue: VecDeque<Vec<char>> = VecDeque::new();
        let mut visited: HashMap<char, usize> = HashMap::new();
        let mut shortest_length = usize::max_value();

        queue.push_back(vec![start]);
        visited.insert(start, 0);

        while let Some(path) = queue.pop_front() {
            let &last = path.last().unwrap();
            if path.len() > shortest_length {
                continue;
            }

            if last == end {
                shortest_length = path.len();
                results.push(path.clone());
                continue;
            }

            if let Some(neighbors) = self.layout.get(&last) {
                for &next in neighbors {
                    let new_path = {
                        let mut p = path.clone();
                        p.push(next);
                        p
                    };

                    let new_len = path.len();
                    if !visited.contains_key(&next) || new_len <= visited[&next] {
                        visited.insert(next, new_len);
                        queue.push_back(new_path);
                    }
                }
            }
        }
        results
    }

    fn get_directional_moves(&self, start: char, end: char) -> Vec<Vec<char>> {
        let paths = self.find_shortest_paths(start, end);

        let mut temp_moves: Vec<Vec<char>> = Vec::new();

        for path in paths.iter() {
            let mut temp_move: Vec<char> = Vec::new();
            for window in path.windows(2) {
                let (from, to) = (window[0], window[1]);
                let (x1, y1) = self.positions[&from];
                let (x2, y2) = self.positions[&to];

                let dx = x2 - x1;
                let dy = y2 - y1;

                let symbol = match (dx, dy) {
                    (-1,  0) => '^',
                    ( 1,  0) => 'v',
                    ( 0, -1) => '<',
                    ( 0,  1) => '>',
                    _ => '?', // shouldn't happen
                };

                temp_move.push(symbol);
            }
            temp_move.push('A');
            temp_moves.push(temp_move);
        }

        temp_moves
    }
}

const NUM_ROBOTS_DIR_KEYPADS_PART1: isize = 3;
const NUM_ROBOTS_DIR_KEYPADS_PART2: isize = 26;


#[cached]
fn complexity(from: char, to: char, mut robot_count: isize) -> u64 {
    robot_count -= 1;

    let mut num_graph: HashMap<char, Vec<char>> = HashMap::new();
        num_graph.insert('0', vec!['A', '2']);
        num_graph.insert('A', vec!['0', '3']);
        num_graph.insert('1', vec!['2', '4']);
        num_graph.insert('2', vec!['0', '3', '1', '5']);
        num_graph.insert('3', vec!['A', '2', '6']);
        num_graph.insert('4', vec!['1', '5', '7']);
        num_graph.insert('5', vec!['2', '6', '4', '8']);
        num_graph.insert('6', vec!['3', '5', '9']);
        num_graph.insert('7', vec!['4', '8']);
        num_graph.insert('8', vec!['5', '7', '9']);
        num_graph.insert('9', vec!['6', '8']);

    let num_positions: HashMap<char, (isize, isize)> = HashMap::from([
        ('7', (0, 0)), ('8', (0, 1)), ('9', (0, 2)),
        ('4', (1, 0)), ('5', (1, 1)), ('6', (1, 2)),
        ('1', (2, 0)), ('2', (2, 1)), ('3', (2, 2)),
        ('0', (3, 1)), ('A', (3, 2))
    ]);

    let mut dir_graph: HashMap<char, Vec<char>> = HashMap::new();
        dir_graph.insert('<', vec!['v']);
        dir_graph.insert('v', vec!['>', '^', '<']);
        dir_graph.insert('>', vec!['A', 'v']);
        dir_graph.insert('^', vec!['A', 'v']);
        dir_graph.insert('A', vec!['>', '^']);

        let dir_positions: HashMap<char, (isize, isize)> = HashMap::from([
            ('^', (0, 1)), ('A', (0, 2)),
            ('<', (1, 0)), ('v', (1, 1)), ('>', (1, 2))
        ]);

    let keypad = if dir_graph.contains_key(&from) && dir_graph.contains_key(&to) {
        Keypad::new(dir_graph, dir_positions)
    } else {
        Keypad::new(num_graph.clone(), num_positions)
    };

    let paths = keypad.get_directional_moves(from, to);

    if robot_count > 0 {
        return paths
            .iter()
            .map(|path| {
                // prepend 'A' to the path
                let path = format!("A{}", path.iter().collect::<String>());
                path.chars()
                    .tuple_windows()
                    .map(|(a, b)| complexity(a, b, robot_count))
                    .sum::<u64>()
            })
            .min()
            .expect("ERROR");
    }
    paths
        .iter()
        .map(|path| path.len() as u64)
        .min()
        .expect("ERROR")

}

fn compute_complexities(is_part1: bool, codes: &Vec<Vec<char>>) -> u64 {
    let mut complexity_value = 0;
    if is_part1 {
        for code in codes {
            let numeric_prefix: String = code.iter()
            .take(code.len()).filter(|c| c.is_ascii_digit()).take_while(|c| c.is_ascii_digit())
            .collect();
            let num = numeric_prefix.parse::<u64>().unwrap();
            for (start,end) in code.iter().tuple_windows() {
                let c = complexity(*start, *end, NUM_ROBOTS_DIR_KEYPADS_PART1);
                complexity_value += c * num;
            }
        }
    } else {
        for code in codes {
            let numeric_prefix: String = code.iter()
            .take(code.len()).filter(|c| c.is_ascii_digit()).take_while(|c| c.is_ascii_digit())
            .collect();
            let num = numeric_prefix.parse::<u64>().unwrap();
            for (start,end) in code.iter().tuple_windows() {
                let c = complexity(*start, *end, NUM_ROBOTS_DIR_KEYPADS_PART2);
                complexity_value += c * num;
            }
        }
    }

    complexity_value
}

fn main() {
    let file = File::open("input.txt").unwrap();

    let reader = BufReader::new(file);

    let mut codes: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        if let Ok(s) = line {
            codes.push(Vec::new());
            codes.last_mut().unwrap().push('A');
            for c in s.trim().chars() {
                codes.last_mut().unwrap().push(c);
            }
        }
    }

    // Part 1
    println!("{}", compute_complexities(true, &codes));

    // Part 2
    println!("{}", compute_complexities(false, &codes));
}
