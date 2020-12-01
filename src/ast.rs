
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
	pub statements: Vec<StatementASTNode>
}

impl FnDeclASTNode {
	pub fn new(string : String, statements: Vec<StatementASTNode>) -> Self {
		FnDeclASTNode { string : string, statements : statements }
	}
	pub fn pretty_print(&self) -> String {
		let mut statement_lines = String::new();
		self.statements.iter().for_each(|stmt| {
			statement_lines = format!("{}\n\t\t{}", statement_lines, stmt.pretty_print());
		});
		format!("FUN INT {}:\n\tparams: ()\n\tbody:{}\n", self.string, statement_lines)
	}
}

pub enum StatementASTNode {
	Return(ExpressionASTNode),
	Declare(String, Option<ExpressionASTNode>), // string is variable name
	Expression(ExpressionASTNode)
}

impl StatementASTNode {
	pub fn pretty_print(&self) -> String {
		match self {
			StatementASTNode::Return(exp) => {
				format!("\tRETURN {}", exp.pretty_print())
			},
			StatementASTNode::Declare(var, Some(exp)) => {
				format!("\tINT {} = {}", var, exp.pretty_print())
			},
			StatementASTNode::Declare(var, None) => {
				format!("\tINT {}", var)
			},
			StatementASTNode::Expression(exp) => {
				format!("\t{}", exp.pretty_print())
			}
		}
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
	Assign(String, Box<ExpressionASTNode>),
	Var(String),
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
