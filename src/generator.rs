pub fn generate_program(ast: super::ast::ProgramASTNode) -> String {
	generate_function(ast.fn_decl)
}

fn generate_function(ast: super::ast::FnDeclASTNode) -> String {
	let s = ast.string;
	format!(" .globl {}\n{}:\n{}", s, s, generate_return(ast.statement))
}

fn generate_return(ast: super::ast::StatementASTNode) -> String {
	let k = ast.ret.get_cst();
	format!("\tmovl ${}, %eax\n\tret\n", k)
}