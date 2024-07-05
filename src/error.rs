use crate::{ast, lexer::Token};



pub fn token_type_err(got:&Token, expected: &Token) {
    panic!("Expected: {:?}\nGot: {:?}", expected, got);
}

pub fn ast_type_err(got: &str, expected: &str) {
    panic!("Expected: {:?}\nGot: {:?}", expected, got);
}