use crate::ast::*;

pub fn generate_program(ast: ProgramASTNode) -> String {
	generate_function(ast.fn_decl)
}

fn generate_function(ast: FnDeclASTNode) -> String {
	let s = ast.string;
	format!(" .globl {}\n{}:\n{}", s, s, generate_return(ast.statement))
}

fn generate_return(ast: StatementASTNode) -> String {
	format!("{}\tret\n", generate_expression(&ast.ret))
}

fn generate_expression(ast: &ExpressionASTNode) -> String {
	match ast {
		ExpressionASTNode::BinOp(op, e1, e2) => {
			let to_return = format!("{}\tpush %eax\n{}\tpop %ecx\n", generate_logical_and_expression(&*e1), generate_logical_and_expression(&*e2));
			match op {
				BinaryOperator::Or => {
					// TODO
					panic!("or not yet supported")
				},
				_ => {
					generate_logical_and_expression(ast)
				}
			}
		},
		ExpressionASTNode::Wrapped(exp) => {
			generate_expression(exp)
		}
		_ => {
			generate_logical_and_expression(ast)
		}		
	}
}

fn generate_logical_and_expression(ast: &ExpressionASTNode) -> String {
	match ast {
		ExpressionASTNode::BinOp(op, e1, e2) => {
			let to_return = format!("{}\tpush %eax\n{}\tpop %ecx\n", generate_equality_expression(&*e1), generate_equality_expression(&*e2));
			match op {
				BinaryOperator::And => {
					// TODO
					panic!("and not yet supported")
				},
				_ => {
					generate_equality_expression(ast)
				}
			}
		},
		ExpressionASTNode::Wrapped(exp) => {
			generate_expression(exp)
		}
		_ => {
			generate_equality_expression(ast)
		}		
	}
}

fn generate_equality_expression(ast: &ExpressionASTNode) -> String {
	match ast {
		ExpressionASTNode::BinOp(op, e1, e2) => {
			let to_return = format!("{}\tpush %eax\n{}\tpop %ecx\n\tcmpl %eax, %ecx\n\tmovl $0, %eax\n", generate_relational_expression(&*e1), generate_relational_expression(&*e2));
			match op {
				BinaryOperator::Equal => {
					format!("{}\tsete %al\n", to_return)
				},
				BinaryOperator::Neq => {
					format!("{}\tsetne %al\n", to_return)
				},
				_ => {
					generate_relational_expression(ast)
				}
			}
		},
		ExpressionASTNode::Wrapped(exp) => {
			generate_expression(exp)
		}
		_ => {
			generate_relational_expression(ast)
		}
	}
}

fn generate_relational_expression(ast: &ExpressionASTNode) -> String {
	match ast {
		ExpressionASTNode::BinOp(op, e1, e2) => {
			let to_return = format!("{}\tpush %eax\n{}\tpop %ecx\n\tcmpl %eax, %ecx\n\tmovl $0, %eax\n", generate_additive_expression(&*e1), generate_additive_expression(&*e2));
			match op {
				BinaryOperator::Lt => {
					format!("{}\tsetl %al\n", to_return)
				},
				BinaryOperator::Gt => {
					format!("{}\tsetg %al\n", to_return)
				},
				BinaryOperator::Leq => {
					format!("{}\tsetle %al\n", to_return)
				},
				BinaryOperator::Geq => {
					format!("{}\tsetge %al\n", to_return)
				},
				_ => {
					generate_additive_expression(ast)
				}
			}
		},
		ExpressionASTNode::Wrapped(exp) => {
			generate_expression(exp)
		}
		_ => {
			generate_additive_expression(ast)
		}
	}
}

fn generate_additive_expression(ast: &ExpressionASTNode) -> String {
	match ast {
		ExpressionASTNode::BinOp(op, e1, e2) => {
			let to_return = format!("{}\tpush %eax\n{}\tpop %ecx\n", generate_term(&*e1), generate_term(&*e2));
			match op {
				BinaryOperator::Plus => {
					format!("{}\taddl %ecx, %eax\n", to_return)
				},
				BinaryOperator::Minus => {
					format!("{}\tsubl %eax, %ecx\n\tmovl %ecx, %eax\n", to_return)
				},
				_ => {
					generate_term(ast)
				}
			}
		},
		ExpressionASTNode::Wrapped(exp) => {
			generate_expression(exp)
		}
		_ => {
			generate_term(ast)
		}
	}
}

fn generate_term(ast: &ExpressionASTNode) -> String {
	match ast {
		ExpressionASTNode::BinOp(op, e1, e2) => {
			let to_return = format!("{}\tpush %eax\n{}\tpop %ecx\n", generate_factor(&*e1), generate_factor(&*e2));
			match op {
				BinaryOperator::Times => {
					format!("{}\timul %ecx, %eax\n", to_return)
				},
				BinaryOperator::Div => {
					format!("{}\tmovl %eax, %ebx\n\tmovl %ecx, %eax\n\tcdq\n\tidivl %ebx\n", to_return)
				},
				_ => {
					generate_factor(ast)
				}
			}
		},
		_ => {
			generate_factor(ast)
		}
	}
}

fn generate_factor(ast: &ExpressionASTNode) -> String {
	match ast {
		ExpressionASTNode::Cst(c) => {
			format!("\tmovl ${}, %eax\n", c)
		}, // insert constant into operation register
		ExpressionASTNode::UnOp(unary_op, child_node) => {
			match unary_op {
				UnaryOp::Comp => format!("{}\tnot %eax\n", generate_factor(&*child_node)),
				UnaryOp::Neg => format!("{}\tneg %eax\n", generate_factor(&*child_node)),
				UnaryOp::Not => format!("{}\tcmpl $0, %eax\n\tmovl $0, %eax\n\tsete %al\n", generate_factor(&*child_node)),
			}
		},
		_ => {
			format!("{}", generate_expression(ast))
		}
	}
}