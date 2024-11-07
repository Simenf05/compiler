use std::env;
use std::fs;
use std::fs::create_dir_all;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

mod asm_generation;
mod tokenaize;

use crate::asm_generation::{ AsmPlacement, statement_to_asm };
use crate::tokenaize::{ Token, TokenType };

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

fn write_file(path: &str, asm: String) -> io::Result<()> {

    let path = Path::new(path);

    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = File::create(&path)?;
    file.write_all(asm.as_bytes())?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = fs::read_to_string(&args[1]).expect("this file did not work");

    let tokens = tokenaize::tokenaize(file);

    let asm = token_to_asm(tokens);

    let out = format!("./asm/{}", &args[1].split(".").collect::<Vec<&str>>()[0]);
    let out_asm = out.clone() + ".asm";
    let out_o = out.clone() + ".o";

    match write_file(out_asm.as_str(), asm) {
        Err(err) => panic!("writing to file failed {}", err),
        Ok(_) => (),
    }

    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!("nasm -felf64 {}", out_asm).as_str())
        .output();

    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!("ld -o {} {}", out, out_o).as_str())
        .output();
}
