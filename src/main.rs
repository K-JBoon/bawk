#![feature(proc_macro_hygiene)]
extern crate plex;

pub mod parser;
pub mod ast;
pub mod lexer;
pub mod interpreter;
pub mod bawk_type;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 3 {
        panic!("Expected usage: /path/to/bawk /path/to/script /path/to/input");
    } else {
        let script_path = &args[1];
        let file_path = &args[2];

        let mut script = std::fs::read_to_string(script_path)?;
        script.push_str("!mark__bawk__eof!");

        let lexer = lexer::Lexer::new(&script);
        let program = parser::parse(lexer).unwrap();
        interpreter::interp(&program, file_path)
    }
}
