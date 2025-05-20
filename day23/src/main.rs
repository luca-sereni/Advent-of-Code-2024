use std::collections::{HashMap, HashSet};

const NUM_ELEMENTS: usize = 3;

fn find_subsets(
    graphs: &HashMap<&str, Vec<&str>>,
    node: &str,
    current_subset: Vec<String>,
    subsets: &mut HashSet<Vec<String>>,
    num_iterations: usize,
) {
    if num_iterations == NUM_ELEMENTS {
        return;
    }

    for neighbor in graphs.get(node).unwrap() {
        if current_subset.contains(&neighbor.to_string()) {
            continue;
        }

        let is_valid =  {
            let mut is_valid = true;
            for elem in current_subset.iter() {
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

        let mut new_subset = current_subset.clone();
        new_subset.push(neighbor.to_string());
        let len = new_subset.len();

        if len == NUM_ELEMENTS {
            new_subset.sort();
            subsets.insert(new_subset);
        } else {
            find_subsets(graphs, neighbor, new_subset, subsets, num_iterations + 1);
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

fn bron_kerbosch<'a>(graphs: &HashMap<&'a str, Vec<&'a str>>, current_clique: HashSet<&'a str>, nodes: HashSet<&'a str>, visited: HashSet<&'a str>, largest_clique: &mut HashSet<&'a str>) {
    if nodes.is_empty() && visited.is_empty() {
        if current_clique.len() > largest_clique.len() {
            *largest_clique = current_clique;
        }
        return;
    }

    let mut new_nodes = nodes.clone();

    for node in nodes.iter() {
        let neighbors = graphs.get(*node).unwrap();
        let neighbors_set: HashSet<&str> = neighbors.iter().cloned().collect();
        let mut new_clique = current_clique.clone();
        new_clique.insert(*node);
        let new_nodes_intersection = new_nodes.intersection(&neighbors_set).cloned().collect::<HashSet<&str>>();
        let new_visited = visited.intersection(&neighbors_set).cloned().collect::<HashSet<&str>>();
        bron_kerbosch(
            graphs,
            new_clique,
            new_nodes_intersection,
            new_visited,
            largest_clique,
        );
        new_nodes.remove(node);
        let mut new_visited = visited.clone();
        new_visited.insert(node);
    }
}

fn part2<'a>(graphs: &HashMap<&'a str, Vec<&'a str>>, nodes: &Vec<&'a str>) {
    let cliques = HashSet::new();
    let nodes: HashSet<&str> = nodes.iter().cloned().collect();
    let visited = HashSet::new();
    let mut largest_clique = HashSet::new();

    bron_kerbosch(graphs, cliques, nodes, visited, &mut largest_clique);

    let mut final_clique = largest_clique.iter().cloned().collect::<Vec<&str>>();
    final_clique.sort();

    for i in 0..final_clique.len() {
        if i == final_clique.len() - 1 {
            println!("{}", final_clique[i]);
        } else {
            print!("{},", final_clique[i]);
        }
    }
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

    // PART 2
    part2(&graphs, &nodes);
}
