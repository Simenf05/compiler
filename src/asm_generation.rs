use crate::tokenaize::{ Token, TokenType };

pub struct AsmWithPlacement {
    pub placement: AsmPlacement,
    pub asm: String,
}

pub enum AsmPlacement {
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

    println!("{statement:?}");

    AsmWithPlacement {
        placement: AsmPlacement::Bss,
        asm,
    }
}

pub fn statement_to_asm(statement: &Vec<Token>) -> Option<AsmWithPlacement> {
    match statement[0].token_type {
        TokenType::Return => Some(return_asm(statement)),
        TokenType::Let => Some(let_declaration(statement)),
        TokenType::Const => Some(const_declaration(statement)),
        _ => None,
    }
}