use std::collections::VecDeque;
use std::io::prelude::*;
use std::{
    fs::File,
    io::{self, BufRead},
};

pub struct Parser {
    current_instruction: String,
    instruction_type: InstructionType,
    instructions: VecDeque<String>,
    symbol: String,
    dest: String,
    comp: String,
    jump: String,
}

impl Parser {
    pub fn new(filename: &str) -> Self {
        let mut file = File::open(filename).unwrap();
        let mut buffer = String::new();
        let mut tokens = VecDeque::new();

        file.read_to_string(&mut buffer);

        let mut lines = buffer.lines();

        while let Some(line) = lines.next() {
            if !line.starts_with("//") && !line.is_empty() {
                tokens.push_back(line.to_string());
            }
        }
        println!("FILE: {:?}", tokens);

        Parser {
            current_instruction: "".to_string(),
            instruction_type: InstructionType::None,
            instructions: tokens,
            symbol: "".to_string(),
            dest: "".to_string(),
            comp: "".to_string(),
            jump: "".to_string(),
        }
    }

    pub fn advance(&mut self) -> Result<(), std::io::Error> {
        match self.instructions.pop_front() {
            Some(token) => {
                if token.starts_with("@") {
                    self.instruction_type = InstructionType::A_INSTRUCTION;
                    self.current_instruction = token.to_string();
                    println!("{}", self.current_instruction);
                    self.handle_instruction();
                    println!("{}", self.symbol);
                    return Ok(());
                }

                if token.starts_with("(") {
                    self.instruction_type = InstructionType::L_INSTRUCTION;
                    self.current_instruction = token.to_string();
                    println!("{}", self.current_instruction);
                    self.handle_instruction();
                    println!("{}", self.symbol);
                    return Ok(());
                }

                self.instruction_type = InstructionType::C_INSTRUCTION;
                self.current_instruction = token.to_string();
                println!("{}", self.current_instruction);
                self.handle_instruction();
                println!("C_INSTRUCTION- dest: {}", self.dest);
                println!("C_INSTRUCTION- comp: {}", self.comp);
            }

            None => panic!("There are no more lines to parse."),
        }

        Ok(())
    }

    pub fn symbol(&self) -> String {
        self.symbol.clone()
    }

    pub fn dest(&self) -> String {
        self.dest.clone()
    }

    pub fn comp(&self) -> String {
        self.comp.clone()
    }

    pub fn jump(&self) -> String {
        self.jump.clone()
    }

    pub fn has_more_instructions(&mut self) -> bool {
        self.instructions.len() != 0
    }

    fn handle_instruction(&mut self) {
        match self.instruction_type {
            InstructionType::A_INSTRUCTION => {
                let mut c1 = self.current_instruction.chars();

                c1.next();
                self.symbol = c1.as_str().to_string();
            }
            InstructionType::L_INSTRUCTION => {
                let mut c1 = self.current_instruction.chars();
                let start = 1;
                let mut end = 0;

                while let Some(c) = c1.next() {
                    if c != '(' && c != ')' {
                        end += 1;
                    }
                }
                self.symbol = self.current_instruction[start..=end].to_string();
            }
            InstructionType::C_INSTRUCTION => {
                let mut c1 = self.current_instruction.chars();
                let mut dest = "".to_string();
                let mut comp = "".to_string();
                let mut jump = "".to_string();

                if self.current_instruction.contains("=") {
                    while let Some(c) = c1.next() {
                        if c == '=' {
                            break;
                        }
                        dest.push(c);
                    }

                    self.dest = dest;
                }

                while let Some(c) = c1.next() {
                    if c == ';' || c == ' ' {
                        break;
                    }
                    comp.push(c);
                }
                self.comp = comp;

                while let Some(c) = c1.next() {
                    jump.push(c);
                }

                self.jump = jump;
            }
            (_) => panic!("Incorrect instruction type"),
        }
    }
}

enum InstructionType {
    A_INSTRUCTION,
    C_INSTRUCTION,
    L_INSTRUCTION,
    None,
}
