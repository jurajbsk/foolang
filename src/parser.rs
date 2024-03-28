use core::panic;
use std::collections::VecDeque;
use std::panic::AssertUnwindSafe;

use crate::lexer::TokenType;
use crate::lexer::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum ExprType {
	EMPTY,
	FILE,
	FUNCTION_DECL,
}

#[derive(Clone, Debug)]
pub struct Expr {
	sym: ExprType,
	val: Option<String>,
}

impl Expr {
	fn new(sym: ExprType) -> Expr {
		Expr {sym, val: None}
	}
}

#[derive(Clone, Debug)]
pub struct ASTree {
	val: Expr,
	left: Option<Box<ASTree>>,
	right: Option<Box<ASTree>>,
}

fn expect(token: Option<Token>, expected: TokenType) -> Option<String> {
	match token {
		Some(value) if value.sym == expected => {
            return value.val;
        }
        value => {
            panic!("Expected {:?}, but got {:?}", expected, value);
        }
	}
}

fn end(tree: Option<ASTree>) -> Option<Box<ASTree>> {
	return match tree {
		Some(tree) => Some(Box::new(tree)),
		None => None,
	}
}

fn parse_expr(tokens: &mut VecDeque<Token>, i:i32) -> Option<Box<ASTree>> {
	let mut tree: Option<ASTree> = None;

	if tokens.is_empty() {
		return None;
	}
	while let Some(token) = tokens.pop_front() {
		match token.sym {
			TokenType::FUNCTION => {
				let name = expect(tokens.pop_front(), TokenType::IDENTIFIER);
				_ = expect(tokens.pop_front(), TokenType::OPEN_ROUND);
				_ = expect(tokens.pop_front(), TokenType::CLOSE_ROUND);
				_ = expect(tokens.pop_front(), TokenType::OPEN_SQUIGGLY);

				let functree = ASTree {
					val: Expr {
						sym: ExprType::FUNCTION_DECL,
						val: name
					},
					left: parse_expr(tokens, i+1),
					right: None
				};
				tree = Some(functree.clone());
				if functree.left.is_none() {
					return end(tree);
				}
			},

			TokenType::CLOSE_SQUIGGLY => {
				return end(tree);
			},

			_ => panic!("Unimplemented Token"),
		}
	}
	if let Some(ref mut sometree) = tree {
		sometree.right = otherbranch(tokens, i+1);
	}

	return match tree {
		Some(tree) => Some(Box::new(tree)),
		None => None,
	};
	//panic!("How did we get here?");
}

fn otherbranch(tokens: &mut VecDeque<Token>, i:i32) -> Option<Box<ASTree>> {
	let parse = parse_expr(tokens, i+1);
	return match parse {
		Some(branch) => Some(Box::new(ASTree {
			val: Expr::new(ExprType::EMPTY),
			left: Some(branch),
			right: otherbranch(tokens, i+1),
		})),
		None => None
	}
}

pub fn parse_tokens(tokens: VecDeque<Token>) -> ASTree {
	let mut tokenlist = tokens.clone();
	let tree = ASTree {
		val: Expr {sym: ExprType::FILE, val: None},
		left: parse_expr(&mut tokenlist, 1),
		right: otherbranch(&mut tokenlist, 1)
	};

	return tree;
}