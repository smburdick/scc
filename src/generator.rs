use crate::ast::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

pub fn generate_program(ast: ProgramASTNode) -> String {
	generate_function(ast.fn_decl)
}

fn generate_function(ast: FnDeclASTNode) -> String {
	let s = ast.string;
	format!(" .globl {}\n{}:\n\tpush %ebp\n\tmovl %esp, %ebp\n{}", s, s, generate_return(ast.statements))
}

fn generate_return(stmts: Vec<StatementASTNode>) -> String {
	let mut stack_index = -4; // FIXME should we start from here?
	let mut var_map = HashMap::<String, i64>::new();
	let mut to_return = String::new();
	stmts.iter().for_each(|stmt| {
		match stmt {
			StatementASTNode::Return(expr) => {
				to_return = format!("{}\t{}", to_return, generate_logical_or_expression(&expr, &var_map));

			},
			StatementASTNode::Declare(var, Some(expr)) => {
				let a = var_map.get(var);
				match a {
					Some(_) => panic!(format!("Variable {} already in use", var)),
					None => {
						let expr_s = generate_logical_or_expression(&*expr, &var_map);
						var_map.insert(var.to_string(), stack_index);
						stack_index = stack_index - 4;
						to_return = format!("{}\t{}\tpushl %eax\n", to_return, expr_s);
					}
				}
			},
			StatementASTNode::Declare(var, None) => {
				let a = var_map.get(var);
				match a {
					Some(_) => panic!(format!("Variable {} already in use", var)),
					None => {
						var_map.insert(var.to_string(), stack_index);
						stack_index = stack_index - 4;
						to_return = format!("{}\tpushl $0\n", to_return);
					}
				}
			},			
			StatementASTNode::Expression(exp) => {
				to_return = format!("{}\n{}", to_return, generate_logical_or_expression(&exp, &var_map));
			}
		}
	});
	format!("{}\tmovl %ebp, %esp\n\tpop %ebp\n\tret\n", to_return)
}

fn generate_logical_or_expression(ast: &ExpressionASTNode, map: &HashMap<String, i64>) -> String {
	match ast {
		ExpressionASTNode::BinOp(op, e1, e2) => {
			match op {
				BinaryOperator::Or => {
					let label = generate_label();
					format!("{}\tcmpl $0, %eax\n\tje _clause_{}\n\tmovl $1, %eax\n\tjmp _end_{}\n_clause_{}:\n{}\tcmpl $0, %eax\n\tmovl $0, %eax\n\tsetne %al\n_end_{}:\n",
						generate_logical_and_expression(&*e1, map), label, label, label,
						generate_logical_and_expression(&*e2, map), label)
				
				},
				_ => {
					generate_logical_and_expression(ast, map)
				}
			}
		},
		ExpressionASTNode::Wrapped(exp) => {
			generate_logical_or_expression(exp, map)
		}
		_ => {
			generate_logical_and_expression(ast, map)
		}		
	}
}

fn generate_logical_and_expression(ast: &ExpressionASTNode, map: &HashMap<String, i64>) -> String {
	match ast {
		ExpressionASTNode::BinOp(op, e1, e2) => {
			match op {
				BinaryOperator::And => {
					let label = generate_label();
					format!("{}\tcmpl $0, %eax\n\tjne _clause_{}\n\tjmp _end_{}\n_clause_{}:\n{}\tcmpl $0, %eax\n\tmovl $0, %eax\n\tsetne %al\n_end_{}:\n",
						generate_equality_expression(&*e1, map), label, label, label,
						generate_equality_expression(&*e2, map), label)
				},
				_ => {
					generate_equality_expression(ast, map)
				}
			}
		},
		ExpressionASTNode::Wrapped(exp) => {
			generate_logical_or_expression(exp, map)
		}
		_ => {
			generate_equality_expression(ast, map)
		}		
	}
}

fn generate_label() -> u64 {
	let mut hasher = DefaultHasher::new();
	let now = SystemTime::now();
	now.hash(&mut hasher);
	hasher.finish()
}

