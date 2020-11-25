
pub struct ProgramASTNode {
	pub fn_decl: FnDeclASTNode
}

impl ProgramASTNode {
	pub fn new(fn_decl : FnDeclASTNode) -> Self {
		ProgramASTNode { fn_decl: fn_decl }
	}
}

pub struct FnDeclASTNode {
	pub string: String,
	pub statement: StatementASTNode
}

impl FnDeclASTNode {
	pub fn new(string : String, statement: StatementASTNode) -> Self {
		FnDeclASTNode { string : string, statement : statement }
	}
}

pub struct StatementASTNode {
	pub ret: ConstASTNode
}

impl StatementASTNode {
	pub fn new(ret: ConstASTNode) -> Self {
		StatementASTNode { ret: ret }
	}
}

pub struct ConstASTNode {
	pub c: i64
}

impl ConstASTNode {
	pub fn new(c: i64) -> Self {
		ConstASTNode { c: c }
	}
}

