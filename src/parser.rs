use crate::lexer::Lexer;

use super::Token;
use super::ast;




fn parse(lex: Lexer) -> ast::AST {
    // Find the package
    lex.eat(&Token::Package);
}

fn err_tok(val: bool) {
    if val {
        panic!("Unexpected Token");
    }
}