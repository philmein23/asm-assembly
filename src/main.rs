mod parser;
use parser::{InstructionType, Parser};

mod code;
use code::Code;

mod symbol_table;
use symbol_table::SymbolTable;

fn is_string_numeric(str: &str) -> bool {
    for c in str.chars() {
        if c.is_alphabetic() {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut parser = Parser::new("Add.asm");
    let mut symbol_table = SymbolTable::new();
    symbol_table.initialize_labels();

    while parser.has_more_instructions() {
        parser.advance();

        match parser.instruction_type() {
            Some(InstructionType::A) => {
                symbol_table.increment_line();

                let symbol = parser.symbol();

                let address = if is_string_numeric(&symbol) {
                    let address = symbol.parse::<usize>().unwrap();
                    address
                } else if !is_string_numeric(&symbol) && !symbol_table.contains(&symbol) {
                    symbol_table.add_entry(symbol.clone(), symbol_table.current_address());
                    symbol_table.increment_address();
                    symbol_table.get_address(&symbol)
                } else {
                    symbol_table.get_address(&symbol)
                };

                let binary = format!("{:015b}", address);
                println!("0{}", binary);
            }
            Some(InstructionType::L) => {
                let symbol = parser.symbol();

                let address = if symbol_table.contains(&symbol) {
                    symbol_table.get_address(&symbol)
                } else {
                    symbol_table.add_entry(symbol.clone(), symbol_table.current_line() + 1);
                    symbol_table.get_address(&symbol)
                };

                let binary = format!("{:015b}", address);
                println!("0{}", binary);
            }
            Some(InstructionType::C) => {
                symbol_table.increment_line();

                let parser_dest = parser.dest();
                let dest = Code::dest(parser_dest.as_str());

                let parser_comp = parser.comp();
                let comp = Code::comp(parser_comp.as_str());

                let parser_jump = parser.jump();
                let jump = Code::jump(parser_jump.as_str());

                println!("111{}{}{}", comp, dest, jump);
            }
            None => eprint!("No specified instruction type"),
        }
    }
}
