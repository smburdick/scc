
pub struct ProgramASTNode {
	pub fn_decl: FnDeclASTNode
}

impl ProgramASTNode {
	pub fn new(fn_decl : FnDeclASTNode) -> Self {
		ProgramASTNode { fn_decl: fn_decl }
	}
	pub fn pretty_print(&self) -> String {
		self.fn_decl.pretty_print()
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
	pub fn pretty_print(&self) -> String {
		format!("FUN INT {}:\n\tparams: ()\n\tbody:\n\t\t{}", self.string, self.statement.pretty_print())
	}
}

pub struct StatementASTNode {
	pub ret: ExpressionASTNode
}

impl StatementASTNode {
	pub fn new(ret: ExpressionASTNode) -> Self {
		StatementASTNode { ret: ret }
	}
	pub fn pretty_print(&self) -> String {
		format!("RETURN {}", self.ret.pretty_print())
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
			UnaryOp::Not => "NOT".to_string(),
			UnaryOp::Comp => "COMP".to_string(),
			UnaryOp::Neg => "NEG".to_string(),
		}
	}
}

pub enum BinaryOperator {
	Plus,
	Minus,
	Times,
	Div,
	Gt,
	Lt,
	Geq,
	Leq,
	Equal,
	Neq,
	And,
	Or,
}

impl BinaryOperator {
	pub fn to_string(&self) -> String {
		match self {
			BinaryOperator::Plus => "PLUS".to_string(),
			BinaryOperator::Minus => "MINUS".to_string(),
			BinaryOperator::Times => "TIMES".to_string(),
			BinaryOperator::Div => "DIV".to_string(),
			BinaryOperator::Lt => "LT".to_string(),
			BinaryOperator::Gt => "GT".to_string(),
			BinaryOperator::Leq => "LEQ".to_string(),
			BinaryOperator::Geq => "GEQ".to_string(),
			BinaryOperator::Neq => "NEQ".to_string(),
			BinaryOperator::Equal => "EQ".to_string(),
			BinaryOperator::And => "AND".to_string(),
			BinaryOperator::Or => "OR".to_string()
		}
	}
}

pub enum ExpressionASTNode {
	BinOp(BinaryOperator, Box<ExpressionASTNode>, Box<ExpressionASTNode>),
	UnOp(UnaryOp, Box<ExpressionASTNode>),
	Cst(i64),
	Wrapped(Box<ExpressionASTNode>)
}

impl ExpressionASTNode {
	pub fn pretty_print(&self) -> String {
		match self {
			ExpressionASTNode::BinOp(op, e1, e2) => {
				format!("{} {} {}", (*e1).pretty_print(), op.to_string(), (*e2).pretty_print())
			},
			ExpressionASTNode::UnOp(op, e) => {
				format!("{} {}", op.to_string(), (*e).pretty_print())
			},
			ExpressionASTNode::Cst(c) => {
				format!("Int<{}>", c)
			},
			ExpressionASTNode::Wrapped(exp) => {
				format!("({})", (*exp).pretty_print())
			}
		}
	}
}
