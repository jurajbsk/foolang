use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum TokenType {
    IDENTIFIER,

    OPEN_ROUND,
    CLOSE_ROUND,

    OPEN_SQUARE,
    CLOSE_SQUARE,

    OPEN_SQUIGGLY,
    CLOSE_SQUIGGLY,

    FUNCTION,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub sym: TokenType,
    pub val: Option<String>,
}

impl Token {
    fn new(sym: TokenType, val: Option<String>) -> Token {
        Token {sym, val}
    }
}

pub fn tokenize(code: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut token = String::new();

    let token_table: HashMap<String, TokenType> = HashMap::from([
        (String::from("("), TokenType::OPEN_ROUND),
        (String::from(")"), TokenType::CLOSE_ROUND),

        (String::from("["), TokenType::OPEN_SQUARE),
        (String::from("]"), TokenType::CLOSE_SQUARE),

        (String::from("{"), TokenType::OPEN_SQUIGGLY),
        (String::from("}"), TokenType::CLOSE_SQUIGGLY),

        (String::from("fn"), TokenType::FUNCTION),
    ]);

    for c in code.chars().collect::<Vec<char>>() {
        if c.is_whitespace() {continue}

        if token != String::new() && token_table.contains_key(&String::from(c)) {
            tokens.push(Token::new(TokenType::IDENTIFIER, Some(token.clone())));
            token.clear();
        }

        token.push(c);

        if token_table.contains_key(&token) {
            tokens.push(Token::new(token_table.get(&token).unwrap().clone(), None));
            token.clear();
        }		
    }

    return tokens;
}
