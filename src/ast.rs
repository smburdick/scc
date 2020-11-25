
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
	pub ret: ExpressionASTNode
}

impl StatementASTNode {
	pub fn new(ret: ExpressionASTNode) -> Self {
		StatementASTNode { ret: ret }
	}
}

pub enum UnaryOp {
	Not,
	Comp,
	Neg,
}

impl UnaryOp {
	fn to_string(&self) -> String {
		match *self {
			UnaryOp::Not => "!".to_string(),
			UnaryOp::Comp => "~".to_string(),
			UnaryOp::Neg => "-".to_string(),
		}
	}
}

pub enum ExpressionASTNode {
	Op(UnaryOp, Box<ExpressionASTNode>),
	Cst(i64)
}

impl ExpressionASTNode {
	pub fn new_cst(c: i64) -> Self {
		ExpressionASTNode::Cst(c)
	}
	pub fn new_op(op: UnaryOp, exp: ExpressionASTNode) -> Self {
		ExpressionASTNode::Op(op, Box::new(exp))
	}
	pub fn get_cst(&self) -> i64 {
		match &*self {
			ExpressionASTNode::Op(_, un_op) => {
				let mut op = un_op;
				loop {
					match &**op {
						ExpressionASTNode::Op(_, op2) => op = &op2,
						ExpressionASTNode::Cst(c) => return *c,
					}
				}

			}
			ExpressionASTNode::Cst(c) => *c,
		}
	}
}

