use std::default;

const LEFT_BRACE: &str = "LEFT_BRACE";
const RIGHT_BRACE: &str = "RIGHT_BRACE";
const LEFT_BRACKET: &str = "LEFT_BRACKET";
const RIGHT_BRACKET: &str = "RIGHT_BRACKET";
const COLON: &str = "COLON";
const COMMA: &str = "COMMA";
const STRING: &str = "STRING";
const NUMBER: &str = "NUMBER";
const TRUE: &str = "TRUE";
const FALSE: &str = "FALSE";
const NULL: &str = "NULL";

struct Token {
    value: String,
    token_type: String,
}

fn Lexer(input: String) -> Vec<Token> {
    let current_position = 0;
    let mut tokens: Vec<Token> = Vec::new();

    for i in input.chars() {
        let character: String = i.to_string();

        match i {
            '{' => tokens.push(Token {
                value: character,
                token_type: LEFT_BRACE.to_string(),
            }),
            '}' => tokens.push(Token {
                value: character,
                token_type: RIGHT_BRACE.to_string(),
            }),
            '[' => tokens.push(Token {
                value: character,
                token_type: LEFT_BRACKET.to_string(),
            }),
            ']' => tokens.push(Token {
                value: character,
                token_type: RIGHT_BRACKET.to_string(),
            }),
            ':' => tokens.push(Token {
                value: character,
                token_type: COLON.to_string(),
            }),
            ',' => tokens.push(Token {
                value: character,
                token_type: COMMA.to_string(),
            }),
            '"' => {},
			default =>{
				
			}
        }
    }

    return tokens;
}

fn main() {
    println!("A simple Json Parser in Rust");
}
