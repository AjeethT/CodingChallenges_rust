use std::default;
use regex::Regex;

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
    let mut current_position: usize = 0;
    let mut tokens: Vec<Token> = Vec::new();

    for i in input.chars() {
        let character: String = i.to_string();

        match i {
            '{' => {
                tokens.push(Token {
                    value: character,
                    token_type: LEFT_BRACE.to_string(),
                });
                current_position = current_position + 1;
            }
            '}' => {
                tokens.push(Token {
                    value: character,
                    token_type: RIGHT_BRACE.to_string(),
                });
                current_position = current_position + 1;
            }
            '[' => {
                tokens.push(Token {
                    value: character,
                    token_type: LEFT_BRACKET.to_string(),
                });
                current_position = current_position + 1;
            }
            ']' => {
                tokens.push(Token {
                    value: character,
                    token_type: RIGHT_BRACKET.to_string(),
                });
                current_position = current_position + 1;
            }
            ':' => {
                tokens.push(Token {
                    value: character,
                    token_type: COLON.to_string(),
                });
                current_position = current_position + 1;
            }
            ',' => {
                tokens.push(Token {
                    value: character,
                    token_type: COMMA.to_string(),
                });
                current_position = current_position + 1;
            }
            '"' => {
                let mut string_seq = String::new();
                current_position = current_position + 1;
                for c in input[current_position..].chars() {
                    if c == '"' {
                        break;
                    } else {
                        string_seq.push(c);
                        current_position = current_position + 1;
                    }
                }
                current_position=current_position+1;
                tokens.push(Token {
                    value: string_seq,
                    token_type: STRING.to_string(),
                });
            }
            default => {
                        let reg_num = Regex::new(r"[0-9]").unwrap();
                        let reg_true = Regex::new(r"true").unwrap();
                        let reg_false = Regex::new(r"false").unwrap();
                        let reg_null = Regex::new(r"null").unwrap();
                        let reg_space = Regex::new(r"\s").unwrap();

                        if reg_space.is_match(&character) {
                            current_position=current_position+1;
                        }
                        else if reg_num.is_match(&character) {
                            for n in input[current_position..].chars() {
                                let mut number_seq = String::new();
                                if!reg_num.is_match(&n.to_string()) {
                                    break;
                                } else {
                                    number_seq.push(n);
                                    current_position = current_position + 1;
                                }
                            }
                        }
                        else if reg_true.is_match(&input[current_position..current_position+4]) {
                            tokens.push(Token {
                                value: input[current_position..current_position+4].to_string(),
                                token_type: TRUE.to_string(),
                            });
                            current_position = current_position + 4;
                        }
                        else if reg_false.is_match(&input[current_position..current_position+5]) {
                            tokens.push(Token{
                                value: input[current_position..current_position+5].to_string(),
                                token_type: FALSE.to_string(),
                            });
                            current_position = current_position + 5;
                        }
                        else if reg_null.is_match(&input[current_position..current_position+4]) {
                            tokens.push(Token {
                                value: input[current_position..current_position+4].to_string(),
                                token_type: NULL.to_string(),
                            });
                            current_position = current_position + 4;
                        }
                        else {
                            panic!("Invalid character found: {}", character);
                        }
            }
        }
    }

    return tokens;
}

fn main() {
    println!("A simple Json Parser in Rust");
    let input_json = r#"{"name":"Ajeeth"}"#;
    let tokens = Lexer(input_json.to_string());
    for token in tokens {
        println!("Token: {}, Value: {}", token.token_type, token.value);
    }
}
