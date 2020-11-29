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
	let expr = parse_expression(tokens);
	token = tokens.drain(0..1).collect::<String>();
	if token != ";" {
		panic!("Failed to parse statement semicolon");
	}
	StatementASTNode::new(expr)
}

pub fn parse_expression(tokens: &mut Vec<String>) -> ExpressionASTNode {
	let mut tokens = tokens;
	let term = parse_term(tokens);
	let mut next = &tokens[0];
	let mut terms = Vec::new();
	while (next == "+" || next == "-") {
		let op = convert_to_additive(&next);
		let next_term = parse_term(tokens);
		terms.push((op, Box::new(next_term)));
		next = &tokens[0];
	}
	ExpressionASTNode::new(Box::new(term), terms)
}

fn parse_term(tokens: &mut Vec<String>) -> TermASTNode {
	let mut tokens = tokens;
	let factor = parse_factor(tokens);
	let mut next = &tokens[0];
	let mut factors = Vec::new();
	while (next == "*" || next == "/") {
		let op = convert_to_multiplicative(&next);
		let next_factor = parse_factor(tokens);
		factors.push((op, Box::new(next_factor)));
		next = &tokens[0];
	}
	TermASTNode::new(Box::new(factor), factors)
}

fn parse_factor(tokens: &mut Vec<String>) -> FactorASTNode {
	let mut next = tokens.drain(0..1).collect::<String>();
	let mut tokens = tokens;
	if next == "(" {
		let exp = parse_expression(tokens);
		if tokens.drain(0..1).collect::<String>() != ")" {
			panic!("Something went wrong with parsing");	
		}
		FactorASTNode::WrappedExp(Box::new(exp))
	} else if is_unop(&next) {
		let op = convert_to_op(&next);
		let factor = parse_factor(tokens);
		FactorASTNode::SingleOp(op, Box::new(factor))
	} else { // int
		FactorASTNode::Int(next.parse::<i64>().unwrap())
	}
}

fn is_unop(op: &str) -> bool {
	op == "~" || op == "!" || op == "-"
}

fn convert_to_op(op: &str) -> UnaryOp {
	match op {
		"!" => UnaryOp::Not,
		"-" => UnaryOp::Neg,
		"~" => UnaryOp::Comp,
		&_ => panic!("Invalid op")
	}
}

fn convert_to_additive(op: &str) -> Additive {
	match op {
		"+" => Additive::Plus,
		"-" => Additive::Minus,
		_ => panic!("Invalid op token")
	}
}

fn convert_to_multiplicative(op: &str) -> Multiplicative {
	match op {
		"*" => Multiplicative::Times,
		"/" => Multiplicative::Div,
		_ => panic!("Invalid op token")
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
