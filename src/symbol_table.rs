use std::{collections::HashMap, fmt::format};

pub struct SymbolTable {
    table: HashMap<String, usize>,
    current_line: usize,
    current_address: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            table: HashMap::new(),
            current_line: 0,
            current_address: 16,
        }
    }

    pub fn increment_line(&mut self) {
        self.current_line += 1;
    }

    pub fn increment_address(&mut self) {
        self.current_address += 1;
    }

    pub fn current_line(&self) -> usize {
        self.current_line.clone()
    }

    pub fn current_address(&self) -> usize {
        self.current_address.clone()
    }

    pub fn add_entry(&mut self, label: String, value: usize) {
        self.table.insert(label, value);
    }

    pub fn contains(&self, label: &String) -> bool {
        self.table.contains_key(label)
    }

    pub fn get_address(&self, label: &String) -> usize {
        *self.table.get(label).unwrap()
    }

    pub fn initialize_labels(&mut self) {
        for i in 0..=15 {
            let ram = format!("R{}", i);

            self.table.insert(ram, i);
        }

        self.table.insert("SP".to_string(), 0);
        self.table.insert("LCL".to_string(), 1);
        self.table.insert("ARG".to_string(), 2);
        self.table.insert("THIS".to_string(), 3);
        self.table.insert("THAT".to_string(), 4);
        self.table.insert("SCREEN".to_string(), 16384);
        self.table.insert("KBD".to_string(), 24576);
    }
}
