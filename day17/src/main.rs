use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Clone, Copy, Debug)]
pub enum RegisterId {
    A = 0,
    B = 1, 
    C = 2,
    IC = 3
}

impl RegisterId {
    fn from_char(c: char) -> Self {
        match c {
            'A' => RegisterId::A,
            'B' => RegisterId::B,
            'C' => RegisterId::C,
            _ => panic!("Wrong register id")
        }
    }

    fn to_char(&self) -> char {
        match self {
            RegisterId::A => 'A',
            RegisterId::B => 'B',
            RegisterId::C => 'C',
            RegisterId::IC => '_',
        }
    }

    fn to_usize(&self) -> usize {
        match self {
            RegisterId::A => 0,
            RegisterId::B => 1,
            RegisterId::C => 2,
            RegisterId::IC => 3,
        }
    }
}

#[derive(Debug)]
pub struct Register {
    identifier: RegisterId,
    value: usize,
}

impl Register {
    fn new(id: RegisterId, init_value: usize) -> Self {
        Register { identifier: id, value: init_value }
    }

    fn set_value(&mut self, new_value: usize) {
        self.value = new_value;
    }

    fn get_value(&self) -> usize {
        self.value
    }

    fn get_id(&self) -> RegisterId {
        self.identifier
    }
}

pub trait ComboOperandTrait {
    fn take_value(&self, registers: &[Register; NUM_REGISTERS]) -> usize;
}

#[derive(Debug)]
pub enum ComboOperand {
    Op(usize),
    Op4,
    Op5,
    Op6,
}

impl ComboOperand {
    fn new(id: usize) -> Self {
        match id {
            0..4 => ComboOperand::Op(id),
            4 => ComboOperand::Op4,
            5 => ComboOperand::Op5,
            6 => ComboOperand::Op6,
            _ => panic!("Error Combo operand")
        }
    }
}

impl ComboOperandTrait for ComboOperand {
    fn take_value(&self, registers: &[Register; NUM_REGISTERS]) -> usize {
        match self {
            ComboOperand::Op(x) => *x,
            ComboOperand::Op4 => registers[RegisterId::A.to_usize()].get_value(),
            ComboOperand::Op5 => registers[RegisterId::B.to_usize()].get_value(),
            ComboOperand::Op6 => registers[RegisterId::C.to_usize()].get_value(),
        }
    }
}

pub trait Instruction {
    fn execute(&self, registers: &mut [Register; NUM_REGISTERS]);
}

#[derive(Debug)]
pub enum Operation {
    Adv(ComboOperand),
    Bxl(usize),
    Bst(ComboOperand),
    Jnz(usize),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand)
}

impl Instruction for Operation {
    fn execute(&self, registers: &mut [Register; NUM_REGISTERS]) {
        /*println!("BEFORE OP {:?}", self);
        println!("REGISTERS: {:?}", registers);*/
        match self {
            Operation::Adv(combo_operand) => {
                let base: usize = 2;
                let denominator = base.pow(combo_operand.take_value(registers) as u32);
                let value = registers[RegisterId::A.to_usize()].get_value() / denominator;
                registers[RegisterId::A.to_usize()].set_value(value);
                registers[RegisterId::IC.to_usize()].set_value(registers[RegisterId::IC.to_usize()].get_value() + 1);
            },
            Operation::Bxl(lit) => {
                registers[RegisterId::B.to_usize()].set_value(registers[RegisterId::B.to_usize()].get_value() ^ lit);
                registers[RegisterId::IC.to_usize()].set_value(registers[RegisterId::IC.to_usize()].get_value() + 1);
            },
            Operation::Bst(combo_operand) => {
                registers[RegisterId::B.to_usize()].set_value(combo_operand.take_value(registers) % 8);
                registers[RegisterId::IC.to_usize()].set_value(registers[RegisterId::IC.to_usize()].get_value() + 1);
            },
            Operation::Jnz(lit) => {
                if registers[RegisterId::A.to_usize()].get_value() != 0 {
                    let new_ic = lit / 2;
                    registers[RegisterId::IC.to_usize()].set_value(new_ic);
                } else {
                    registers[RegisterId::IC.to_usize()].set_value(registers[RegisterId::IC.to_usize()].get_value() + 1);
                }
            },
            Operation::Bxc => {
                registers[RegisterId::B.to_usize()].set_value(registers[RegisterId::B.to_usize()].get_value() ^ registers[RegisterId::C.to_usize()].get_value());
                registers[RegisterId::IC.to_usize()].set_value(registers[RegisterId::IC.to_usize()].get_value() + 1);
            },
            Operation::Out(combo_operand) => {
                let value = combo_operand.take_value(registers) % 8;
                print!("{},", value);
                registers[RegisterId::IC.to_usize()].set_value(registers[RegisterId::IC.to_usize()].get_value() + 1);
            },
            Operation::Bdv(combo_operand) => {
                let base: usize = 2;
                let denominator = base.pow(combo_operand.take_value(registers) as u32);
                let value = registers[RegisterId::A.to_usize()].get_value() / denominator;
                registers[RegisterId::B.to_usize()].set_value(value);
                registers[RegisterId::IC.to_usize()].set_value(registers[RegisterId::IC.to_usize()].get_value() + 1);
            },
            Operation::Cdv(combo_operand) => {
                let base: usize = 2;
                let denominator = base.pow(combo_operand.take_value(registers) as u32);
                let value = registers[RegisterId::A.to_usize()].get_value() / denominator;
                registers[RegisterId::C.to_usize()].set_value(value);
                registers[RegisterId::IC.to_usize()].set_value(registers[RegisterId::IC.to_usize()].get_value() + 1);
            },
        }
    }
}

