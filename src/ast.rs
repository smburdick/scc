
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
			UnaryOp::Not => "!".to_string(),
			UnaryOp::Comp => "~".to_string(),
			UnaryOp::Neg => "-".to_string(),
		}
	}
}

pub enum BinOp {
	Plus,
	Minus,
	Times,
	Div,
	Mod
}

pub enum Additive {
	Plus,
	Minus
}

pub enum Multiplicative {
	Times,
	Div
}

pub struct ExpressionASTNode {
	initial_term: Box<TermASTNode>,
	extra_terms: Vec<(Additive, Box<TermASTNode>)>
}

impl ExpressionASTNode {
	pub fn new(initial_term: Box<TermASTNode>, extra_terms: Vec<(Additive, Box<TermASTNode>)>) -> Self {
		ExpressionASTNode { initial_term: initial_term, extra_terms: extra_terms }
	}
}

impl ExpressionASTNode {
	pub fn new_cst(c: i64) -> Self {
		panic!("No such function");
		//ExpressionASTNode::Cst(c)
	}
	pub fn new_op(op: UnaryOp, exp: ExpressionASTNode) -> Self {
		//ExpressionASTNode::Op(op, Box::new(exp))
		panic!("No such function");
	}
}

pub struct TermASTNode {
	initial_factor: Box<FactorASTNode>,
	extra_factors: Vec<(Multiplicative, Box<FactorASTNode>)>
}

impl TermASTNode {
	pub fn new(initial_factor: Box<FactorASTNode>, extra_factors: Vec<(Multiplicative, Box<FactorASTNode>)>) -> Self {
		TermASTNode { initial_factor: initial_factor, extra_factors: extra_factors }
	}
}

pub enum FactorASTNode { // listed in order of precedence
	WrappedExp(Box<ExpressionASTNode>),
	SingleOp(UnaryOp, Box<FactorASTNode>),
	Int(i64)
}
