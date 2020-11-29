mod lexer;
mod parser;
mod ast;
mod generator;

use std::fs;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	println!("{:?}", args);
	if args.len() < 2 {
		panic!("Usage: cargo run FILENAME");
	}
	let file_name = &args[1];
	println!("file_name: {}", file_name);
	let tokens = lexer::lex(file_name);
	//let tokens = lexer::lex(&"c/return_.c");
	println!("Printing tokens");
	tokens.print_all();
	let mut token_vec = tokens.tokens().to_vec();
	let program: ast::ProgramASTNode = parser::parse_program(&mut token_vec);
	assert!(token_vec.len() == 0);
	let program = generator::generate_program(program);
	fs::write("out/a.s", program).expect("Unable to write program");
}