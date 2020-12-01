use crate::ast::*;

pub fn parse_program(tokens: &mut Vec<String>) -> ProgramASTNode {
	let mut tokens = tokens;
	ProgramASTNode::new(parse_fn_decl(&mut tokens))
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
	let statements = parse_statements(&mut tokens);
	if tokens.drain(0..1).collect::<String>() != "}" {
		panic!("Failed to parse function declaration");
	}
	FnDeclASTNode::new(string, statements)
}

pub fn parse_statements(tokens: &mut Vec<String>) -> Vec<StatementASTNode> {
	let to_return = Vec::<StatementASTNode>::new();
	while tokens.len() != 0 && tokens[0] != "}" {
		if tokens[0] == "return" {
			tokens.drain(0..1).collect::<String>();
			let exp = parse_expression(tokens);
			to_return.push(StatementASTNode::Return(exp));
		} else if tokens[0] == "int" {
			let var_name = tokens.drain(0..1).collect::<String>();
			match tokens.drain(0..1).collect::<String>().as_str() {
				";" => to_return.push(StatementASTNode::Declare(var_name, None)),
				"=" => {
					let exp = parse_expression(tokens);
					to_return.push(StatementASTNode::Declare(var_name, Some(exp)));
				},
				_ => {
					panic!("Invalid assignment");
				}
			}
		} else {
			to_return.push(StatementASTNode::Expression(parse_expression(tokens)));
			if tokens.drain(0..1).collect::<String>() != ";" {
				panic!("invalid assignment");
			}
		}
	}
	to_return
}

pub fn parse_expression(tokens: &mut Vec<String>) -> ExpressionASTNode {
	let mut token = tokens.drain(0..1).collect::<String>();
	if tokens[0] == "=" {
		tokens.drain(0..1).collect::<String>();
		return ExpressionASTNode::Assign(token, Box::new(parse_expression(&mut tokens)));
	}
	parse__or_expression(&mut tokens)
}

pub fn parse__or_expression(tokens: &mut Vec<String>) -> ExpressionASTNode {
	let mut tokens = tokens;
	let exp = parse_logical_and_expression(tokens);
	let next = &tokens[0];
	if next == "||" {
		let op = convert_bin_op(&next);
		tokens.drain(0..1);
		let next_term = parse_expression(tokens);
		return ExpressionASTNode::BinOp(op, Box::new(exp), Box::new(next_term));
	}
	exp
}

pub fn parse_logical_and_expression(tokens: &mut Vec<String>) -> ExpressionASTNode {
	let mut tokens = tokens;
	let exp = parse_equality_expression(tokens);
	let next = &tokens[0];
	if next == "&&" {
		let op = convert_bin_op(&next);
		tokens.drain(0..1);
		let next_term = parse_expression(tokens);
		return ExpressionASTNode::BinOp(op, Box::new(exp), Box::new(next_term));
	}
	exp
}

pub fn parse_equality_expression(tokens: &mut Vec<String>) -> ExpressionASTNode {
	let mut tokens = tokens;
	let exp = parse_relational_expression(tokens);
	let next = &tokens[0];
	if next == "!=" || next == "==" {
		let op = convert_bin_op(&next);
		tokens.drain(0..1);
		let next_term = parse_expression(tokens);
		return ExpressionASTNode::BinOp(op, Box::new(exp), Box::new(next_term));
	}
	exp
}

pub fn parse_relational_expression(tokens: &mut Vec<String>) -> ExpressionASTNode {
	let mut tokens = tokens;
	let exp = parse_additive_expression(tokens);
	let next = &tokens[0];
	if next == "<" || next == ">" || next == "<=" || next == ">=" {
		let op = convert_bin_op(&next);
		tokens.drain(0..1);
		let next_term = parse_expression(tokens);
		return ExpressionASTNode::BinOp(op, Box::new(exp), Box::new(next_term));
	}
	exp
}

pub fn parse_additive_expression(tokens: &mut Vec<String>) -> ExpressionASTNode {
	let mut tokens = tokens;
	let term = parse_term(tokens);
	let next = &tokens[0];
	if next == "+" || next == "-" {
		let op = convert_bin_op(&next);
		tokens.drain(0..1);
		let next_term = parse_expression(tokens);
		return ExpressionASTNode::BinOp(op, Box::new(term), Box::new(next_term));
	}
	term
}

fn parse_term(tokens: &mut Vec<String>) -> ExpressionASTNode {
	let mut tokens = tokens;
	let mut factor = parse_factor(tokens);
	let next = &tokens[0];
	if next == "*" || next == "/" {
		let op = convert_bin_op(&next);
		tokens.drain(0..1);
		let next_term = parse_term(tokens);
		return ExpressionASTNode::BinOp(op, Box::new(factor), Box::new(next_term));
	}
	factor
}

fn parse_factor(tokens: &mut Vec<String>) -> ExpressionASTNode {
	let mut next = tokens.drain(0..1).collect::<String>();
	let mut tokens = tokens;
	if next == "(" {
		let exp = parse_expression(tokens);
		if tokens.drain(0..1).collect::<String>() != ")" {
			panic!("Something went wrong with parsing");	
		}
		ExpressionASTNode::Wrapped(Box::new(exp))
	} else if is_unop(&next) {
		let op = convert_to_op(&next);
		let exp = parse_expression(tokens);
		ExpressionASTNode::UnOp(op, Box::new(exp))
	} else if next == "int" {
		let var = tokens.drain(0..1).collect::<String>();
		ExpressionASTNode::Var(var)
	} else { // int
		ExpressionASTNode::Cst(next.parse::<i64>().unwrap())
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

fn convert_bin_op(op: &str) -> BinaryOperator {
	match op {
		"+" => BinaryOperator::Plus,
		"-" => BinaryOperator::Minus,
		"*" => BinaryOperator::Times,
		"/" => BinaryOperator::Div,
		">" => BinaryOperator::Gt,
		"<" => BinaryOperator::Lt,
		"==" => BinaryOperator::Equal,
		"!=" => BinaryOperator::Neq,
		"<=" => BinaryOperator::Leq,
		">=" => BinaryOperator::Geq,
		"&&" => BinaryOperator::And,
		"||" => BinaryOperator::Or,
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
