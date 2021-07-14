mod parser;
use parser::Parser;

fn main() {
    let mut parser = Parser::new("Add.asm");

    while parser.has_more_instructions() {
        parser.advance();
        println!(
            "Symbol  {:?} Dest {:?} Comp {:?} Jump {:?}",
            parser.symbol(),
            parser.dest(),
            parser.comp(),
            parser.jump()
        );
    }
}
