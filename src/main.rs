use std::env;
use std::fs;

mod lexer;
mod parser;

fn main() {
	let args: Vec<String> = env::args().collect();

	let mut in_path = String::new();
	let mut out_path = String::new();

	for i in 1..args.len() {
		if args[i] == "-i" {
			in_path = args[i + 1].clone();
		}
		else if args[i] == "-o" {
			out_path = args[i + 1].clone();
		}
	}

	let code = fs::read_to_string(&in_path).expect("Failed to read input");
	let tokens = lexer::tokenize(code);
	println!("{:#?}\n", tokens);
	let tree = parser::parse_tokens(tokens.clone().into());
	println!("{:#?}", tree);
}
