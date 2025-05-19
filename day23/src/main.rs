use std::collections::{HashMap, HashSet};

const NUM_ELEMENTS: usize = 3;

fn find_subsets(
    graphs: &HashMap<&str, Vec<&str>>,
    node: &str,
    current_triangle: Vec<String>,
    triangles: &mut HashSet<Vec<String>>,
    num_iterations: usize,
) {
    if num_iterations == NUM_ELEMENTS {
        return;
    }

    for neighbor in graphs.get(node).unwrap() {
        if current_triangle.contains(&neighbor.to_string()) {
            continue;
        }

        let is_valid =  {
            let mut is_valid = true;
            for elem in current_triangle.iter() {
                if !graphs.get(neighbor).unwrap().contains(&elem.as_str()) {
                    is_valid = false;
                    break;
                }
            }
            is_valid
        };
        if !is_valid {
            continue;
        }
        let mut new_triangle = current_triangle.clone();
        new_triangle.push(neighbor.to_string());
        let len = new_triangle.len();

        if len == NUM_ELEMENTS {
            new_triangle.sort();
            triangles.insert(new_triangle);
        } else {
            find_subsets(graphs, neighbor, new_triangle, triangles, num_iterations + 1);
        }
    }
}

fn part1(triangles: &HashSet<Vec<String>>) -> usize {
    let mut counter = 0;
    for triangle in triangles.iter() {
        for node in triangle.iter() {
            if node.starts_with("t") {
                counter += 1;
                break;
            }
        }
    }
    counter
}

fn main() {
    let input = include_str!("../input.txt");

    let lines = input.lines().collect::<Vec<&str>>();

    let mut graphs: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines.iter() {
        let (host1, host2) = line.split_once('-').unwrap();
        graphs.entry(host1.trim()).or_insert(Vec::new()).push(host2.trim());
        graphs.entry(host2.trim()).or_insert(Vec::new()).push(host1.trim());
    }

    let nodes: Vec<&str> = graphs.keys().cloned().collect();

    // PART 1
    let mut triangles = HashSet::new();
    for node in nodes.iter() {
        find_subsets(&graphs, node, vec![node.to_string()], &mut triangles, 1);
    }

    println!("Subsets with names that starts with t: {}", part1(&triangles));
}
