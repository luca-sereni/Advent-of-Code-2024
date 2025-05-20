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

#[derive(Debug)]
pub struct Adder {
    input_x: String,
    input_y: String,
    input_carry: String,
    and_input_signals: String,
    xor_input_signals: String,
    and_before_carry: String,
    output: String,
    output_carry: String,
}

impl Adder {
    fn new(input_x: &str, input_y: &str, input_carry: &str, and_input_signals: &str, xor_input_signals: &str, and_before_carry: &str, output: &str, output_carry: &str) -> Self {
        Adder {
            input_x: input_x.to_string(),
            input_y: input_y.to_string(),
            input_carry: input_carry.to_string(),
            and_input_signals: and_input_signals.to_string(),
            xor_input_signals: xor_input_signals.to_string(),
            and_before_carry: and_before_carry.to_string(),
            output: output.to_string(),
            output_carry: output_carry.to_string(),
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

fn part2(wires: &HashSet<Wire>, operations: &Vec<Operation>) {
    let mut x_values = wires
        .iter()
        .filter(|wire| wire.name.starts_with("x"))
        .collect::<Vec<&Wire>>();
    let mut y_values = wires
        .iter()
        .filter(|wire| wire.name.starts_with("y"))
        .collect::<Vec<&Wire>>();

    let mut z_values = wires
        .iter()
        .filter(|wire| wire.name.starts_with("z"))
        .collect::<Vec<&Wire>>();

    z_values.sort_by(|a, b| a.name.cmp(&b.name));
    x_values.sort_by(|a, b| a.name.cmp(&b.name));
    y_values.sort_by(|a, b| a.name.cmp(&b.name));

    let mut adders: Vec<Adder> = Vec::new();

    let carry_0_opt = operations
        .iter()
        .filter_map(|op| {
            if let Operation::And(a, b, c) = op {
                if a == &x_values[0].name && b == &y_values[0].name {
                    Some(c)
                } else if a == &y_values[0].name && b == &x_values[0].name {
                    Some(c)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<&String>>();
    let carry_0 = carry_0_opt[0].to_string();
    let mut prev_carry = carry_0.clone();

    // Create the adders and where an error occurs, it means that the wires should be swapped
    for i in 1..x_values.len() {
        let and_input_signals_option = operations.iter()
            .filter_map(|op| {
                if let Operation::And(a, b, c) = op {
                    if a == &x_values[i].name && b == &y_values[i].name {
                        Some(c)
                    } else if a == &y_values[i].name && b == &x_values[i].name {
                        Some(c)
                    }else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<&String>>();
        let and_input_signals = and_input_signals_option[0];
        let xor_input_signals_option = operations.iter()
            .filter_map(|op| {
                if let Operation::Xor(a, b, c) = op {
                    if a == &x_values[i].name && b == &y_values[i].name {
                        Some(c)
                    } else if a == &y_values[i].name && b == &x_values[i].name {
                        Some(c)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<&String>>();
        let xor_input_signals = xor_input_signals_option[0];
        println!("xor_input_signals: {:?}", xor_input_signals);
        let and_before_carry_option = operations.iter()
            .filter_map(|op| {
                if let Operation::And(a, b, c) = op {
                    if a == &xor_input_signals.to_string() && b == &prev_carry.to_string() {
                        Some(c)
                    } else if a == &prev_carry.to_string() && b == &xor_input_signals.to_string() {
                        Some(c)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<&String>>();
        let and_before_carry = and_before_carry_option[0];
        let output_option = operations.iter()
            .filter_map(|op| {
                if let Operation::Xor(a, b, c) = op {
                    if a == &prev_carry.to_string() && b == &xor_input_signals.to_string() {
                        Some(c)
                    } else if a == &xor_input_signals.to_string() && b == &prev_carry.to_string() {
                        Some(c)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<&String>>();
        println!("{:?}", output_option);
        let output = output_option[0];
        let output_carry_option = operations.iter()
            .filter_map(|op| {
                if let Operation::Or(a, b, c) = op {
                    if a == &and_before_carry.to_string() && b == &and_input_signals.to_string() {
                        Some(c)
                    } else if a == &and_input_signals.to_string() && b == &and_before_carry.to_string() {
                        Some(c)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<&String>>();
        let output_carry = output_carry_option[0];
        let adder = Adder::new(
            &x_values[i].name,
            &y_values[i].name,
            &prev_carry,
            and_input_signals,
            xor_input_signals,
            and_before_carry,
            output,
            output_carry,
        );
        adders.push(adder);
        prev_carry = output_carry.to_string();
        //println!("Adders: {:?}", adders);
    }
    
    // To check if the swaps work correctly
    /*let mut x_value: usize = 0;
    let mut y_value: usize = 0;
    let mut z_value: usize = 0;
    for i in 0..x_values.len() {
        if let Some(value) = x_values[i].value {
            x_value += (value as usize) << i;
        }
        if let Some(value) = y_values[i].value {
            y_value += (value as usize) << i;
        }
    }
    for i in 0..z_values.len() {
        if let Some(value) = z_values[i].value {
            z_value += (value as usize) << i;
        }
    }

    let final_value = x_value + y_value;
    println!("X value: 0{:0b}", x_value);
    println!("Y value: 0{:0b}", y_value);
    println!("Z value: {:b}", z_value);
    println!("F value: {:b}", final_value);

    let mut carries: Vec<u8> = Vec::new();
    for _ in 0..z_values.len() {
        carries.push(0);
    }

    let mut incorrect_wires_names: Vec<String> = Vec::new();

    for i in 0..x_values.len() {
        if x_values[i].value.unwrap() + y_values[i].value.unwrap() + carries[i] > 1 {
            carries[i + 1] = 1;
        }
        let temp = (x_values[i].value.unwrap() + y_values[i].value.unwrap() + carries[i]) % 2;

        if temp == z_values[i].value.unwrap() {
            println!("Wire {} is correct", z_values[i].name);
        } else {
            println!("Wire {} is incorrect", z_values[i].name);
            incorrect_wires_names.push(z_values[i].name.clone());
        }
    }

    for name in incorrect_wires_names {
        let related_operations = operations
            .iter()
            .filter(|op| {
                if let Operation::And(_, _, c) = op {
                    c == &name
                } else if let Operation::Or(_, _, c) = op {
                    c == &name
                } else if let Operation::Xor(_, _, c) = op {
                    c == &name
                } else {
                    false
                }
            })
            .collect::<Vec<&Operation>>();

        println!("Related operations for wire {}: {:?}", name, related_operations);
    }*/
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

    // PART 2
    part2(&wires, &operations);
}
