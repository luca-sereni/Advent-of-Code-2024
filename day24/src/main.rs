use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum Operation {
    And(String, String, String),
    Or(String, String, String),
    Xor(String, String, String),
}

impl Operation {
    fn execute(&mut self, wires: &mut HashSet<Wire>) -> bool {
        match self {
            Operation::And(a, b, result) => {
                let a_val = wires.get(a.as_str()).and_then(|w| w.value);
                let b_val = wires.get(b.as_str()).and_then(|w| w.value);
                if let (Some(a_val), Some(b_val)) = (a_val, b_val) {
                    if let Some(mut wire_to_modify) = wires.take(result.as_str()) {
                        wire_to_modify.value = Some(a_val & b_val);
                        wires.insert(wire_to_modify);
                        return true;
                    }
                }
                false
            }
            Operation::Or(a, b, result) => {
                let a_val = wires.get(a.as_str()).and_then(|w| w.value);
                let b_val = wires.get(b.as_str()).and_then(|w| w.value);
                if let (Some(a_val), Some(b_val)) = (a_val, b_val) {
                    if let Some(mut wire_to_modify) = wires.take(result.as_str()) {
                        wire_to_modify.value = Some(a_val | b_val);
                        wires.insert(wire_to_modify);
                        return true;
                    }
                }
                false
            }
            Operation::Xor(a, b, result) => {
                let a_val = wires.get(a.as_str()).and_then(|w| w.value);
                let b_val = wires.get(b.as_str()).and_then(|w| w.value);
                if let (Some(a_val), Some(b_val)) = (a_val, b_val) {
                    if let Some(mut wire_to_modify) = wires.take(result.as_str()) {
                        wire_to_modify.value = Some(a_val ^ b_val);
                        wires.insert(wire_to_modify);
                        return true;
                    }
                }
                false
            }
        }
    }
}

#[derive(Debug)]
pub struct Wire {
    name: String,
    value: Option<u8>,
}

impl PartialEq for Wire {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Wire {}

impl Hash for Wire {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

// Implement Borrow<str> so we can use &str as a key
impl Borrow<str> for Wire {
    fn borrow(&self) -> &str {
        &self.name
    }
}

impl Wire {
    fn new(name: &str, value: Option<u8>) -> Self {
        Wire {
            name: name.to_string(),
            value,
        }
    }
}

fn parse_file(input: &str, wires: &mut HashSet<Wire>, operations: &mut Vec<Operation>) {
    let (part1, part2) = input.split_once("\n\n").unwrap();

    for line in part2.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let wire0 = Wire::new(parts[0], None);
        wires.insert(wire0);
        let wire1 = Wire::new(parts[2], None);
        wires.insert(wire1);
        let wire2 = Wire::new(parts[4], None);
        wires.insert(wire2);
        let op = match parts[1] {
            "AND" => Operation::And(
                parts[0].to_string(),
                parts[2].to_string(),
                parts[4].to_string(),
            ),
            "OR" => Operation::Or(
                parts[0].to_string(),
                parts[2].to_string(),
                parts[4].to_string(),
            ),
            "XOR" => Operation::Xor(
                parts[0].to_string(),
                parts[2].to_string(),
                parts[4].to_string(),
            ),
            _ => continue,
        };
        operations.push(op);
    }

    for line in part1.lines() {
        let (name, value) = line.split_once(": ").unwrap();
        if let Some(mut wire_to_modify) = wires.take(name) {
            wire_to_modify.value = Some(value.parse::<u8>().ok().unwrap());
            wires.insert(wire_to_modify);
        }
    }
}

fn part1(wires: &HashSet<Wire>) -> usize {
    let mut output_wires = wires
        .iter()
        .filter(|wire| wire.name.starts_with("z"))
        .collect::<Vec<&Wire>>();

    let mut final_value: usize = 0;

    output_wires.sort_by(|a, b| a.name.cmp(&b.name));

    for (i, wire) in output_wires.iter().enumerate() {
        if let Some(value) = wire.value {
            final_value += (value as usize) << i
        }
    }

    final_value
}

fn main() {
    let input = include_str!("../input.txt");
    let mut wires: HashSet<Wire> = HashSet::new();
    let mut operations: Vec<Operation> = Vec::new();
    parse_file(input, &mut wires, &mut operations);

    let mut remaining_operations = operations.clone();

    while remaining_operations.len() > 0 {
        remaining_operations.clear();
        for op in operations.iter_mut() {
            if !op.execute(&mut wires) {
                remaining_operations.push(op.clone());
            }
        }
    }

    // PART 1
    println!("Part 1: {}", part1(&wires));
}
