mod lexer;
mod parser;
mod ast;
mod generator;

use std::fs;

fn main() {
	let tokens = lexer::lex(&"c/return_2.c");
	println!("Printing tokens");
	tokens.print_all();
	let mut token_vec = tokens.tokens().to_vec();
	let program: ast::ProgramASTNode = parser::parse_program(&mut token_vec);
	assert!(token_vec.len() == 0);
	let program = generator::generate_program(program);
	fs::write("out/a.s", program).expect("Unable to write program");
}