const NUM_REGISTERS: usize = 3 + 1; //Considering the instruction counter

fn parse_file(reader: BufReader<File>, registers: &mut [Register; NUM_REGISTERS], operations: &mut Vec<Operation>) {
    for line in reader.lines() {
        let mut id: char = 'A';
        let mut value: usize = 0;
        if let Ok(line) = line {
            if line.starts_with("Register") {
                let portions_opt = line.split_once(':');
                if let Some(portions) = portions_opt {
                    let id_part = portions.0.trim();
                    let id_opt = id_part.strip_prefix("Register ");
                    if let Some(id_str) = id_opt {
                        id = id_str.chars().next().expect("Expected a single character in id_str");
                    }

                    let value_part = portions.1.trim();
                    value = value_part.parse::<usize>().expect("Failed to parse value as usize");
                }

                let id_reg = RegisterId::from_char(id);
                registers[id_reg.to_usize()].set_value(value);
            } else if line.starts_with("Program:") {
                let program_opt = line.strip_prefix("Program: ");
                if let Some(program) = program_opt {
                    let mut i = 0;
                    let program = program.trim();
                    while i < program.len() {
                        let opcode = program.chars().nth(i).unwrap().to_string().parse::<usize>().expect("Failed to parse value as usize");
                        i += 2;
                        let operand = program.chars().nth(i).unwrap().to_string().parse::<usize>().expect("Failed to parse value as usize");

                        let operation = match opcode {
                            0 => Operation::Adv(ComboOperand::new(operand)),
                            1 => Operation::Bxl(operand),
                            2 => Operation::Bst(ComboOperand::new(operand)),
                            3 => Operation::Jnz(operand),
                            4 => Operation::Bxc,
                            5 => Operation::Out(ComboOperand::new(operand)),
                            6 => Operation::Bdv(ComboOperand::new(operand)),
                            7 => Operation::Cdv(ComboOperand::new(operand)),
                            _ => panic!("Wrong operation")
                        };

                        operations.push(operation);
                        i += 2;
                    }
                }
            } else {
                continue;
            }
        }
    }
}

fn main() {
    let file = File::open("input2.txt").unwrap();

    let reader = BufReader::new(file);

    let mut registers: [Register; NUM_REGISTERS] = [
        Register::new(RegisterId::A, 0),
        Register::new(RegisterId::B, 0),
        Register::new(RegisterId::C, 0),
        Register::new(RegisterId::IC, 0)
    ];

    let mut operations: Vec<Operation> = Vec::new();

    parse_file(reader, &mut registers, &mut operations);

    let mut i = registers[RegisterId::IC.to_usize()].get_value();

    //println!("{:?}", operations);

    while i < operations.len() {
        operations[i].execute(&mut registers);
        i = registers[RegisterId::IC.to_usize()].get_value();
    }
}