use crate::ast::*;

pub fn parse_program(tokens: &mut Vec<String>) -> ProgramASTNode {
	let mut tokens = tokens;
	super::ast::ProgramASTNode::new(parse_fn_decl(&mut tokens))
}

pub fn parse_fn_decl(tokens: &mut Vec<String>) -> FnDeclASTNode {
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
	FnDeclASTNode::new(string, statement)
}

pub fn parse_statement(tokens: &mut Vec<String>) -> StatementASTNode {
	let mut token = tokens.drain(0..1).collect::<String>();
	if token != "return" {
		panic!("Failed to parse return statement");
	}
	let mut tokens = tokens;
	let expr = parse_expression(tokens.drain(0..1).collect::<String>());
	token = tokens.drain(0..1).collect::<String>();
	if token != ";" {
		panic!("Failed to parse statement semicolon");
	}
	StatementASTNode::new(expr)
}

pub fn parse_expression(token: String) -> ExpressionASTNode {
	match token.parse::<i64>() {
		Ok(n) => return ExpressionASTNode::new_cst(n),
		Err(_) => {
			let op = get_operator(&token[0..1]);
			return ExpressionASTNode::new_op(op, parse_expression(token[1..].to_string()));
		},
	}
}

fn get_operator(op: &str) -> UnaryOp {
	match op {
		"!" => UnaryOp::Not,
		"-" => UnaryOp::Neg,
		"~" => UnaryOp::Comp,
		_ => panic!("Invalid operator"),
	}
}
