use std::{fs::File, io::{BufRead, BufReader}};
use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    is_end_of_word: bool,
    children: HashMap<char, TrieNode>,
}

#[derive(Default, Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default();
        }
        current_node.is_end_of_word = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut current_node = &self.root;

        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                None => return false,
            }
        }

        current_node.is_end_of_word
    }
}

fn parse_file(reader: &mut BufReader<File>, patterns: &mut Vec<String>, designs: &mut Vec<String>) {
    let mut buf: String = String::new();
    if let Ok(_) = reader.read_line(&mut buf) {
        let tmp_patterns: Vec<&str> = buf.split(", ").collect();
        for p in tmp_patterns {
            patterns.push(p.trim().to_string());
        }
    }

    for line in reader.lines() {
        if let Ok(s) = line {
            if s.is_empty() {
                continue;
            }
            designs.push(s.trim().to_string());
        }
    }
}

fn build_trie(patterns: &Vec<String>) -> Trie {
    let mut trie_root = Trie::new();
    
    for pattern in patterns {
        trie_root.insert(pattern);
    }

    trie_root
}

fn find_possible_designs(designs: &Vec<String>, pattern_trie: &Trie) -> usize {
    let mut count = 0;
    
    for design in designs.iter() {
        let design_len = design.len();
        let mut i = 0;

        // This vector will hold the number of ways to reach a character in the design
        // The first element is 1 because there is one way to reach the start of the design
        let mut num_ways: Vec<usize> = vec![0; design_len + 1];
        num_ways[0] = 1;

        while i < design_len {
            let mut j = i + 1;
            while j <= design_len {
                let sub_design = &design[i..j];
                if pattern_trie.contains(sub_design) {
                    num_ways[j] += 1 * num_ways[i];
                }
                j += 1;
            }
            i += 1;
        }
        if num_ways[design_len] > 0 {
            count += 1;
        }
    }

    count
}

fn main() {
    let file = File::open("input.txt").unwrap();

    let mut reader = BufReader::new(file);
    let mut patterns: Vec<String> = Vec::new();
    let mut designs: Vec<String> = Vec::new();

    parse_file(&mut reader, &mut patterns, &mut designs);

    let trie = build_trie(&patterns);

    let count = find_possible_designs(&designs, &trie);
    println!("Number of possible designs: {}", count);
}
