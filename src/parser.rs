use std::collections::VecDeque;
use std::io::prelude::*;
use std::{
    fs::File,
    io::{self, BufRead},
};

pub struct Parser {
    current_instruction: String,
    instruction_type: Option<InstructionType>,
    instructions: VecDeque<String>,
    symbol: String,
    dest: Option<String>,
    comp: String,
    jump: Option<String>,
}

impl Parser {
    pub fn new(filename: &str) -> Self {
        let mut file = File::open(filename).unwrap();
        let mut buffer = String::new();
        let mut tokens = VecDeque::new();
        let mut token = "";

        file.read_to_string(&mut buffer);

        let mut lines = buffer.lines();

        while let Some(line) = lines.next() {
            if !line.starts_with("//") && !line.is_empty() {
                if line.contains(char::is_whitespace) {
                    token = match line.split_whitespace().next() {
                        None => line.trim(),
                        Some(val) => val.trim(),
                    };
                    tokens.push_back(token.to_string());

                    continue;
                }
                tokens.push_back(line.trim().to_string());
            }
        }

        println!("FILE: {:?}", tokens);

        Parser {
            current_instruction: "".to_string(),
            instruction_type: None,
            instructions: tokens,
            symbol: "".to_string(),
            dest: None,
            comp: "".to_string(),
            jump: None,
        }
    }

    pub fn advance(&mut self) -> Result<(), std::io::Error> {
        match self.instructions.pop_front() {
            Some(token) => {
                if token.starts_with("@") {
                    self.instruction_type = Some(InstructionType::A);
                    self.current_instruction = token.to_string();
                    self.handle_instruction();
                    return Ok(());
                }

                if token.starts_with("(") {
                    self.instruction_type = Some(InstructionType::L);
                    self.current_instruction = token.to_string();
                    self.handle_instruction();
                    return Ok(());
                }

                self.instruction_type = Some(InstructionType::C);
                self.current_instruction = token.to_string();
                self.handle_instruction();
            }

            None => panic!("There are no more lines to parse."),
        }

        Ok(())
    }

    pub fn symbol(&self) -> String {
        self.symbol.clone()
    }

    pub fn dest(&self) -> Option<String> {
        self.dest.clone()
    }

    pub fn comp(&self) -> String {
        self.comp.clone()
    }

    pub fn jump(&self) -> Option<String> {
        self.jump.clone()
    }

    pub fn instruction_type(&self) -> &Option<InstructionType> {
        &self.instruction_type
    }

    pub fn has_more_instructions(&mut self) -> bool {
        self.instructions.len() != 0
    }

    fn handle_instruction(&mut self) {
        match self.instruction_type {
            Some(InstructionType::A) => {
                let mut c1 = self.current_instruction.chars().peekable();

                c1.next();
                self.symbol = c1.collect();
            }
            Some(InstructionType::L) => {
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
            Some(InstructionType::C) => {
                let mut c1 = self.current_instruction.chars().peekable();
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
                }
                self.dest = if dest.is_empty() { None } else { Some(dest) };

                while c1.peek() != Some(&';') {
                    match c1.next() {
                        None => break,
                        Some(c) => comp.push(c),
                    }
                }
                self.comp = comp;

                while let Some(c) = c1.next() {
                    match c {
                        val if val == ';' => {
                            continue;
                        }
                        val2 => jump.push(val2),
                    }
                }

                self.jump = if jump.is_empty() { None } else { Some(jump) }
            }
            (_) => panic!("Incorrect instruction type"),
        }
    }
}

pub enum InstructionType {
    A,
    C,
    L,
}
