pub fn parse_program(tokens: &mut Vec<String>) -> super::ast::ProgramASTNode {
	let mut tokens = tokens;
	super::ast::ProgramASTNode::new(parse_fn_decl(&mut tokens))
}

pub fn parse_fn_decl(tokens: &mut Vec<String>) -> super::ast::FnDeclASTNode {
	let mut token = tokens.drain(0..1).collect::<String>();
	if token != "int" {
		panic!("Failed to parse function declaration");
	}
	token = tokens.drain(0..1).collect::<String>();
	let string = token.clone();
	// skip past the ()
	if tokens.drain(0..1).collect::<String>() != "(" || tokens.drain(0..1).collect::<String>() != ")" || tokens.drain(0..1).collect::<String>() != "{" {
		panic!("Failed to parse function declaration");
	}
	let mut tokens = tokens;
	let statement = parse_statement(&mut tokens);
	if tokens.drain(0..1).collect::<String>() != "}" {
		panic!("Failed to parse function declaration");
	}
	super::ast::FnDeclASTNode::new(string, statement)
}

pub fn parse_statement(tokens: &mut Vec<String>) -> super::ast::StatementASTNode {
	let mut token = tokens.drain(0..1).collect::<String>();
	if token != "return" {
		panic!("Failed to parse return statement");
	}
	token = tokens.drain(0..1).collect::<String>();
	let mut i : i64;
	match token.parse::<i64>() {
		Ok(n) => i = n,
		Err(e) => panic!("Failed to parse return value"),
	}
	token = tokens.drain(0..1).collect::<String>();
	if token != ";" {
		panic!("Failed to parse statement semicolon");
	}
	let c = super::ast::ConstASTNode::new(i);
	super::ast::StatementASTNode::new(c)
}
