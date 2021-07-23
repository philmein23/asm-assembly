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

fn first_pass(table: &mut SymbolTable) {
    let mut parser = Parser::new("MaxL.asm");

    while parser.has_more_instructions() {
        parser.advance();

        match parser.instruction_type() {
            Some(InstructionType::A) => {
                table.increment_line();
            }
            Some(InstructionType::L) => table.add_entry(parser.symbol(), table.current_line()),
            Some(InstructionType::C) => {
                table.increment_line();
            }
            None => eprint!("No specified instruction type"),
        }
    }
}

fn second_pass(table: &mut SymbolTable) {
    let mut parser = Parser::new("MaxL.asm");

    while parser.has_more_instructions() {
        parser.advance();

        match parser.instruction_type() {
            Some(InstructionType::A) => {
                let symbol = parser.symbol();
                let address = if is_string_numeric(&symbol) {
                    let address = symbol.parse::<usize>().unwrap();
                    address
                } else if !is_string_numeric(&symbol) && !table.contains(&symbol) {
                    table.add_entry(symbol.clone(), table.current_address());
                    table.increment_address();
                    table.get_address(&symbol)
                } else {
                    table.get_address(&symbol)
                };

                let binary = format!("{:015b}", address);
                println!("0{}", binary);
            }
            Some(InstructionType::L) => {
                continue;
            }
            Some(InstructionType::C) => {
                let parser_dest = parser.dest();
                let dest = Code::dest(parser_dest.as_deref());

                let parser_comp = parser.comp();
                let comp = Code::comp(parser_comp.as_str());

                let parser_jump = parser.jump();
                let jump = Code::jump(parser_jump.as_deref());

                println!("111{}{}{}", comp, dest, jump);
            }
            None => eprint!("No specified instruction type"),
        }
    }
}

fn main() {
    let mut symbol_table = SymbolTable::new();
    symbol_table.initialize_labels();

    first_pass(&mut symbol_table);
    second_pass(&mut symbol_table);
}
