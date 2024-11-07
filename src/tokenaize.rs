
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Return,
    Semicolon,
    Number,
    String,
    Let,
    Declaration,
    Variable,
    Const,
    Print,
}

fn evaluate_buffer(buffer: &str) -> Option<Token> {
    let special = match buffer {
        "=" => Some(Token {
            token_type: TokenType::Declaration,
            value: None,
        }),
        ";" => Some(Token {
            token_type: TokenType::Semicolon,
            value: None,
        }),

        "return" => Some(Token {
            token_type: TokenType::Return,
            value: None,
        }),

        "let" => Some(Token {
            token_type: TokenType::Let,
            value: None,
        }),
        "const" => Some(Token {
            token_type: TokenType::Const,
            value: None,
        }),
        "print" => Some(Token {
            token_type: TokenType::Print,
            value: None,
        }),
        _ => None,
    };

    if !special.is_none() {
        return special;
    }

    // Numeric literal
    let is_numeric = buffer.parse::<u8>();
    match is_numeric {
        Ok(_) => {
            return Some(Token {
                token_type: TokenType::Number,
                value: Some(buffer.to_string()),
            })
        }
        _ => (),
    }

    // String literal
    let first_char = buffer.chars().nth(0);
    let last_char = buffer.chars().last();

    let buffer_len = buffer.len();

    if !first_char.is_none() && first_char == Some('"') {
        if !last_char.is_none() && last_char == Some('"') {
            return Some(Token {
                token_type: TokenType::String,
                value: Some(buffer[1..buffer_len - 1].to_string()),
            });
        }
    }

    let mut is_alphanum = true;

    for char in buffer.chars() {
        if !char.is_alphanumeric() {
            is_alphanum = false;
            break;
        }
    }

    if is_alphanum && buffer_len > 0 {
        return Some(Token {
            token_type: TokenType::Variable,
            value: Some(buffer.to_string()),
        });
    }

    None
}

pub fn tokenaize(file: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut in_lit = false;

    for char in file.chars() {
        buffer += char.to_string().as_str();

        if buffer.len() == 1 && buffer.chars().last() == Some('"') {
            in_lit = true;
        } else if in_lit && buffer.chars().last() == Some('"') {
            in_lit = false;
        }

        if (buffer.chars().last() == Some(' ')
            || buffer.chars().last() == Some('\n')
            || buffer.chars().last() == Some(';')) 
            && !in_lit
        {
            let buffer_len = buffer.len();
            let first_part = evaluate_buffer(&buffer[..buffer_len - 1]);
            let last_char = evaluate_buffer(&buffer[buffer_len - 1..]);

            match first_part {
                Some(token) => tokens.push(token),
                _ => (),
            }
            match last_char {
                Some(token) => tokens.push(token),
                _ => (),
            }
            buffer.clear();
        }
    }
    tokens
}
