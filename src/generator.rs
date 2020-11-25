use crate::ast::*;

pub fn generate_program(ast: ProgramASTNode) -> String {
	generate_function(ast.fn_decl)
}

fn generate_function(ast: FnDeclASTNode) -> String {
	let s = ast.string;
	format!(" .globl {}\n{}:\n{}", s, s, generate_return(ast.statement))
}

fn generate_return(ast: StatementASTNode) -> String {
	format!("{}\tret\n", generate_expression(ast.ret))
}

fn generate_expression(ast: ExpressionASTNode) -> String {
	match ast {
		ExpressionASTNode::Cst(c) => format!("\tmovl ${}, %eax\n", c), // insert constant into operation register
		ExpressionASTNode::Op(unary_op, child_node) => {
			match unary_op {
				UnaryOp::Comp => format!("{}\tnot %eax\n", generate_expression(*child_node)),
				UnaryOp::Neg => format!("{}\tneg %eax\n", generate_expression(*child_node)),
				UnaryOp::Not => format!("{}\tcmpl $0, %eax\n\tmovl $0, %eax\n\tsete %al\n", generate_expression(*child_node)),
			}
		}
	}
}