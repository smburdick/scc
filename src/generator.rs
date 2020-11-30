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
			let to_return = format!("{}\tpush %eax\n{}\tpop %ecx\n", generate_term(&*e1), generate_term(&*e2));
			match op {
				BinaryOperator::Plus => {
					println!("Add");
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