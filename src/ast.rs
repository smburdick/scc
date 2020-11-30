
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

pub enum Number {
	Integer,
	Float,
	Double
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

pub enum Additive {
	Plus,
	Minus
}

impl Additive {
	pub fn to_string(&self) -> String {
		match self {
			Additive::Plus => "PLUS".to_string(),
			Additive::Minus => "MINUS".to_string()
		}
	}
}

pub enum Multiplicative {
	Times,
	Div
}

impl Multiplicative {
	pub fn to_string(&self) -> String {
		match self {
			Multiplicative::Times => "TIMES".to_string(),
			Multiplicative::Div => "DIV".to_string()
		}
	}
}

pub struct ExpressionASTNode {
	pub initial_term: Box<TermASTNode>,
	pub extra_terms: Vec<(Additive, Box<TermASTNode>)>
}

impl ExpressionASTNode {
	pub fn new(initial_term: Box<TermASTNode>, extra_terms: Vec<(Additive, Box<TermASTNode>)>) -> Self {
		ExpressionASTNode { initial_term: initial_term, extra_terms: extra_terms }
	}
	pub fn pretty_print(&self) -> String {
		let mut to_return = (*self.initial_term).pretty_print();
		self.extra_terms.iter().for_each(|term|
			to_return = format!("{} {} {}", to_return, term.0.to_string(), (*term.1).pretty_print())
		);
		to_return
	}
}

pub struct TermASTNode {
	pub initial_factor: Box<FactorASTNode>,
	pub extra_factors: Vec<(Multiplicative, Box<FactorASTNode>)>
}

impl TermASTNode {
	pub fn new(initial_factor: Box<FactorASTNode>, extra_factors: Vec<(Multiplicative, Box<FactorASTNode>)>) -> Self {
		TermASTNode { initial_factor: initial_factor, extra_factors: extra_factors }
	}
	pub fn pretty_print(&self) -> String {
		let mut to_return = (*self.initial_factor).pretty_print();
		self.extra_factors.iter().for_each(|factor|
			to_return = format!("{} {} {}", to_return, factor.0.to_string(), (*factor.1).pretty_print())
		);
		to_return
	}
}

pub enum FactorASTNode { // listed in order of precedence
	WrappedExp(Box<ExpressionASTNode>),
	SingleOp(UnaryOp, Box<FactorASTNode>),
	Int(i64)
}

impl FactorASTNode {
	pub fn pretty_print(&self) -> String {
		match self {
			FactorASTNode::WrappedExp(exp) => {
				format!("({})", (*exp).pretty_print())
			},
			FactorASTNode::SingleOp(op, exp) => {
				format!("{} {}", op.to_string(), (*exp).pretty_print())
			},
			FactorASTNode::Int(i) => {
				format!("Int<{}>", i)
			}
		}
	}
}