#![allow(dead_code)]
#![allow(unused_variables)]
// use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
struct AST {}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: Option<String>,
}

#[derive(PartialEq, Debug)]
enum TokenType {
    Return,
    Semicolon,
    Number,
    String,
    Let,
    Declaration,
    Variable,
    Const,
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

fn tokenaize(file: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();

    for char in file.chars() {
        buffer += char.to_string().as_str();
        if buffer.chars().last() == Some(' ')
            || buffer.chars().last() == Some('\n')
            || buffer.chars().last() == Some(';')
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

// fn build_ast(filevec: Vec<&str>) -> AST {

//     let vecstr = filevec.iter().map(|x| String::from(*x));

//     let functions: HashMap<String, Function> = HashMap::new();

//     let mut operations: Vec<Operation> = vec!();
//     let mut opr: Vec<String> = vec!();

//     // let mut block_state = BlockType::Global;

//     for item in vecstr {
//         let last_char_opt = item.chars().last();

//         let last_char = match last_char_opt {
//             Some(value) => value,
//             _ => continue,
//         };

//         if item == "func" {
//             // block_state = BlockType::Func;
//             continue;
//         }

//         if item == "}" {

//         }

//         opr.push(item);

//         if last_char == ';' {
//             operations.push(Operation {
//                 opr: opr.clone(),
//             });
//             opr.clear();
//         }

//     }

//     AST {
//         functions,
//     }
// }

fn parse(tokens: Vec<Token>) -> AST {
    AST {}
}

#[derive(Debug)]
struct AsmWithPlacement {
    placement: AsmPlacement,
    asm: String,
}

#[derive(Debug)]
enum AsmPlacement {
    Data,
    Bss,
    Text,
}

fn return_asm(statement: &Vec<Token>) -> AsmWithPlacement {
    let mut asm = String::from("    mov rax, 60\n");

    let string_value = statement[1].value.as_ref().unwrap();

    let value = match statement[1].token_type {
        TokenType::Number => format!("    mov rdi, {}\n", string_value.parse::<u8>().unwrap()),
        TokenType::Variable => format!("    movzx rdi, byte [{}]\n", string_value.to_string()),
        _ => panic!("noe er galt"),
    };

    asm += value.as_str();
    asm += "    syscall\n";
    AsmWithPlacement {
        placement: AsmPlacement::Text,
        asm,
    }
}

fn const_declaration(statement: &Vec<Token>) -> AsmWithPlacement {
    let mut asm = String::from("");

    asm += statement[1].value.as_ref().unwrap().as_str();
    asm += " db ";
    asm += statement[3].value.as_ref().unwrap().as_str();
    asm += "\n";

    AsmWithPlacement {
        placement: AsmPlacement::Data,
        asm,
    }
}

fn let_declaration(statement: &Vec<Token>) -> AsmWithPlacement {
    let asm = String::from("");

    AsmWithPlacement {
        placement: AsmPlacement::Bss,
        asm,
    }
}

fn statement_to_asm(statement: &Vec<Token>) -> Option<AsmWithPlacement> {
    match statement[0].token_type {
        TokenType::Return => Some(return_asm(statement)),
        TokenType::Let => Some(let_declaration(statement)),
        TokenType::Const => Some(const_declaration(statement)),
        _ => None,
    }
}

fn token_to_asm(tokens: Vec<Token>) -> String {
    let mut result = String::new();

    let mut text = String::new();
    text += "section .text\n";
    text += "    global _start\n";
    text += "_start:\n";

    let mut bss = String::new();
    bss += "\nsection .bdd\n";

    let mut data = String::new();
    data += "\nsection .data\n";

    let mut statement: Vec<Token> = vec![];
    for token in tokens {
        if token.token_type == TokenType::Semicolon {
            let statement_asm_opt = statement_to_asm(&statement);

            if !statement_asm_opt.is_none() {
                let statement_asm = statement_asm_opt.unwrap();
                match statement_asm.placement {
                    AsmPlacement::Bss => bss += statement_asm.asm.as_str(),
                    AsmPlacement::Data => data += statement_asm.asm.as_str(),
                    AsmPlacement::Text => text += statement_asm.asm.as_str(),
                };
            }
            statement.clear();
            continue;
        }
        statement.push(token);
    }

    result += &text;
    result += &data;
    result += &bss;

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = fs::read_to_string(&args[1]).expect("this file did not work");

    let tokens = tokenaize(file);

    let asm = token_to_asm(tokens);

    let path = Path::new("./asm/out.asm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(asm.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    let _ = Command::new("sh")
        .arg("-c")
        .arg("nasm -felf64 ./asm/out.asm")
        .output();

    let _ = Command::new("sh")
        .arg("-c")
        .arg("ld -o ./asm/out ./asm/out.o")
        .output();
}