fn generate_equality_expression(ast: &ExpressionASTNode, map: &HashMap<String, i64>) -> String {
	match ast {
		ExpressionASTNode::BinOp(op, e1, e2) => {
			let to_return = format!("{}\tpush %eax\n{}\tpop %ecx\n\tcmpl %eax, %ecx\n\tmovl $0, %eax\n",
				generate_relational_expression(&*e1, map), generate_relational_expression(&*e2, map));
			match op {
				BinaryOperator::Equal => {
					format!("{}\tsete %al\n", to_return)
				},
				BinaryOperator::Neq => {
					format!("{}\tsetne %al\n", to_return)
				},
				_ => {
					generate_relational_expression(ast, map)
				}
			}
		},
		ExpressionASTNode::Wrapped(exp) => {
			generate_logical_or_expression(exp, map)
		}
		_ => {
			generate_relational_expression(ast, map)
		}
	}
}

fn generate_relational_expression(ast: &ExpressionASTNode, map: &HashMap<String, i64>) -> String {
	match ast {
		ExpressionASTNode::BinOp(op, e1, e2) => {
			let to_return = format!("{}\tpush %eax\n{}\tpop %ecx\n\tcmpl %eax, %ecx\n\tmovl $0, %eax\n",
				generate_additive_expression(&*e1, map), generate_additive_expression(&*e2, map));
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
					generate_additive_expression(ast, map)
				}
			}
		},
		ExpressionASTNode::Wrapped(exp) => {
			generate_logical_or_expression(exp, map)
		}
		_ => {
			generate_additive_expression(ast, map)
		}
	}
}

fn generate_additive_expression(ast: &ExpressionASTNode, map: &HashMap<String, i64>) -> String {
	match ast {
		ExpressionASTNode::BinOp(op, e1, e2) => {
			let to_return = format!("{}\tpush %eax\n{}\tpop %ecx\n",
				generate_term(&*e1, map), generate_term(&*e2, map));
			match op {
				BinaryOperator::Plus => {
					format!("{}\taddl %ecx, %eax\n", to_return)
				},
				BinaryOperator::Minus => {
					format!("{}\tsubl %eax, %ecx\n\tmovl %ecx, %eax\n", to_return)
				},
				_ => {
					generate_term(ast, map)
				}
			}
		},
		ExpressionASTNode::Wrapped(exp) => {
			generate_logical_or_expression(exp, map)
		}
		_ => {
			generate_term(ast, map)
		}
	}
}

fn generate_term(ast: &ExpressionASTNode, map: &HashMap<String, i64>) -> String {
	match ast {
		ExpressionASTNode::BinOp(op, e1, e2) => {
			let to_return = format!("{}\tpush %eax\n{}\tpop %ecx\n", generate_factor(&*e1, map), generate_factor(&*e2, map));
			match op {
				BinaryOperator::Times => {
					format!("{}\timul %ecx, %eax\n", to_return)
				},
				BinaryOperator::Div => {
					format!("{}\tmovl %eax, %ebx\n\tmovl %ecx, %eax\n\tcdq\n\tidivl %ebx\n", to_return)
				},
				_ => {
					generate_factor(ast, map)
				}
			}
		},
		_ => {
			generate_factor(ast, map)
		}
	}
}

fn generate_factor(ast: &ExpressionASTNode, map: &HashMap<String, i64>) -> String {
	match ast {
		ExpressionASTNode::Cst(c) => {
			format!("\tmovl ${}, %eax\n", c)
		}, // insert constant into operation register
		ExpressionASTNode::UnOp(unary_op, child_node) => {
			match unary_op {
				UnaryOp::Comp => format!("{}\tnot %eax\n", generate_factor(&*child_node, map)),
				UnaryOp::Neg => format!("{}\tneg %eax\n", generate_factor(&*child_node, map)),
				UnaryOp::Not => format!("{}\tcmpl $0, %eax\n\tmovl $0, %eax\n\tsete %al\n",
					generate_factor(&*child_node, map)),
			}
		},
		ExpressionASTNode::Assign(var, exp) => {
			let exp_s = generate_logical_or_expression(exp, map);
			let offset = map.get(var);
			match offset {
				Some(&_offset) => {
					format!("{}\tmovl %eax, {}(%ebp)\n", exp_s, _offset)
				},
				None => panic!("Invalid Assign")
			}
		},
		ExpressionASTNode::Var(var) => {
			let offset = map.get(var);
			match offset {
				Some(&_offset) => {
					format!("\tmovl {}(%ebp), %eax\n", _offset)
				},
				None => panic!("Invalid Var")
			}
		},
		_ => {
			format!("{}", generate_logical_or_expression(ast, map))
		}
	}
}