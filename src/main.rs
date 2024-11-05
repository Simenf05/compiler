
#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;
use std::env;
use std::fs;


#[derive(Debug)]
struct Operation {

    opr: Vec<String>

}

fn build_function() -> Function {
    
    let opr: Vec<Operation> = vec!();
    Function {
        operations: opr
    }
}

#[derive(Debug)]
struct Function {
    operations: Vec<Operation>
}

#[derive(Debug)]
struct AST {

    functions: HashMap<String, Function>,

}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: Option<String>,
}


#[derive(Debug)]
enum TokenType {
    Return,
    Semicolon,
    Number,
    String
}

fn evaluate_buffer(buffer: &str) -> Option<Token> {

    // Numeric literal
    let is_numeric = buffer.parse::<u8>();

    match is_numeric {
        Ok(value) => return Some(Token {
            token_type: TokenType::Number,
            value: Some(buffer.to_string()),
        }),
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
            })
        }
    }


    // Other types

    match buffer {
        ";" => return Some(Token {
            token_type: TokenType::Semicolon,
            value: None,
        }),
        "return" => return Some(Token {
            token_type: TokenType::Return,
            value: None,
        }),
        _ => None
    }
}

fn tokenaize(file: String) {

    let mut tokens = Vec::new();

    let mut buffer  = String::new();

    for char in file.chars() {
        buffer += char.to_string().as_str();
        if buffer.chars().last() == Some(' ') || buffer.chars().last() == Some(';')  {

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

    println!("{tokens:?}")

}

fn build_ast(filevec: Vec<&str>) -> AST {



    let vecstr = filevec.iter().map(|x| String::from(*x));

    let functions: HashMap<String, Function> = HashMap::new();

    let mut operations: Vec<Operation> = vec!();
    let mut opr: Vec<String> = vec!();

    // let mut block_state = BlockType::Global;

    for item in vecstr {
        let last_char_opt = item.chars().last();

        let last_char = match last_char_opt {
            Some(value) => value,
            _ => continue,
        };


        if item == "func" {
            // block_state = BlockType::Func;
            continue;
        }

        if item == "}" {
            
        }


        opr.push(item);


        if last_char == ';' {
            operations.push(Operation {
                opr: opr.clone(),
            });
            opr.clear();
        }

    }

    AST {
        functions,
    }
}


fn main() {

    
    let args: Vec<String> = env::args().collect();    

    
    let file = fs::read_to_string(&args[1])
    .expect("this file did not work");


    let token_vec = tokenaize(file);

    // let tree = build_ast(filevec);

    // println!("{tree:?}")

}


