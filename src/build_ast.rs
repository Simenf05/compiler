use std::{env::consts, os::linux::raw::stat, vec};

use crate::tokenaize::{Token, TokenType};

#[derive(Debug)]
struct Constant {}

#[derive(Debug)]
struct Statement {
    ast: Option<AST>,
    code: Option<Vec<Token>>,
}

#[derive(Debug)]
pub struct AST {
    condition: Option<Vec<Token>>,
    statments: Vec<Statement>,
}

pub fn tokens_to_ast(tokens: Vec<Token>, condition: Option<Vec<Token>>) -> AST {
    let mut statments = vec![];

    let mut block_level = 0u8;
    let mut statement: Vec<Token> = vec![];

    for token in tokens {
        if token.token_type == TokenType::OpenBlock {
            block_level += 1;
        } else if token.token_type == TokenType::CloseBlock {
            block_level -= 1;

            if block_level == 0 {
                let mut condition_tokens = vec![];
                let mut block_statments = vec![];

                let mut open_block_found = false;

                for token in statement.clone() {
                    if token.token_type == TokenType::OpenBlock && !open_block_found {
                        open_block_found = true;
                        continue;
                    }
                    if open_block_found {
                        block_statments.push(token);
                    } else {
                        condition_tokens.push(token);
                    }
                }


                let block_ast = tokens_to_ast(block_statments, Some(condition_tokens));
                // println!("{:#?}", condition_tokens.clone());
                // println!("{:#?}", block_statments.clone());

                statments.push(Statement {
                    ast: Some(block_ast),
                    code: None,
                });

                statement.clear();
                continue;
            }
        }

        if block_level == 0 && token.token_type == TokenType::Semicolon {
            statments.push(Statement {
                ast: None,
                code: Some(statement.clone()),
            });
            statement.clear();
            continue;
        }

        statement.push(token);
    }

    AST {
        condition,
        statments,
    }
}
