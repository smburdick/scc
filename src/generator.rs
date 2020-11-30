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
	let mut to_return = generate_term(&*ast.initial_term);
	ast.extra_terms.iter().for_each(|term| {
		to_return = format!("{}\tpush %eax\n{}\tpop %ecx\n", to_return, generate_term(&*term.1));
		match term.0 {
			Additive::Plus => {
				to_return = format!("{}\taddl %ecx, %eax\n", to_return);
			},
			Additive::Minus => {
				to_return = format!("{}\tsubl %eax, %ecx\n\tmovl %ecx, %eax\n", to_return);
			}
		}
	});
	to_return
}

fn generate_term(ast: &TermASTNode) -> String {
	let mut to_return = generate_factor(&*ast.initial_factor);
	ast.extra_factors.iter().for_each(|factor| {
		to_return = format!("{}\tpush %eax\n{}\tpop %ecx\n", to_return, generate_factor(&*factor.1));
		match factor.0 {
			Multiplicative::Times => {
				to_return = format!("{}\timul %ecx, %eax\n", to_return);
			},
			Multiplicative::Div => {
				to_return = format!("{}\tmovl %eax, %ebx\n\tmovl %ecx, %eax\n\tcdq\n\tidivl %ebx\n", to_return);
			}
		}	
	});
	to_return
}

fn generate_factor(ast: &FactorASTNode) -> String {
	match ast {
		FactorASTNode::Int(c) => format!("\tmovl ${}, %eax\n", c), // insert constant into operation register
		FactorASTNode::SingleOp(unary_op, child_node) => {
			match unary_op {
				UnaryOp::Comp => format!("{}\tnot %eax\n", generate_factor(&*child_node)),
				UnaryOp::Neg => format!("{}\tneg %eax\n", generate_factor(&*child_node)),
				UnaryOp::Not => format!("{}\tcmpl $0, %eax\n\tmovl $0, %eax\n\tsete %al\n", generate_factor(&*child_node)),
			}
		},
		FactorASTNode::WrappedExp(exp) => {
			format!("{}", generate_expression(&**exp)) // FIXME
		}
	}
}