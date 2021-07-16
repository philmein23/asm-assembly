mod parser;
use parser::{InstructionType, Parser};

mod code;
use code::Code;

mod symbol_table;
use symbol_table::SymbolTable;

fn main() {
    let mut parser = Parser::new("Add.asm");

    while parser.has_more_instructions() {
        parser.advance();

        match parser.instruction_type() {
            Some(InstructionType::A) | Some(InstructionType::L) => {
                let symbol = parser.symbol();
                let int = symbol.parse::<u32>().unwrap();
                let binary = format!("{:015b}", int);
                println!("0{}", binary);
            }
            Some(InstructionType::C) => {
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
