use std::fs;
extern crate regex;

pub struct TokenCollection {
	tokens: Vec<String>
}

impl TokenCollection {
	pub fn new() -> Self {
		TokenCollection { tokens: Vec::new() }
	}
	pub fn insert(&mut self, s: String) {
		self.tokens.push(s);
	}
	pub fn print_all(&self) {
		self.tokens.iter().for_each(|s| println!("{}", s));
	}
}

fn main() {
	let tokens = lex(&"c/return_2.c");
	tokens.print_all();
}

// Acceots a file, returns a list of tokens
pub fn lex<'a>(file_name: &str) -> TokenCollection {
	let contents = fs::read_to_string(&file_name)
		.expect("Could not read file");
	let re = regex::Regex::new(r"(\{|\}|\(|\)|;|[a-zA-Z]\w*|[0-9]+)").unwrap();
	let mut token_collection = TokenCollection::new();
	//let mut values : Vec<String> = Vec::new();
	for part in re.find_iter(&contents) {
		let slice = &contents[part.start()..part.end()];
		token_collection.insert(slice.to_string());
	}
	token_collection
}