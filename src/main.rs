mod parser;
use parser::{InstructionType, Parser};

mod code;
use code::Code;

fn main() {
    let mut parser = Parser::new("Add.asm");

    while parser.has_more_instructions() {
        parser.advance();

        match parser.instruction_type() {
            Some(InstructionType::A) => {}
            Some(InstructionType::C) => {
                let parser_dest = parser.dest();
                let dest = Code::dest(parser_dest.as_str());

                let parser_comp = parser.comp();
                let comp = Code::comp(parser_comp.as_str());

                let parser_jump = parser.jump();
                let jump = Code::jump(parser_jump.as_str());

                println!("Binary format: 111{}{}{}", comp, dest, jump);
            }
            Some(InstructionType::L) => {}
            None => eprint!("No specified instruction type"),
        }
    }
}
